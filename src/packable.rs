pub trait Packable {
    const PACKED_SIZE: usize;

    fn unpack(data: &[u8]) -> Result<Self, crate::error::NftCanvasError> where Self: Sized;
    fn pack(&self) -> Vec<u8>;
    fn pack_into(&self, data: &mut [u8]) -> Result<(), crate::error::NftCanvasError>;
}

#[macro_export]
macro_rules! implement_packable {
    ($for_type:ty, $packed_size:expr) => {
        impl Packable for $for_type {
            const PACKED_SIZE: usize = $packed_size;

            fn unpack(mut data: &[u8]) -> Result<Self, crate::error::NftCanvasError> {
                if data.len() != Self::PACKED_SIZE {
                    // panic!("Failed to unpack type {}, len={}, expected={}", stringify!($for_type), data.len(), Self::PACKED_SIZE);
                    return Err(crate::error::NftCanvasError::FailedToUnpackData);
                }
                assert_eq!(data.len(), Self::PACKED_SIZE);
                borsh::BorshDeserialize::deserialize(&mut data)
                    .map_err(|_| crate::error::NftCanvasError::FailedToUnpackData)
            }

            fn pack(&self) -> Vec<u8> {
                let mut result = borsh::BorshSerialize::try_to_vec(self).unwrap();
                result.resize(Self::PACKED_SIZE, 0);
                result
            }

            fn pack_into(&self, data: &mut [u8]) -> Result<(), crate::error::NftCanvasError> {
                if data.len() != Self::PACKED_SIZE {
                    // panic!("Failed to pack_into type {}, len={}, expected={}", stringify!($for_type), data.len(), Self::PACKED_SIZE);
                    return Err(crate::error::NftCanvasError::FailedToPackData);
                }
                data.copy_from_slice(&self.pack());
                Ok(())
            }
        }
    };
}