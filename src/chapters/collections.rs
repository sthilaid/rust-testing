pub fn run() {
    println!("** collections **");
    vector_tests();
}

pub fn vector_tests() {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack: {:?}", stack);
}
