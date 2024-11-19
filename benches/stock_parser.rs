// Add this to your stock_parser.rs or create a new file

use std::time::Instant;

pub fn parse_stock_simple(input: &str) -> Vec<(u32, u32)> {
    input
        .split('|')
        .filter(|s| !s.is_empty())
        .filter_map(|item| {
            let parts: Vec<&str> = item.split('-').collect();
            if parts.len() >= 2 {
                if let (Ok(id), Ok(stock)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    if stock > 0 {
                        return Some((id, stock));
                    }
                }
            }
            None
        })
        .collect()
}

pub fn parse_stock_iterator(input: &str) -> Vec<(u32, u32)> {
    input
        .split('|')
        .take_while(|s| !s.is_empty())
        .filter_map(|item| {
            let mut iter = item.split('-');
            match (iter.next(), iter.next()) {
                (Some(id), Some(stock)) => {
                    if let (Ok(id_num), Ok(stock_num)) = (id.parse::<u32>(), stock.parse::<u32>()) {
                        if stock_num > 0 {
                            return Some((id_num, stock_num));
                        }
                    }
                }
                _ => (),
            }
            None
        })
        .collect()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_data(size: usize) -> String {
        let mut result = String::with_capacity(size * 30);
        for i in 0..size {
            let stock = if i % 3 == 0 { "1" } else { "0" };
            result.push_str(&format!("2{:04}-{}-{}-1100000000|", i, stock, i * 10));
        }
        result
    }

    #[test]
    fn benchmark_implementations() {
        // Test with small data
        let small_data = "20067-0-104-1100000000|20069-1-47-1100000000|21021-0-447-1630000000|21022-0-1010-1630000000|";

        println!("\nSmall data test (4 items):");

        // Run each implementation multiple times to get more accurate results
        for _ in 0..5 {
            let start = Instant::now();
            let result = parse_stock_simple(small_data);
            let duration = start.elapsed();

            let start = Instant::now();
            let result = parse_stock_iterator(small_data);
            let duration = start.elapsed();

            let start = Instant::now();
            let result = parse_stock_bytes(small_data.as_ref());
            let duration = start.elapsed();
            println!("Bytes implementation: {:?} - Found {:?} items", duration, result);
            println!("Bytes implementation: {:?} - Found {} items", duration, result.len());

            println!("---");
        }

        // Test with medium data (1000 items)
        let medium_data = generate_test_data(1000);

        println!("\nMedium data test (1000 items):");

        for _ in 0..5 {
            let start = Instant::now();
            let result = parse_stock_simple(&medium_data);
            let duration = start.elapsed();
            println!("Simple implementation: {:?} - Found {} items", duration, result.len());

            let start = Instant::now();
            let result = parse_stock_iterator(&medium_data);
            let duration = start.elapsed();
            println!("Iterator implementation: {:?} - Found {} items", duration, result.len());

            let start = Instant::now();
            let result = parse_stock_bytes((&medium_data).as_ref());
            let duration = start.elapsed();
            println!("Bytes implementation: {:?} - Found {} items", duration, result.len());

            println!("---");
        }

        // Test with large data (100000 items)
        let large_data = generate_test_data(100000);

        println!("\nLarge data test (100000 items):");

        for _ in 0..5 {
            let start = Instant::now();
            let result = parse_stock_simple(&large_data);
            let duration = start.elapsed();
            println!("Simple implementation: {:?} - Found {} items", duration, result.len());

            let start = Instant::now();
            let result = parse_stock_iterator(&large_data);
            let duration = start.elapsed();
            println!("Iterator implementation: {:?} - Found {} items", duration, result.len());

            let start = Instant::now();
            let result = parse_stock_bytes((&large_data).as_ref());
            let duration = start.elapsed();
            println!("Bytes implementation: {:?} - Found {} items", duration, result.len());

            println!("---");
        }
    }
}