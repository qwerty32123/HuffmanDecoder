use std::collections::HashMap;

#[derive(Debug)]
pub struct HybridLookupTable {
   pub short_table: HashMap<u32, (char, u8)>,
   pub long_codes: HashMap<u64, (char, u8)>,
   pub max_short_bits: u8,
}

impl HybridLookupTable {
    pub fn new(max_short_bits: u8) -> Self {
        HybridLookupTable {
            short_table: HashMap::with_capacity(1 << max_short_bits),
            long_codes: HashMap::new(),
            max_short_bits,
        }
    }

    pub fn add_code(&mut self, code: &str, character: char) {
        let code_int = u64::from_str_radix(code, 2).unwrap();
        let code_len = code.len() as u8;

        if code_len <= self.max_short_bits {
            let prefix_mask = (1u32 << (self.max_short_bits - code_len)) - 1;
            let base_index = (code_int as u32) << (self.max_short_bits - code_len);

            for i in 0..=prefix_mask {
                self.short_table.insert(base_index | i, (character, code_len));
            }
        } else {
            self.long_codes.insert(code_int, (character, code_len));
        }
    }

   pub fn lookup(&self, bits: u64, length: u8) -> Option<(char, u8)> {
        if length <= self.max_short_bits {
            let mask = (1u32 << self.max_short_bits) - 1;
            return self.short_table.get(&((bits as u32) & mask))
                .copied();
        }

        for (&code_bits, &(character, code_len)) in &self.long_codes {
            if code_len <= length {
                let mask = (1 << code_len) - 1;
                if (bits >> (length - code_len)) == (code_bits & mask) {
                    return Some((character, code_len));
                }
            }
        }
        None
    }
}