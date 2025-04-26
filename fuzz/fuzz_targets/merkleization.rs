#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};
use std::convert::TryFrom;
use std::mem;
use ssz_rs::{Node, is_valid_merkle_branch};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    leaf: [u8; 32],
    branch: Vec<[u8; 32]>,
    index: usize,
    root: [u8; 32],
}

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = Unstructured::new(data).arbitrary::<FuzzInput>() {
        let mut depth = input.branch.len();
        let index = if depth > 0 { input.index % depth } else { 0 };

        ///////// MANDATORY CHECKS //////////
        /////////// AVOID PANIC /////////////
        let max_depth = mem::size_of::<usize>() * 8;
        if depth > max_depth {
            return
        }
        /////////// AVOID PANIC /////////////
        ///////// MANDATORY CHECKS //////////

        let leaf_node  = Node::try_from(&input.leaf[..]).unwrap();
        let branch_nodes: Vec<_> = input.branch
            .into_iter()
            .map(|b| Node::try_from(&b[..]).unwrap())
            .collect();
        let root_node = Node::try_from(&input.root[..]).unwrap();

        is_valid_merkle_branch(
            &leaf_node,
            branch_nodes.iter(),
            depth,
            index,
            &root_node,
        );
    }
});
