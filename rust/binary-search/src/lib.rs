pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut vec: Vec<_> = array.to_vec();
    vec.sort();

    let (mut lo, mut hi) = (0, (vec.len() - 1) as i32);

    while lo <= hi {
        let mid = (hi + lo) / 2;
        if vec[mid as usize] == key {
            return Some(mid as usize);
        } else if vec[mid as usize] > key {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    None
}
