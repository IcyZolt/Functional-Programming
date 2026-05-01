fn reverse_list(list: &[i32]) -> Vec<i32> {
    if list.is_empty() {
        return vec![];
    }
    //recursively reverse: reverse the tail, then push the head
    let mut result = reverse_list(&list[1..]);
    result.push(list[0]);
    result
}

fn do_binary_addition(a: &[i32], b: &[i32], carry: i32) -> Vec<i32> {
    match (a, b, carry) {
        ([], [], 0) => vec![],
        ([], [], 1) => vec![1],  //final carry-out
        ([], _, _)  => do_binary_addition(&[0], b, carry),  //pad with zero
        (_, [], _)  => do_binary_addition(a, &[0], carry),  //pad with zero
        ([ah, ar @ ..], [bh, br @ ..], _) => {
            let bit_sum = ah + bh + carry;
            let bit     = bit_sum - (bit_sum / 2) * 2;  //compute current bit
            let next    = bit_sum / 2;  //carry to next position
            let mut result = vec![bit];
            result.extend(do_binary_addition(ar, br, next));
            result
        }
    }
}

pub fn binary_addition(a: &[i32], b: &[i32]) -> Vec<i32> {
    //reverse to lsb-first, add, then reverse back
    let ra = reverse_list(a);
    let rb = reverse_list(b);
    let raw = do_binary_addition(&ra, &rb, 0);
    reverse_list(&raw)
}

fn flip_bits(list: &[i32]) -> Vec<i32> {
    match list {
        []          => vec![],
        [h, rest @ ..] => {
            let flipped = if *h == 0 { 1 } else { 0 };  //invert each bit
            let mut result = vec![flipped];
            result.extend(flip_bits(rest));
            result
        }
    }
}

fn pad_list(list: &[i32], len: usize) -> Vec<i32> {
    if list.len() >= len {
        return list.to_vec();
    }
    //prepend leading zeros to match target length
    let mut result = vec![0; len - list.len()];
    result.extend_from_slice(list);
    result
}

fn twos_complement(list: &[i32]) -> Vec<i32> {
    let flipped = flip_bits(list);  //invert all bits
    binary_addition(&flipped, &[1])  //add one to get two's complement
}

fn drop_leading_bit(list: &[i32], max_len: usize) -> Vec<i32> {
    if list.len() <= max_len {
        list.to_vec()
    } else {
        //keep only the rightmost max_len bits
        list[list.len() - max_len..].to_vec()
    }
}

pub fn binary_subtraction(a: &[i32], b: &[i32]) -> Vec<i32> {
    let max_len = a.len().max(b.len());
    let pa = pad_list(a, max_len);  //align to same length
    let pb = pad_list(b, max_len);
    //subtract via two's complement addition
    let tc  = twos_complement(&pb);
    let raw = binary_addition(&pa, &tc);
    drop_leading_bit(&raw, max_len)
}