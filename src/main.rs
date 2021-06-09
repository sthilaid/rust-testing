mod chapters {
    pub mod collections;
    pub mod functional;
    pub mod generics;
    pub mod smart_pointers;
}

fn main() {
    println!(" --- running tests ---");
    chapters::collections::run();
    chapters::generics::run();
    chapters::functional::run();
    chapters::smart_pointers::run();
}
