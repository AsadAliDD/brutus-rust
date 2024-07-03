use sha2::{Digest, Sha256};
use std::fs::read_to_string;

pub fn count_lines(filename: &str) -> u128 {
    let mut num_lines: u128 = 0;
    for _ in read_to_string(filename).unwrap().lines() {
        num_lines += 1;
    }
    num_lines
}

pub fn read_lines(filename: &str, startline: u128, endline: u128) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut i: u128 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        i += 1;
        if i >= startline && i <= endline {
            lines.push(line.to_string());
        }
    }
    lines
}

pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn split_chunks(line_count: &u128, chunk_size: &u16) -> Vec<(u128, u128)> {
    let mut chunks: Vec<(u128, u128)> = Vec::new();
    let num_chunks = *line_count / *chunk_size as u128;

    for i in 0..num_chunks {
        let start = i * *chunk_size as u128 + 1;
        let mut end = (i + 1) * *chunk_size as u128;
        if i == num_chunks - 1 {
            end = *line_count;
        }
        chunks.push((start, end));
    }
    chunks
}
