//! Error types

use solana_program::decode_error::DecodeError;
use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NftCanvasError {
    #[error("ProgramKeyMismatch")]
    ProgramKeyMismatch,
    #[error("RentSysvarKeyMismatch")]
    RentSysvarKeyMismatch,
    #[error("TokenProgramKeyMismatch")]
    TokenProgramKeyMismatch,
    #[error("PixelAccountKeyMismatch")]
    PixelAccountKeyMismatch,
    #[error("MintPoolWalletKeyMismatch")]
    MintPoolWalletKeyMismatch,
    #[error("TeamWalletKeyMismatch")]
    TeamWalletKeyMismatch,
    #[error("TeamTokenAccountKeyMismatch")]
    TeamTokenAccountKeyMismatch,
    #[error("TradePoolKeyMismatch")]
    TradePoolKeyMismatch,
    #[error("TradePoolTokenAccountKeyMismatch")]
    TradePoolTokenAccountKeyMismatch,
    #[error("PixelOwnerKeyMismatch")]
    PixelOwnerKeyMismatch,
    #[error("BuyerTokenAccountKeyMismatch")]
    BuyerTokenAccountKeyMismatch,

    #[error("FunderDidNotSign")]
    FunderDidNotSign,
    #[error("PixelOwnerDidNotSign")]
    PixelOwnerDidNotSign,

    #[error("PixelIndexOutOfBounds")]
    PixelIndexOutOfBounds,
    #[error("PixelUninitialized")]
    PixelUninitialized,
    #[error("BuyPriceTooLow")]
    BuyPriceTooLow,
    #[error("CouldNotDirectBuy")]
    CouldNotDirectBuy,

    #[error("FailedToPackData")]
    FailedToPackData,
    #[error("FailedToUnpackData")]
    FailedToUnpackData,
}

impl From<NftCanvasError> for ProgramError {
    fn from(e: NftCanvasError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for NftCanvasError {
    fn type_of() -> &'static str { "NftCanvasError" }
}