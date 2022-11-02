#![feature(async_fn_in_trait)]

use bytes::Bytes;
use std::io::Write;

trait KvIterator {
    async fn next(&mut self) -> Option<(&[u8], &[u8])>;
}

struct TestIterator {
    idx: usize,
    to_idx: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl KvIterator for TestIterator {
    async fn next(&mut self) -> Option<(&[u8], &[u8])> {
        if self.idx >= self.to_idx {
            return None;
        }
        self.key.clear();
        write!(&mut self.key, "key_{:05}", self.idx).unwrap();

        self.value.clear();
        write!(&mut self.value, "value_{:05}", self.idx).unwrap();

        self.idx += 1;
        Some((&self.key[..], &self.value[..]))
    }
}

impl TestIterator {
    pub fn new(from_idx: usize, to_idx: usize) -> Self {
        Self {
            idx: from_idx,
            to_idx,
            key: Vec::new(),
            value: Vec::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut iter = TestIterator::new(0, 10);
    while let Some((key, value)) = iter.next().await {
        println!(
            "{:?} {:?}",
            Bytes::copy_from_slice(key),
            Bytes::copy_from_slice(value)
        )
    }
}
