pub fn run() {
    println!("\n*****************************************************************");
    println!("functional");
    println!("*****************************************************************");

    closures_tests();
}

fn make_closure() -> impl (Fn(u8) -> u8) {
    let add_five = |x| x + 5;
    add_five
}

fn make_closure2<'a>() -> impl (FnOnce(&'a str) -> String) {
    let s = String::from("hello ");
    let add_hello = move |s2| s + s2;
    add_hello
}

fn closures_tests() {
    println!("\n--- closures ---\n");

    let f = make_closure();
    println!(" f(1): {:?}", f(1));

    let f2 = make_closure2();
    println!(" f2(\"world\"): {:?}", f2("world"));
}
