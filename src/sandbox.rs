use rawmem::Rawmem;

const START: u8 = 0u8;
const END: u8 = 128u8;

fn avg_hamming_dist() -> f64 {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    for byte in START..=END {
        sum += Rawmem::hamming_weight(byte) as u64;
        count += 1
    }
    (sum as f64) / (count as f64)
}

fn avg_hamming_dist2(mask: u8) -> f64 {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    for byte1 in START..=END {
        for byte2 in START..=END {
            sum += Rawmem::hamming_weight((byte1 ^ mask) ^ (byte2 ^ mask)) as u64;
            count += 1
        }
    }
    (sum as f64) / (count as f64)
}

fn avg_hamming_dist3() -> f64 {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    for byte1 in START..=END {
        for byte2 in START..=END {
            for mask in 0u8..=255u8 {
                sum += Rawmem::hamming_weight((byte1 ^ mask) ^ (byte2 ^ mask)) as u64;
                count += 1
            }
        }
    }
    (sum as f64) / (count as f64)
}

fn avg_hamming_dist4() -> f64 {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    for byte1 in START..=END {
        for byte2 in START..=END {
            for mask1 in 0u8..=255u8 {
                for mask2 in 0u8..=255u8 {
                    sum += Rawmem::hamming_weight((byte1 ^ mask1) ^ (byte2 ^ mask2)) as u64;
                    count += 1
                }
            }
        }
    }
    (sum as f64) / (count as f64)
}


fn avg_hamming_dist5(start: u8, end: u8) -> f64 {

    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    for byte1 in start..=end {
        for byte2 in start..=end {
            sum += Rawmem::hamming_weight(byte1 ^ byte2) as u64;
            count += 1
        }
    }
    (sum as f64) / (count as f64)
}

fn main() {

    println!("{}", avg_hamming_dist5(0u8, 96u8));
    println!("{}", avg_hamming_dist5(128u8, 255u8));
    println!("{}", avg_hamming_dist5(0u8, 255u8));
}
