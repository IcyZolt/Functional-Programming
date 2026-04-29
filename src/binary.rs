fn do_binary_addition(a: &[i32], b: &[i32], carry: i32) -> Vec<i32> {
    match (a, b, carry) {
        ([], [], 0) => vec![],
        ([], [], 1) => vec![1],
        ([], _, _)  => do_binary_addition(&[0], b, carry),
        (_, [], _)  => do_binary_addition(a, &[0], carry),
        ([ah, ar @ ..], [bh, br @ ..], _) => {
            let bit_sum = ah + bh + carry;
            let bit     = bit_sum - (bit_sum / 2) * 2;
            let next    = bit_sum / 2;
            let mut result = vec![bit];
            result.extend(do_binary_addition(ar, br, next));
            result
        }
    }
}

pub fn binary_addition(a: &[i32], b: &[i32]) -> Vec<i32> {
    let ra = reverse_list(a);
    let rb = reverse_list(b);
    let raw = do_binary_addition(&ra, &rb, 0);
    reverse_list(&raw)
}

pub fn reverse_list(list: &[i32]) -> Vec<i32> {
    if list.is_empty() {
        return vec![];
    }
    let mut result = reverse_list(&list[1..]);
    result.push(list[0]);
    result
}

pub fn flip_bits(list: &[i32]) -> Vec<i32> {
    match list {
        [] => vec![],
        [h, rest @ ..] => {
            let flipped = if *h == 0 { 1 } else { 0 };
            let mut result = vec![flipped];
            result.extend(flip_bits(rest));
            result
        }
    }
}

pub fn pad_list(list: &[i32], len: usize) -> Vec<i32> {
    if list.len() >= len {
        return list.to_vec();
    }
    let mut result = vec![0; len - list.len()];
    result.extend_from_slice(list);
    result
}

pub fn twos_complement(list: &[i32]) -> Vec<i32> {
    let flipped = flip_bits(list);
    binary_addition(&flipped, &[1])
}

pub fn drop_leading_bit(list: &[i32], max_len: usize) -> Vec<i32> {
    if list.len() <= max_len {
        list.to_vec()
    } else {
        list[list.len() - max_len..].to_vec()
    }
}

pub fn binary_subtraction(a: &[i32], b: &[i32]) -> Vec<i32> {
    let max_len = a.len().max(b.len());
    let pa = pad_list(a, max_len);
    let pb = pad_list(b, max_len);
    let tc  = twos_complement(&pb);
    let raw = binary_addition(&pa, &tc);
    drop_leading_bit(&raw, max_len)
}
