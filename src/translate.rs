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
        println!("Translation: (no recognizable numbers found)");
    } else {
        let translated = nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Translation: {}", translated);
    }
}