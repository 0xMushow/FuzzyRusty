#![no_main]
use libfuzzer_sys::{fuzz_mutator, fuzz_target};
use ssz_rs::{Deserialize, Serialize};
use ssz_rs::{Bitlist, Bitvector, List, Node, U256, Vector};

unsafe extern "C" {
    fn LLVMFuzzerMutate(data: *mut u8, size: usize, max_size: usize) -> usize;
}

fn basic_roundtrip<T>(data: &[u8])
where
    T: Deserialize + Serialize + Eq + std::fmt::Debug,
{
    let Ok(value) = T::deserialize(data) else { return };
    let mut buf = Vec::new();
    value.serialize(&mut buf).unwrap();
    let re = T::deserialize(&buf[..]).unwrap();
    assert_eq!(value, re);
}

fuzz_target!(|data: &[u8]| {
    basic_roundtrip::<Bitlist<64>>(data);
    basic_roundtrip::<Bitvector<128>>(data);
    basic_roundtrip::<U256>(data);
    basic_roundtrip::<Node>(data);
    basic_roundtrip::<Vector<u8, 16>>(data);
    basic_roundtrip::<List<u32, 32>>(data);
});


fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
let mut x = seed as u64;
    fn rnd_byte(x: &mut u64) -> u8 {
        *x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        (*x >> 32) as u8
    }

    match seed % 10 {
        0 => {
            // Generate a valid SSZ Bitlist<64>
            let bit_len = (seed as usize % 65).min(64);
            // SSZ bitlist = ceil(n/8) bytes of bits + 1 byte termination bit
            let sz = ((bit_len + 7) / 8) + 1;
            let sz = sz.min(max_size);
            // fill with random bits...
            for b in &mut data[..sz] {
                *b = rnd_byte(&mut x);
            }
            // set the termination bit at position `bit_len`
            let idx = bit_len / 8;
            let pos = bit_len % 8;
            data[idx] |= 1 << pos;
            sz
        }
        1 => {
            // Valid SSZ Bitvector<128> = exactly 16 bytes
            let sz = 16.min(max_size);
            for b in &mut data[..sz] {
                *b = rnd_byte(&mut x);
            }
            sz
        }
        2 => {
            // Valid SSZ U256 = exactly 32 bytes LE
            let sz = 32.min(max_size);
            for i in 0..sz {
                data[i] = rnd_byte(&mut x);
            }
            sz
        }
        3 => {
            // Valid SSZ Node (32-byte hash)
            let sz = 32.min(max_size);
            for i in 0..sz {
                data[i] = rnd_byte(&mut x);
            }
            sz
        }
        4 => {
            // Valid SSZ Vector<u8,16> = exactly 16 bytes
            let sz = 16.min(max_size);
            for b in &mut data[..sz] {
                *b = rnd_byte(&mut x);
            }
            sz
        }
        5 => {
            // Valid SSZ List<u32,32> = 4-byte length + N×4-byte elements
            let max_elems = (max_size.saturating_sub(4)) / 4;
            let n = ((seed as usize) % 33).min(max_elems);
            let total = 4 + n * 4;
            // write length prefix
            data[..4].copy_from_slice(&(n as u32).to_le_bytes());
            // write each u32
            for i in 0..n {
                let rnd = u32::from_le_bytes([
                    rnd_byte(&mut x),
                    rnd_byte(&mut x),
                    rnd_byte(&mut x),
                    rnd_byte(&mut x),
                ]);
                data[4 + i * 4..4 + (i + 1) * 4].copy_from_slice(&rnd.to_le_bytes());
            }
            total
        }
        _ => {
            unsafe { LLVMFuzzerMutate(data.as_mut_ptr(), size, max_size) }
        }
    }
});