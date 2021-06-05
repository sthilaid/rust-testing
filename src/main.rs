mod chapters {
    pub mod collections;
    pub mod generics;
}

fn main() {
    println!(" --- running tests ---");
    chapters::collections::run();
    chapters::generics::run();
}
