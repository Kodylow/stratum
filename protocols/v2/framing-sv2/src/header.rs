#[cfg(not(feature = "with_serde"))]
use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
use binary_sv2::{Deserialize, Serialize, U24};
use core::convert::TryInto;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Header {
    extension_type: u16, // TODO use specific type?
    msg_type: u8,        // TODO use specific type?
    msg_length: U24,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            extension_type: 0,
            msg_type: 0,
            // converting 0_32 into a U24 never panic
            msg_length: 0_u32.try_into().unwrap(),
        }
    }
}

impl Header {
    pub const LEN_OFFSET: usize = const_sv2::SV2_FRAME_HEADER_LEN_OFFSET;
    pub const LEN_SIZE: usize = const_sv2::SV2_FRAME_HEADER_LEN_END;
    pub const LEN_END: usize = Self::LEN_OFFSET + Self::LEN_SIZE;

    pub const SIZE: usize = const_sv2::SV2_FRAME_HEADER_SIZE;

    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, isize> {
        if bytes.len() < Self::SIZE {
            return Err((Self::SIZE - bytes.len()) as isize);
        };

        let extension_type = u16::from_le_bytes([bytes[0], bytes[1]]);
        let msg_type = bytes[2];
        let msg_length = u32::from_le_bytes([bytes[3], bytes[4], bytes[5], 0]);

        Ok(Self {
            extension_type,
            msg_type,
            // Converting and u32 with the most significant byte set to 0 to and U24 never panic
            msg_length: msg_length.try_into().unwrap(),
        })
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        let inner: u32 = self.msg_length.into();
        inner as usize
    }

    #[inline]
    pub fn from_len(len: u32, message_type: u8, extension_type: u16) -> Option<Header> {
        Some(Self {
            extension_type,
            msg_type: message_type,
            msg_length: len.try_into().ok()?,
        })
    }

    pub fn msg_type(&self) -> u8 {
        self.msg_type
    }

    pub fn channel_msg(&self) -> bool {
        let mask = 0b0000_0000_0000_0001;
        self.extension_type & mask == self.extension_type
    }
}

pub struct NoiseHeader {}

impl NoiseHeader {
    pub const SIZE: usize = const_sv2::NOISE_FRAME_HEADER_SIZE;
    pub const LEN_OFFSET: usize = const_sv2::NOISE_FRAME_HEADER_LEN_OFFSET;
    pub const LEN_END: usize = const_sv2::NOISE_FRAME_HEADER_LEN_END;
}
