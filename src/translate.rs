//look up a word in both the chinese and english lists, return its index if found
fn translate(word: &str, chinese: &[&str], english: &[&str]) -> Option<u64> {
    for (i, &w) in chinese.iter().enumerate() {
        if w == word {
            return Some(i as u64);
        }
    }
    //not found in chinese so try english list
    for (i, &w) in english.iter().enumerate() {
        if w == word {
            return Some(i as u64);
        }
    }
    None //word not recognized in either language
}

//filter out unrecognized words and translate the rest to numbers
fn translate_list<'a>(input: &[&'a str], chinese: &[&str], english: &[&str]) -> Vec<u64> {
    input.iter()
        .filter_map(|w| translate(w, chinese, english))
        .collect() //collect only successful translations
}

//print the translated numbers as a space-separated string
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

//recursively sum all numbers in a list
fn add_list(nums: &[u64]) -> u64 {
    match nums {
        []                => 0,
        [head, tail @ ..] => head + add_list(tail), //add head then recurse on tail
    }
}

//recursively multiply all numbers in a list
fn multiply_list(nums: &[u64]) -> u64 {
    match nums {
        []                => 1,
        [head, tail @ ..] => head * multiply_list(tail),
    }
}

//print the addition expression and its result
fn print_addition(nums: &[u64]) {
    let expr = nums.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" + ");
    println!("Addition: {} = {}", expr, add_list(nums));
}

//print the multiplication expression and its result
fn print_multiplication(nums: &[u64]) {
    let expr = nums.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" * ");
    println!("Multiplication: {} = {}", expr, multiply_list(nums));
}

//main entry point: translate, then print translation/add/multiply
fn go(input: &[&str], chinese: &[&str], english: &[&str]) {
    let nums = translate_list(input, chinese, english);

    if nums.len() > 1 {
        print_addition(&nums); //only print math if we have multiple numbers
        print_multiplication(&nums);
    }
}
