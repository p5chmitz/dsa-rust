// Testing out Rust's stack operations
pub fn stack_0() {
    let mut v = vec![11, 22, 33, 44];
    println!("Original: {v:?}");
    let pop = v.pop().expect("There was none left");
    println!("Popped {pop} which leaves {v:?}");
    let push = 55;
    v.push(push);
    println!("Pushed {push} which leaves {v:?}");
    let last = v.last();
    println!(
        "The last (top) element is {} which still leaves {:?}",
        last.expect("Aint nothin there"),
        v
    );
    println!("The size of the stack is {}", v.len());
    if v.is_empty() {
        println!("Its empty :(")
    } else {
        println!("Theres still {} elements in the stack!", v.len())
    }
}
