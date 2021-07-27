use std::str::FromStr;

use solana_program_test::*;
use solana_sdk::pubkey::Pubkey;

use chikin_nft_canvas;
use chikin_nft_canvas::config;

#[tokio::test]
async fn test_get_id() {
    let program = Pubkey::from_str("ALaYfBMScNrJxKTfgpfFYDQSMYJHpzuxGq15TM2j6o8E").unwrap();
    let token_mint = config::token_mint::id();
    let team_token_account = config::team_token_account::id();
    let trade_pool = config::get_trade_pool(&program).0;
    let pixel0 = config::get_pixel(&program, 0).0;
    let pixel1 = config::get_pixel(&program, 1).0;
    let pixel54 = config::get_pixel(&program, 54).0;

    println!("program={}", program);
    println!("token_mint={}", token_mint);
    println!("team_token_account={}", team_token_account);
    println!("trade_pool={}", trade_pool);
    println!("pixel0={}", pixel0);
    println!("pixel1={}", pixel1);
    println!("pixel54={}", pixel54);
}