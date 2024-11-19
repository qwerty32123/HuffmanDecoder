pub fn parse_stock_bytes(input: &[u8]) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let mut id = 0u32;
        let mut stock = 0u32;

        // Parse ID
        while i < input.len() && input[i] != b'-' {
            if input[i].is_ascii_digit() {
                id = id * 10 + (input[i] - b'0') as u32;
            }
            i += 1;
        }
        i += 1; // Skip '-'

        // Parse stock
        while i < input.len() && input[i] != b'-' {
            if input[i].is_ascii_digit() {
                stock = stock * 10 + (input[i] - b'0') as u32;
            }
            i += 1;
        }

        // Skip to next record
        while i < input.len() && input[i] != b'|' {
            i += 1;
        }
        i += 1; // Skip '|'

        if stock > 0 {
            result.push((id, stock));
        }
    }
    result
}
