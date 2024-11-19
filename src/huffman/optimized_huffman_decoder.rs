use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::Read;
use crate::huffman::bit_buffer::BitBuffer;
use crate::huffman::hybrid_lookup_table::HybridLookupTable;
use crate::huffman::node::Node;
use crate::MAX_SHORT_BITS;

pub struct OptimizedHuffmanDecoder {
    tree: Option<Node>,
    freqs: HashMap<char, usize>,
    lookup_table: HybridLookupTable,
}

impl OptimizedHuffmanDecoder {
    pub fn new() -> Self {
        OptimizedHuffmanDecoder {
            tree: None,
            freqs: HashMap::new(),
            lookup_table: HybridLookupTable::new(MAX_SHORT_BITS as u8),
        }
    }

    pub fn build_efficient_tree(&mut self) {
        let mut heap = BinaryHeap::new();

        // Create initial nodes and add to heap
        for (&char, &freq) in &self.freqs {
            heap.push(Reverse((freq, heap.len(), Node::leaf(char, freq))));
        }

        // Build tree
        while heap.len() > 1 {
            let Reverse((freq1, _, node1)) = heap.pop().unwrap();
            let Reverse((freq2, _, node2)) = heap.pop().unwrap();

            let combined_freq = freq1 + freq2;
            let parent = Node::internal(
                combined_freq,
                *Box::new(node1),
                *Box::new(node2)
            );

            heap.push(Reverse((combined_freq, heap.len(), parent)));
        }

        self.tree = heap.pop().map(|Reverse((_, _, node))| node);
        self.build_codes();
    }

    pub fn build_codes(&mut self) {
        if let Some(ref root) = self.tree.clone() {
            self.build_codes_recursive(root, String::new());
        }
    }

    pub fn build_codes_recursive(&mut self, node: &Node, code: String) {
        if node.left.is_none() && node.right.is_none() {
            if let Some(c) = node.char {
                if !code.is_empty() {
                    self.lookup_table.add_code(&code, c);
                }
            }
            return;
        }

        if let Some(ref left) = node.left {
            self.build_codes_recursive(left, code.clone() + "0");
        }
        if let Some(ref right) = node.right {
            self.build_codes_recursive(right, code + "1");
        }
    }

    pub fn parse_header_fast(&mut self, data: &[u8]) -> usize {
        let chars_count = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
        let mut pos = 12;
        self.freqs.clear();

        for _ in 0..chars_count {
            let count = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            let character = data[pos + 4] as char;
            self.freqs.insert(character, count);
            pos += 8;
        }

        pos
    }

    // This function will now be your main entry point
    pub fn decode_file(&mut self, path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Ok(self.decode_to_bytes(&data))
    }

    pub fn decode_to_bytes(&mut self, data: &[u8]) -> Vec<u8> {
        let pos = self.parse_header_fast(data);
        self.build_efficient_tree();

        let packed_bits = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
        let packed_bytes = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
        let pos = pos + 12;

        let result = self.decode_bits(&data[pos..pos + packed_bytes], packed_bits);
        // Convert chars to bytes
        result.into_iter().flat_map(|c| c.to_string().into_bytes()).collect()
    }

    pub fn decode_bits(&self, data: &[u8], total_bits: usize) -> Vec<char> {
        let mut result = Vec::with_capacity(total_bits);
        let mut buffer = BitBuffer::new();
        let mut bytes_processed = 0;

        // Pre-fill buffer
        for &byte in data.iter().take(8) {
            buffer.add_byte(byte);
            bytes_processed += 1;
        }

        while buffer.bits_in_buffer >= 8 {
            let lookup_bits = buffer.peek_bits(8);
            let lookup_result = self.lookup_table.lookup(lookup_bits, 8);

            if let Some((character, code_len)) = lookup_result {
                buffer.consume_bits(code_len);
                result.push(character);
            } else {
                if let Some(ref root) = self.tree {
                    let mut node = root;
                    while node.left.is_some() && node.right.is_some() && buffer.bits_in_buffer > 0 {
                        let bit = (buffer.peek_bits(1) & 1) == 1;
                        buffer.consume_bits(1);
                        node = if bit {
                            node.right.as_ref().unwrap()
                        } else {
                            node.left.as_ref().unwrap()
                        };
                    }

                    if node.left.is_none() && node.right.is_none() {
                        if let Some(c) = node.char {
                            result.push(c);
                        }
                    }
                }
            }

            while buffer.bits_in_buffer <= 56 && bytes_processed < data.len() {
                buffer.add_byte(data[bytes_processed]);
                bytes_processed += 1;
            }
        }

        result
    }
}