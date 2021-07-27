use solana_program::borsh::get_packed_len;
use solana_program_test::*;
use solana_sdk::native_token::Sol;
use solana_sdk::rent::Rent;

use chikin_nft_canvas;
use chikin_nft_canvas::instruction::NftCanvasInstruction;
use chikin_nft_canvas::state::Pixel;
use chikin_nft_canvas::config;

#[tokio::test]
async fn test_packed_size() {
    println!("NftCanvasInstruction.len={}", get_packed_len::<NftCanvasInstruction>());
    println!("Pixel.len={}", get_packed_len::<Pixel>());
    let rent = Rent::default().minimum_balance(get_packed_len::<Pixel>());
    println!("Pixel.rent={}", Sol(rent));
    let pixel_count = 1000 * 1000;
    println!("Pixel.rent.total={}", Sol(rent * pixel_count));

    println!("MINT_COST={}", Sol(config::MINT_COST));
}