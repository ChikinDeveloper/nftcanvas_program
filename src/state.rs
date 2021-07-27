use borsh::BorshDeserialize;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

use crate::packable::Packable;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct Pixel {
    pub index: u32,
    pub color: [u8; 3],
    pub owner_wallet: Pubkey,
    pub sell_price: u64,
    pub best_buy_info: Option<PixelBuyInfo>,
}

implement_packable!(Pixel, 88);

impl Pixel {
    pub fn new(index: u32, color: [u8; 3], owner_wallet: Pubkey, sell_price: u64) -> Pixel {
        Pixel {
            index,
            color,
            owner_wallet,
            sell_price,
            best_buy_info: None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct PixelBuyInfo {
    pub price: u64,
    pub buyer_wallet: Pubkey,
}