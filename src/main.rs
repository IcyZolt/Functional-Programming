mod helper;
mod binary;

fn hitchhiker(list: &[i32]) -> bool {
    list.contains(&42)
}
fn everyother(list: &[i32]) -> Vec<i32> {
    if list.len() < 2 {
        return vec![];
    }
    let mut result = vec![list[1]];
    result.extend(everyother(&list[2..]));
    result
}

fn goldbach(n: i64) {
    if n <= 2 {
        println!("Err: integer must be greater than 2");
    } else if n % 2 != 0 {
        println!("Err: integer must be even");
    } else {
        crate::helper::find_pair(n as u64, 2);
    }
}

fn main() {
    println!("{}", hitchhiker(&[]));
    println!("{}", hitchhiker(&[1, 2, 3]));    
    println!("{}", hitchhiker(&[7, 42, 99]));  

    println!("{:?}", everyother(&[]));
    println!("{:?}", everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]));

    goldbach(28);
    goldbach(6);
    goldbach(100);
    goldbach(3);
    goldbach(2);
    goldbach(-4);

    println!("{:?}", binary::binary_addition(&[1,0,1,0], &[1]));
    println!("{:?}", binary::binary_addition(&[1], &[1,0,1,0]));
    println!("{:?}", binary::binary_addition(&[1,1,1], &[1,1]));
    println!("{:?}", binary::binary_addition(&[1,1,1,1], &[1]));
    
    println!("{:?}", binary::binary_subtraction(&[1,0,1,0], &[1]));
    println!("{:?}", binary::binary_subtraction(&[1,1,1], &[1,1]));
    println!("{:?}", binary::binary_subtraction(&[1,0,0,0], &[1]));

}
