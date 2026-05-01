fn translate(word: &str, chinese: &[&str], english: &[&str]) -> Option<u64> {
    for (i, &w) in chinese.iter().enumerate() { //look for 
        if w == word {
            return Some(i as u64);
        }
    }
    for (i, &w) in english.iter().enumerate() {
        if w == word {
            return Some(i as u64);
        }
    }
    None
}

fn translate_list<'a>(input: &[&'a str], chinese: &[&str], english: &[&str]) -> Vec<u64> {
    input.iter()
        .filter_map(|w| translate(w, chinese, english))
        .collect()
}

fn print_translation(nums: &[u64]) {
    if nums.is_empty() {
        println!("no recognizable numbers found for translation");
    } else {
        let translated = nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Translation: {}", translated);
    }
}

fn add_list(nums: &[u64]) -> u64 {
    match nums {
        []                => 0,
        [head, tail @ ..] => head + add_list(tail),
    }
}

fn multiply_list(nums: &[u64]) -> u64 {
    match nums {
        []                => 1,
        [head, tail @ ..] => head * multiply_list(tail),
    }
}

fn print_addition(nums: &[u64]) {
    let expr = nums.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" + ");
    println!("Addition: {} = {}", expr, add_list(nums));
}

fn print_multiplication(nums: &[u64]) {
    let expr = nums.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" * ");
    println!("Multiplication: {} = {}", expr, multiply_list(nums));
}

fn go(input: &[&str], chinese: &[&str], english: &[&str]) {
    let nums = translate_list(input, chinese, english);
    print_translation(&nums);

    if nums.len() > 1 {
        print_addition(&nums);
        print_multiplication(&nums);
    }
}