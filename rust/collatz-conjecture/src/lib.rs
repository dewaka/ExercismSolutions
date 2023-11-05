pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    let mut n = n;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            let res = n.checked_mul(3).and_then(|x| x.checked_add(1));
            if res.is_none() {
                return None;
            } else {
                n = res.unwrap();
            }
        }
        steps += 1;
    }
    Some(steps)
}
