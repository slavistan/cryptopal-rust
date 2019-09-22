
use rawmem::hamming;
use std::slice;

fn main() {
    for ustop in 0u8..=255u8 {
        for ustart in 0u8..=ustop {
            for key in 0u8..=255u8 {
                if hammdist(ustart, ustop, key) < 0 {
                    println!("asdf");
                }
            }
        }
    }
}

fn hammdist(ustart: u8, ustop: u8, key: u8) -> i64{

    let mut uxu = 0u64;
    for bytea in ustart..=ustop {
        for byteb in ustart..=ustop {
            uxu += hamming::hamming_distance(slice::from_ref(&bytea), slice::from_ref(&byteb));
        }
    }

    let mut uxuxk = 0u64;
    for bytea in ustart..=ustop {
        for byteb in ustart..=ustop {
            uxuxk += hamming::hamming_distance(slice::from_ref(&bytea), slice::from_ref(&(byteb^key)));
        }
    }

    (uxuxk as i64) - (uxu as i64)
}

