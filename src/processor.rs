use solana_program;
use solana_program::account_info::AccountInfo;
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;
use spl_token;

use crate::config;
use crate::error::NftCanvasError;
use crate::instruction::NftCanvasInstruction;
use crate::packable::Packable;
use crate::state::{Pixel, PixelBuyInfo};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // assert_eq!(instruction_data.len(), NftCanvasInstruction::PACKED_SIZE);
    let instruction: NftCanvasInstruction = NftCanvasInstruction::unpack(instruction_data)?;
    match instruction {
        NftCanvasInstruction::MintPixel { index, color, sell_price } => {
            process_mint_pixel(program_id, accounts, index, color, sell_price)
        }
        NftCanvasInstruction::UpdatePixelColor { index, color } => {
            process_update_pixel_color(program_id, accounts, index, color)
        }
        NftCanvasInstruction::SellPixel { index, price } => {
            process_sell_pixel(program_id, accounts, index, price)
        }
        NftCanvasInstruction::BuyPixel { index, price, direct_only } => {
            process_buy_pixel(program_id, accounts, index, price, direct_only)
        }
    }
}

pub fn process_mint_pixel(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index: u32,
    color: [u8; 3],
    sell_price: u64,
) -> ProgramResult {
    check_pixel_index(index)?;

    let accounts_iter = &mut accounts.iter();

    let program = next_account_info(accounts_iter)?;
    let system_program_sysvar = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;
    let mint_pool_wallet = next_account_info(accounts_iter)?;
    let pixel_account = next_account_info(accounts_iter)?;
    let owner_wallet = next_account_info(accounts_iter)?;

    // println!("process_mint_pixel: program={}, (owner={})", program.key, program.owner);
    // println!("process_mint_pixel: system_program_sysvar={}, (owner={})", system_program_sysvar.key, system_program_sysvar.owner);
    // println!("process_mint_pixel: rent_sysvar={}, (owner={})", rent_sysvar.key, rent_sysvar.owner);
    // println!("process_mint_pixel: mint_pool_wallet={}, (owner={})", mint_pool_wallet.key, mint_pool_wallet.owner);
    // println!("process_mint_pixel: pixel_account={}, (owner={})", pixel_account.key, pixel_account.owner);
    // println!("process_mint_pixel: owner_wallet={}, (owner={})", owner_wallet.key, owner_wallet.owner);

    //

    let (pixel_account_id, pixel_account_bump_seed) = config::get_pixel(program_id, index);
    let rent_state = Rent::from_account_info(rent_sysvar)?;

    //

    if program.key != program_id {
        return Err(NftCanvasError::ProgramKeyMismatch.into());
    }
    if system_program_sysvar.key != &system_program::id() {
        return Err(NftCanvasError::RentSysvarKeyMismatch.into());
    }
    if rent_sysvar.key != &solana_program::sysvar::rent::id() {
        return Err(NftCanvasError::RentSysvarKeyMismatch.into());
    }
    if mint_pool_wallet.key != &config::mint_pool_wallet::id() {
        return Err(NftCanvasError::MintPoolWalletKeyMismatch.into());
    }
    if pixel_account.key != &pixel_account_id {
        return Err(NftCanvasError::PixelAccountKeyMismatch.into());
    }

    // Transfer sol to mint pool
    invoke(
        &solana_program::system_instruction::transfer(
            owner_wallet.key, mint_pool_wallet.key, config::MINT_COST),
        &[
            owner_wallet.clone(),
            mint_pool_wallet.clone(),
        ],
    )?;

    // Create pixel account
    invoke_signed(
        &system_instruction::create_account(
            owner_wallet.key,
            pixel_account.key,
            rent_state.minimum_balance(Pixel::PACKED_SIZE).max(1),
            Pixel::PACKED_SIZE as u64,
            program.key,
        ),
        &[
            owner_wallet.clone(),
            pixel_account.clone(),
            system_program_sysvar.clone(),
        ],
        &[
            pixel_seeds!(program.key, index, pixel_account_bump_seed),
        ],
    )?;

    // Initialize pixel account
    Pixel::new(index, color, owner_wallet.key.clone(), sell_price)
        .pack_into(&mut &mut pixel_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_update_pixel_color(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index: u32,
    color: [u8; 3],
) -> ProgramResult {
    check_pixel_index(index)?;

    let accounts_iter = &mut accounts.iter();

    let program = next_account_info(accounts_iter)?;
    let pixel_account = next_account_info(accounts_iter)?;
    let pixel_owner_wallet = next_account_info(accounts_iter)?;

    // println!("process_update_pixel_color: program={}, (owner={})", program.key, program.owner);
    // println!("process_update_pixel_color: pixel_account={}, (owner={})", pixel_account.key, pixel_account.owner);
    // println!("process_update_pixel_color: pixel_owner_wallet={}, (owner={})", pixel_owner_wallet.key, pixel_owner_wallet.owner);

    //

    let (pixel_account_id, _) = config::get_pixel(program_id, index);
    let mut pixel_account_state = Pixel::unpack(*pixel_account.data.borrow())?;

    //

    if program.key != program_id {
        return Err(NftCanvasError::ProgramKeyMismatch.into());
    }
    if pixel_account.key != &pixel_account_id {
        return Err(NftCanvasError::PixelAccountKeyMismatch.into());
    }
    if pixel_owner_wallet.key != &pixel_account_state.owner_wallet {
        return Err(NftCanvasError::PixelOwnerKeyMismatch.into());
    }
    if !pixel_owner_wallet.is_signer {
        return Err(NftCanvasError::PixelOwnerDidNotSign.into());
    }

    pixel_account_state.color = color;
    pixel_account_state.pack_into(&mut &mut pixel_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_sell_pixel(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index: u32,
    price: u64,
) -> ProgramResult {
    check_pixel_index(index)?;

    let accounts_iter = &mut accounts.iter();

    let program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let pixel_account = next_account_info(accounts_iter)?;
    let team_token_account = next_account_info(accounts_iter)?;
    let trade_pool = next_account_info(accounts_iter)?;
    let trade_pool_token_account = next_account_info(accounts_iter)?;
    let pixel_owner = next_account_info(accounts_iter)?;
    let pixel_owner_token_account = next_account_info(accounts_iter)?;

    // println!("process_sell_pixel: program={}, (owner={})", program.key, program.owner);
    // println!("process_sell_pixel: token_program={}, (owner={})", token_program.key, token_program.owner);
    // println!("process_sell_pixel: pixel_account={}, (owner={})", pixel_account.key, pixel_account.owner);
    // println!("process_sell_pixel: team_token_account={}, (owner={})", team_token_account.key, team_token_account.owner);
    // println!("process_sell_pixel: trade_pool={}, (owner={})", trade_pool.key, trade_pool.owner);
    // println!("process_sell_pixel: trade_pool_token_account={}, (owner={})", trade_pool_token_account.key, trade_pool_token_account.owner);
    // println!("process_sell_pixel: pixel_owner={}, (owner={})", pixel_owner.key, pixel_owner.owner);
    // println!("process_sell_pixel: pixel_owner_token_account={}, (owner={})", pixel_owner_token_account.key, pixel_owner_token_account.owner);

    //

    let (pixel_account_id, _) = config::get_pixel(program_id, index);
    let (trade_pool_id, trade_pool_bump_seed) = config::get_trade_pool(program_id);
    let trade_pool_token_account_id = config::get_token_account(&trade_pool_id);
    let mut pixel_account_state = Pixel::unpack(*pixel_account.data.borrow())?;
    let pixel_owner_token_account_id = config::get_token_account(&pixel_account_state.owner_wallet);

    //

    if program.key != program_id {
        return Err(NftCanvasError::ProgramKeyMismatch.into());
    }
    if token_program.key != &spl_token::id() {
        return Err(NftCanvasError::TokenProgramKeyMismatch.into());
    }
    if pixel_account.key != &pixel_account_id {
        return Err(NftCanvasError::PixelAccountKeyMismatch.into());
    }
    if team_token_account.key != &config::team_token_account::id() {
        return Err(NftCanvasError::TeamTokenAccountKeyMismatch.into());
    }
    if trade_pool.key != &trade_pool_id {
        return Err(NftCanvasError::TradePoolKeyMismatch.into());
    }
    if trade_pool_token_account.key != &trade_pool_token_account_id {
        return Err(NftCanvasError::TradePoolTokenAccountKeyMismatch.into());
    }
    if pixel_owner.key != &pixel_account_state.owner_wallet {
        return Err(NftCanvasError::PixelOwnerKeyMismatch.into());
    }
    if pixel_owner_token_account.key != &pixel_owner_token_account_id {
        return Err(NftCanvasError::PixelOwnerKeyMismatch.into());
    }

    if !pixel_owner.is_signer {
        return Err(NftCanvasError::PixelOwnerDidNotSign.into());
    }

    //

    if let Some(best_buy_info) = pixel_account_state.best_buy_info.as_ref()
        .and_then(|best_buy_info| (price != 0 && best_buy_info.price >= price).then(|| best_buy_info)) {
        // Process sell :
        let amount_split = config::TradeAmountSplit::split(best_buy_info.price);
        // - Transfer sell_price - tax to seller
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                trade_pool_token_account.key,
                pixel_owner_token_account.key,
                trade_pool.key,
                &[trade_pool.key],
                amount_split.to_seller,
            )?,
            &[trade_pool_token_account.clone(), pixel_owner_token_account.clone(), trade_pool.clone(), token_program.clone()],
            &[
                trade_pool_seeds!(program.key, trade_pool_bump_seed),
            ],
        )?;
        // - Transfer tax to team
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                trade_pool_token_account.key,
                team_token_account.key,
                trade_pool.key,
                &[trade_pool.key],
                amount_split.to_team,
            )?,
            &[trade_pool_token_account.clone(), team_token_account.clone(), trade_pool.clone(), token_program.clone()],
            &[
                trade_pool_seeds!(program.key, trade_pool_bump_seed),
            ],
        )?;
        // - Update pixel owner
        pixel_account_state.owner_wallet = best_buy_info.buyer_wallet;
        pixel_account_state.sell_price = 0;
        pixel_account_state.best_buy_info = None;
    } else {
        // Set sell price :
        // - Update pixel sell price
        pixel_account_state.sell_price = price;
    }
    pixel_account_state.pack_into(&mut &mut pixel_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_buy_pixel(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index: u32,
    price: u64,
    direct_only: u8,
) -> ProgramResult {
    check_pixel_index(index)?;

    let accounts_iter = &mut accounts.iter();

    let program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let pixel_account = next_account_info(accounts_iter)?;
    let team_token_account = next_account_info(accounts_iter)?;
    let trade_pool = next_account_info(accounts_iter)?;
    let trade_pool_token_account = next_account_info(accounts_iter)?;
    let pixel_owner = next_account_info(accounts_iter)?;
    let pixel_owner_token_account = next_account_info(accounts_iter)?;
    let buyer_wallet = next_account_info(accounts_iter)?;
    let buyer_token_account = next_account_info(accounts_iter)?;

    // println!("process_buy_pixel: program={}, (owner={})", program.key, program.owner);
    // println!("process_buy_pixel: token_program={}, (owner={})", token_program.key, token_program.owner);
    // println!("process_buy_pixel: pixel_account={}, (owner={})", pixel_account.key, pixel_account.owner);
    // println!("process_buy_pixel: team_token_account={}, (owner={})", team_token_account.key, team_token_account.owner);
    // println!("process_buy_pixel: trade_pool={}, (owner={})", trade_pool.key, trade_pool.owner);
    // println!("process_buy_pixel: trade_pool_token_account={}, (owner={})", trade_pool_token_account.key, trade_pool_token_account.owner);
    // println!("process_buy_pixel: pixel_owner={}, (owner={})", pixel_owner.key, pixel_owner.owner);
    // println!("process_buy_pixel: pixel_owner_token_account={}, (owner={})", pixel_owner_token_account.key, pixel_owner_token_account.owner);
    // println!("process_buy_pixel: buyer_wallet={}, (owner={})", buyer_wallet.key, buyer_wallet.owner);
    // println!("process_buy_pixel: buyer_token_account={}, (owner={})", buyer_token_account.key, buyer_token_account.owner);

    //

    let (pixel_account_id, _) = config::get_pixel(program_id, index);
    let (trade_pool_id, trade_pool_bump_seed) = config::get_trade_pool(program_id);
    let trade_pool_token_account_id = config::get_token_account(&trade_pool_id);

    let mut pixel_account_state = Pixel::unpack(*pixel_account.data.borrow())?;
    let pixel_owner_token_account_id = config::get_token_account(&pixel_account_state.owner_wallet);

    let buyer_token_account_id = config::get_token_account(buyer_wallet.key);

    //

    if program.key != program_id {
        return Err(NftCanvasError::ProgramKeyMismatch.into());
    }
    if token_program.key != &spl_token::id() {
        return Err(NftCanvasError::TokenProgramKeyMismatch.into());
    }
    if pixel_account.key != &pixel_account_id {
        return Err(NftCanvasError::PixelAccountKeyMismatch.into());
    }
    if team_token_account.key != &config::team_token_account::id() {
        return Err(NftCanvasError::TeamTokenAccountKeyMismatch.into());
    }
    if trade_pool.key != &trade_pool_id {
        return Err(NftCanvasError::TradePoolKeyMismatch.into());
    }
    if trade_pool_token_account.key != &trade_pool_token_account_id {
        return Err(NftCanvasError::TradePoolTokenAccountKeyMismatch.into());
    }
    if pixel_owner.key != &pixel_account_state.owner_wallet {
        return Err(NftCanvasError::PixelOwnerKeyMismatch.into());
    }
    if pixel_owner_token_account.key != &pixel_owner_token_account_id {
        return Err(NftCanvasError::PixelOwnerKeyMismatch.into());
    }
    if buyer_token_account.key != &buyer_token_account_id {
        return Err(NftCanvasError::BuyerTokenAccountKeyMismatch.into());
    }

    if !buyer_wallet.is_signer {
        return Err(NftCanvasError::PixelOwnerDidNotSign.into());
    }

    //

    if pixel_account_state.sell_price != 0 && price >= pixel_account_state.sell_price {
        // Process buy :
        let amount_split = config::TradeAmountSplit::split(price);
        // - Transfer sell_price - tax to seller
        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                buyer_token_account.key,
                pixel_owner_token_account.key,
                buyer_wallet.key,
                &[buyer_wallet.key],
                amount_split.to_seller,
            )?,
            &[buyer_token_account.clone(), pixel_owner_token_account.clone(), buyer_wallet.clone(), token_program.clone()],
        )?;
        // - Transfer tax to team
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                buyer_token_account.key,
                team_token_account.key,
                buyer_wallet.key,
                &[buyer_wallet.key],
                amount_split.to_team,
            )?,
            &[buyer_token_account.clone(), team_token_account.clone(), buyer_wallet.clone(), token_program.clone()],
            &[
                trade_pool_seeds!(program.key, trade_pool_bump_seed),
            ],
        )?;
        // - Update pixel
        pixel_account_state.owner_wallet = buyer_wallet.key.clone();
        pixel_account_state.sell_price = 0;
        // (If last best buyer is current buyer, remove order)
        if pixel_account_state.best_buy_info.as_ref().map(|best_buy_info| &best_buy_info.buyer_wallet == buyer_wallet.key).unwrap_or(false) {
            pixel_account_state.best_buy_info = None;
        }
    } else if pixel_account_state.best_buy_info.as_ref().map(|previous_buy_info| price > previous_buy_info.price).unwrap_or(true) {
        if direct_only != 0 {
            return Err(NftCanvasError::CouldNotDirectBuy.into());
        }
        // Is best buyer :
        // - Refund previous best buyer
        if let Some(previous_buy_info) = pixel_account_state.best_buy_info {
            let previous_buyer_token_account = next_account_info(accounts_iter)?;
            invoke_signed(
                &spl_token::instruction::transfer(
                    token_program.key,
                    trade_pool_token_account.key,
                    previous_buyer_token_account.key,
                    trade_pool.key,
                    &[trade_pool.key],
                    previous_buy_info.price,
                )?,
                &[trade_pool_token_account.clone(), previous_buyer_token_account.clone(), trade_pool.clone(), token_program.clone()],
                &[
                    trade_pool_seeds!(program.key, trade_pool_bump_seed),
                ],
            )?;
        }
        // - Transfer buy_price to trade pool
        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                buyer_token_account.key,
                trade_pool_token_account.key,
                buyer_wallet.key,
                &[buyer_wallet.key],
                price,
            )?,
            &[buyer_token_account.clone(), trade_pool_token_account.clone(), buyer_wallet.clone(), token_program.clone()],
        )?;
        // - Update pixel
        pixel_account_state.best_buy_info = Some(PixelBuyInfo {
            price,
            buyer_wallet: buyer_wallet.key.clone(),
        });
    } else {
        return Err(NftCanvasError::BuyPriceTooLow.into());
    }

    pixel_account_state.pack_into(&mut &mut pixel_account.data.borrow_mut()[..])?;

    Ok(())
}

// Misc

fn check_pixel_index(index: u32) -> ProgramResult {
    if index < config::PIXEL_COUNT {
        Ok(())
    } else {
        Err(NftCanvasError::PixelIndexOutOfBounds.into())
    }
}