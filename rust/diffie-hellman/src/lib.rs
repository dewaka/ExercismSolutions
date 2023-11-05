use rand::prelude::*;

fn mod_pow(b: u128, e: u64, m: u64) -> u64 {
    let mut e = e;
    let mut b = b;
    let mut r = 1;

    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m as u128;
        }
        b = (b * b) % m as u128;
        e /= 2;
    }

    r as u64
}

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(a as u128, b_pub, p)
}
