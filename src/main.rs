mod chapters {
    pub mod collections;
    pub mod functional;
    pub mod generics;
}

fn main() {
    println!(" --- running tests ---");
    chapters::collections::run();
    chapters::generics::run();
    chapters::functional::run();
}
