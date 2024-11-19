#[derive(Debug)]
pub struct BitBuffer {
    pub buffer: u64,
    pub bits_in_buffer: u8,
}

impl BitBuffer {
    pub fn new() -> Self {
        BitBuffer {
            buffer: 0,
            bits_in_buffer: 0,
        }
    }

    #[inline(always)]
     pub fn add_byte(&mut self, byte: u8) {
        self.buffer = (self.buffer << 8) | byte as u64;
        self.bits_in_buffer += 8;
    }

    #[inline(always)]
    pub fn peek_bits(&self, num_bits: u8) -> u64 {
        self.buffer >> (self.bits_in_buffer - num_bits)
    }

    #[inline(always)]
    pub fn consume_bits(&mut self, num_bits: u8) {
        self.buffer &= (1 << (self.bits_in_buffer - num_bits)) - 1;
        self.bits_in_buffer -= num_bits;
    }
}
