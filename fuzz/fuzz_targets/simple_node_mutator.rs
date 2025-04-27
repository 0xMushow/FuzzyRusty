#![no_main]

use libfuzzer_sys::fuzz_target;
use rand::Rng;
use ssz_rs::{Deserialize, Merkleized, Serialize};

fuzz_target!(|data: &[u8]| {
    match ssz_rs::Node::deserialize(data) {
        Ok(mut node) => {
            let mut buf = Vec::new();
            node.serialize(&mut buf).unwrap();
            let mut node2 = ssz_rs::Node::deserialize(&buf).unwrap();
            assert_eq!(node, node2);
            assert_eq!(node.hash_tree_root().unwrap(), node2.hash_tree_root().unwrap());

            // === one‐bit mutator ===
            let mut buf2 = buf.clone();
            if !buf2.is_empty() {
                let mut rng = rand::rng();
                let byte_idx = rng.random_range(0..buf2.len());
                let bit_idx  = rng.random_range(0..8);
                buf2[byte_idx] ^= 1 << bit_idx;
            }

            let mut different_node = ssz_rs::Node::deserialize(&buf2).unwrap();

            assert_ne!(node.hash_tree_root().unwrap(), different_node.hash_tree_root().unwrap());

        }
        Err(_) => {}
    }
});
