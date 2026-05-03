fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    for i in 0..items.len() {
        let mut remaining: Vec<T> = Vec::new();
        for j in 0..items.len() {
            if j != i {
                remaining.push(items[j].clone());
            }
        }
        for perm in permutations(&remaining) {
            let mut full = vec![items[i].clone()];
            full.extend(perm);
            result.push(full);
        }
    }
    result
}

fn main() {
    let stores = vec!["The Leftorium", "Sprawl-Mart", "Try-N-Save", "King Toots"];
    let gifts = vec!["green dress", "saxophone book", "slingshot", "pacifier"];

    for store_perm in permutations(&stores) {
        if store_perm[1] != "The Leftorium" {
            continue;
        }

        for gift_perm in permutations(&gifts) {
            let king_toots_pos = store_perm
                .iter()
                .position(|&s| s == "King Toots")
                .unwrap();
            if gift_perm[king_toots_pos] != "saxophone book" {
                continue;
            }

            let sling_pos = gift_perm.iter().position(|&g| g == "slingshot").unwrap();
            if sling_pos < 3 {
                let next_store = store_perm[sling_pos + 1];
                if next_store == "Sprawl-Mart" {
                    continue;
                }
            }

            let try_n_save_pos = store_perm
                .iter()
                .position(|&s| s == "Try-N-Save")
                .unwrap();
            if try_n_save_pos + 2 >= 4 {
                continue;
            }
            let pacifier_pos = gift_perm.iter().position(|&g| g == "pacifier").unwrap();
            if pacifier_pos != try_n_save_pos + 2 {
                continue;
            }

            println!("Solution found!\n");
            for i in 0..4 {
                println!("Stop {}: {} - {}", i + 1, store_perm[i], gift_perm[i]);
            }
            return;
        }
    }
    println!("No solution found.");
}
