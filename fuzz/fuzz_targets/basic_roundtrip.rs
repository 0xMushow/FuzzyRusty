#![no_main]
use libfuzzer_sys::fuzz_target;
use ssz_rs::{Deserialize, Serialize};
use ssz_rs::{Bitlist, Bitvector, List, Node, U256, Vector};

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
