use crate::{PublicKey, Seed};

use hex::ToHex;
use regex::Regex;

use std::io::Write;
use std::sync::atomic::{AtomicU8, Ordering};

pub fn handle_keypair(
    seed: &Seed,
    pk: &PublicKey,
    regexes: &Vec<Regex>,
    max_leading_zeros: &Vec<AtomicU8>,
) {
    let leading_zeros = leading_zeros_of_pubkey(pk);

    let str_addr = address_for_pubkey(pk).to_string();

    for (re, mlz) in regexes.iter().zip(max_leading_zeros.iter()) {
        if mlz.load(Ordering::Relaxed) > leading_zeros {
            continue;
        }
        if !(re.is_match(&str_addr)) {
            continue;
        }

        if mlz.fetch_max(leading_zeros, Ordering::AcqRel) <= leading_zeros {
            let mut sk = [0u8; 64];
            sk[..32].copy_from_slice(seed);
            sk[32..].copy_from_slice(pk);
            let mut lock = std::io::stdout().lock();
            writeln!(lock, "=======================================").unwrap();
            writeln!(lock, "PrivateKey: {}", sk.encode_hex::<String>()).unwrap();
            writeln!(lock, "PublicKey: {}", pk.encode_hex::<String>()).unwrap();
            writeln!(lock, "Address: {}", str_addr).unwrap();
            writeln!(lock, "Height: {}", leading_zeros).unwrap();
            writeln!(lock, "=======================================").unwrap();
        };
    }
}

fn leading_zeros_of_pubkey(pk: &[u8]) -> u8 {
    let mut zeros = 0u8;
    for b in pk {
        let z = b.leading_zeros();
        zeros += z as u8;
        if z != 8 {
            break;
        }
    }
    zeros
}

fn address_for_pubkey(pk: &[u8]) -> std::net::Ipv6Addr {
    let zeros = leading_zeros_of_pubkey(pk);
    let mut buf = [0u8; 16];
    buf[0] = 0x02;
    buf[1] = zeros;
    for (src, trg) in pk[((zeros / 8) as usize)..]
        .windows(2)
        .zip(buf[2..].iter_mut())
    {
        *trg = src[0].wrapping_shl(((zeros + 1) % 8) as u32)
            ^ src[1].wrapping_shr(8 - ((zeros + 1) % 8) as u32)
            ^ 0xFF;
    }
    std::net::Ipv6Addr::from(buf)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::address_for_pubkey;

    #[test]
    fn test_address_for_pubkey() {
        assert_eq!(
            address_for_pubkey(
                hex::decode("000000000c4f58e09d19592f242951e6aa3185bd5ec6b95c0d56c93ae1268cbd")
                    .unwrap()
                    .as_slice()
            ),
            std::net::Ipv6Addr::from_str("224:7614:e3ec:5cd4:da1b:7ad5:c32a:b9cf").unwrap()
        )
    }
}
