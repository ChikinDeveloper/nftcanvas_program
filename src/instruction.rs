#![allow(dead_code)]

use borsh::BorshDeserialize;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use solana_program;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

use crate::packable::Packable;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum NftCanvasInstruction {
    MintPixel { index: u32, color: [u8; 3], sell_price: u64 },
    UpdatePixelColor { index: u32, color: [u8; 3] },
    SellPixel { index: u32, price: u64 },
    BuyPixel { index: u32, price: u64, direct_only: u8 },
}

impl NftCanvasInstruction {
    pub fn mint_pixel(
        program: Pubkey,
        system_program: Pubkey,
        rent_sysvar: Pubkey,
        mint_pool_wallet: Pubkey,
        pixel_account: Pubkey,
        owner_wallet: Pubkey,
        index: u32,
        color: [u8; 3],
        sell_price: u64,
    ) -> Instruction {
        let object = NftCanvasInstruction::MintPixel { index, color, sell_price };
        let data: Vec<u8> = object.pack();

        let accounts = vec![
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent_sysvar, false),
            AccountMeta::new(mint_pool_wallet, false),
            AccountMeta::new(pixel_account, false),
            AccountMeta::new(owner_wallet, true),
        ];

        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn update_pixel_color(
        program: Pubkey,
        pixel_account: Pubkey,
        owner_wallet: Pubkey,
        index: u32,
        color: [u8; 3],
    ) -> Instruction {
        let object = NftCanvasInstruction::UpdatePixelColor { index, color };
        let data: Vec<u8> = object.pack();

        let accounts = vec![
            AccountMeta::new_readonly(program, false),
            AccountMeta::new(pixel_account, false),
            AccountMeta::new(owner_wallet, true),
        ];

        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn sell_pixel(
        program: Pubkey,
        token_program: Pubkey,
        pixel_account: Pubkey,
        team_token_account: Pubkey,
        trade_pool: Pubkey,
        trade_pool_token_account: Pubkey,
        pixel_owner: Pubkey,
        pixel_owner_token_account: Pubkey,
        index: u32,
        price: u64,
    ) -> Instruction {
        let object = NftCanvasInstruction::SellPixel { index, price };
        let data: Vec<u8> = object.pack();

        let accounts = vec![
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(pixel_account, false),
            AccountMeta::new(team_token_account, false),
            AccountMeta::new(trade_pool, false),
            AccountMeta::new(trade_pool_token_account, false),
            AccountMeta::new(pixel_owner, true),
            AccountMeta::new(pixel_owner_token_account, false),
        ];

        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn buy_pixel(
        program: Pubkey,
        token_program: Pubkey,
        pixel_account: Pubkey,
        team_token_account: Pubkey,
        trade_pool: Pubkey,
        trade_pool_token_account: Pubkey,
        pixel_owner: Pubkey,
        pixel_owner_token_account: Pubkey,
        buyer_wallet: Pubkey,
        buyer_token_account: Pubkey,
        index: u32,
        price: u64,
        direct_only: u8,
    ) -> Instruction {
        let object = NftCanvasInstruction::BuyPixel { index, price, direct_only };
        let data: Vec<u8> = object.pack();

        let accounts = vec![
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(pixel_account, false),
            AccountMeta::new(team_token_account, false),
            AccountMeta::new(trade_pool, false),
            AccountMeta::new(trade_pool_token_account, false),
            AccountMeta::new(pixel_owner, false),
            AccountMeta::new(pixel_owner_token_account, false),
            AccountMeta::new(buyer_wallet, true),
            AccountMeta::new(buyer_token_account, false),
        ];

        Instruction::new_with_bytes(program, &data, accounts)
    }
}

implement_packable!(NftCanvasInstruction, 16);
