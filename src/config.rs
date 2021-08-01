use solana_program::pubkey::Pubkey;
use spl_associated_token_account;

pub const PIXEL_COUNT: u32 = 1000 * 1000; // width * height
pub const MINT_COST: u64 = 1_000_000; // 0.001 Sol
pub const TAX_DIV: u64 = 100; // 1%

pub mod token_mint {
    use solana_program::declare_id;

    declare_id!("8s9FCz99Wcr3dHpiauFRi6bLXzshXfcGTfgQE7UEopVx");
}

pub mod mint_pool_wallet {
    use solana_program::declare_id;

    declare_id!("ARamwbZzoaRjiEnHM2oVmD5bqPpGPNuxUuXWRzsacgaz");
}

pub mod team_token_account {
    use solana_program::declare_id;

    declare_id!("Esi6Z7reZt9NjZ2TeTFRXcTez1XA7764dE9bZoKCdjTb");
}

#[inline(always)]
pub fn get_trade_pool(program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program.to_bytes(),
        "trade_pool".as_bytes(),
    ], program)
}

#[macro_export]
macro_rules! trade_pool_seeds {
    ($program:expr, $bump_seed:expr) => {
        &[
            $program.as_ref(),
            "trade_pool".as_bytes(),
            &[$bump_seed],
        ]
    };
}

#[inline(always)]
pub fn get_pixel(program: &Pubkey, index: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program.to_bytes(),
        "pixel".as_bytes(),
        &index.to_le_bytes(),
    ], program)
}

#[macro_export]
macro_rules! pixel_seeds {
    ($program:expr, $index: expr, $bump_seed:expr) => {
        &[
            $program.as_ref(),
            "pixel".as_bytes(),
            &$index.to_le_bytes(),
            &[$bump_seed],
        ]
    };
}

pub fn get_token_account(owner: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(owner, &token_mint::id())
}

pub struct TradeAmountSplit {
    pub to_seller: u64,
    pub to_team: u64,
}

impl TradeAmountSplit {
    pub fn split(amount: u64) -> TradeAmountSplit {
        let to_team = amount / TAX_DIV;
        TradeAmountSplit {
            to_seller: amount - to_team,
            to_team,
        }
    }
}