## Vulnerability: Exponent Overflow in `is_valid_merkle_branch`

When the Merkle‐proof `depth` approaches the bit-width of `usize`, this line in  
`scr/merkleization/proofs.rs:21`

```rust
if (index / 2usize.pow(i as u32)) % 2 != 0 {
// *** //
}
```

can panic with “attempt to multiply with overflow” (because `2usize.pow(i)` overflows once `i ≥ usize::BITS`).

### How it should work in a standalone environment

Instead of computing powers of two and dividing, use a bit shift to extract the i-th bit of `index` safely, and bound `depth` by `usize::BITS`:

```rust
pub fn is_valid_merkle_branch<'a>(
    leaf: &Node,
    mut branch: impl Iterator<Item = &'a Node>,
    depth: usize,
    index: usize,
    root: &Node,
) -> bool {
    // --- ADD THIS GUARD ---
    let max_depth = usize::BITS as usize;
    if depth > max_depth {
        return false;
    }
    // -----------------------

    let mut value = *leaf;
    let mut hasher = Sha256::new();

    for i in 0..depth {
        let next_node = match branch.next() {
            Some(node) => node,
            None => return false,
        };
        
        if ((index >> i) & 1) != 0 {
            hasher.update(next_node.as_ref());
            hasher.update(value.as_ref());
        } else {
            hasher.update(value.as_ref());
            hasher.update(next_node.as_ref());
        }

        value.as_mut().copy_from_slice(&hasher.finalize_reset());
    }

    value == *root
}

```

1. **Bounds `depth`** to `usize::BITS` so we never shift by or divide with an out-of-range exponent.
2. **Uses `index >> i`** and bit-masking `& 1` instead of `2usize.pow(i)` and division, avoiding any multiplication overflow.  
