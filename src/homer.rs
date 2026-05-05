//generate all possible orderings of a list
fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    //pick each item as the first element
    for i in 0..items.len() {
        let mut remaining: Vec<T> = Vec::new();
        //collect everything except the picked item
        for j in 0..items.len() {
            if j != i {
                remaining.push(items[j].clone());
            }
        }
        //recursively permute the rest, then prepend the picked item
        for perm in permutations(&remaining) {
            let mut full = vec![items[i].clone()];
            full.extend(perm);
            result.push(full); //add this completed permutation to the result set
        }
    }
    result
}

fn main() {
    //list of four stores and four gifts
    let stores = vec!["The Leftorium", "Sprawl-Mart", "Try-N-Save", "King Toots"];
    let gifts = vec!["green dress", "saxophone book", "slingshot", "pacifier"];

    //try every possible ordering of stores
    for store_perm in permutations(&stores) {
        //constraint 3: the leftorium was the 2nd stop
        if store_perm[1] != "The Leftorium" {
            continue;
        }

        //for each store order, try every possible gift order
        for gift_perm in permutations(&gifts) {
            //constraint 1: saxophone book was bought at king toots
            let king_toots_pos = store_perm
                .iter()
                .position(|&s| s == "King Toots")
                .unwrap(); //find which stop has king toots
            if gift_perm[king_toots_pos] != "saxophone book" {
                continue;
            }

            //constraint 2: store after buying the slingshot was not sprawl-mart
            let sling_pos = gift_perm.iter().position(|&g| g == "slingshot").unwrap();
            if sling_pos < 3 {
                let next_store = store_perm[sling_pos + 1];
                if next_store == "Sprawl-Mart" {
                    continue;
                }
            }

            //constraint 4: two stops after try-n-save, he bought the pacifier
            let try_n_save_pos = store_perm
                .iter()
                .position(|&s| s == "Try-N-Save")
                .unwrap();
            if try_n_save_pos + 2 >= 4 {
                continue; //try-n-save was too late for this constraint
            }
            let pacifier_pos = gift_perm.iter().position(|&g| g == "pacifier").unwrap();
            if pacifier_pos != try_n_save_pos + 2 {
                continue;
            }

            //all constraints satisfied, print the solution
            println!("Solution found!\n");
            for i in 0..4 {
                println!("Stop {}: {} - {}", i + 1, store_perm[i], gift_perm[i]);
            }
            return;
        }
    }
    println!("No solution found.");
}
