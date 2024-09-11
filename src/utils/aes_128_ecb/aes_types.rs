const AES_BLOCK_SIZE: usize = 16; // AES block size is 16 bytes (128 bits).
const AES_WORD_SIZE: usize = 8; // 8 slices to represent 128 bits.

pub type AESState = [u8; AES_BLOCK_SIZE]; // A standard 4x4 state for AES.
pub type BitSlicedState = [Word; AES_WORD_SIZE]; // Bit-sliced representation of the state.

pub type Word = u16;
