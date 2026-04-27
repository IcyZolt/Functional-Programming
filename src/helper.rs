fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn find_pair(n: u64, p: u64) {
    if p >= n {
        return;
    } else if is_prime(p) && is_prime(n - p) {
        println!("{} + {} = {}", p, n - p, n);
    } else {
        find_pair(n, p + 1);
    }
}


