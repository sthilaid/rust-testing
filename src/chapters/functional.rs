pub fn run() {
    println!("\n*****************************************************************");
    println!("functional");
    println!("*****************************************************************");

    closures_tests();
    iterator_tests();
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

fn foldl<T: Copy>(array: &[T], base: &T, fun: impl (Fn(&T, &T) -> T)) -> T {
    let mut acc: T = *base;
    for x in array {
        acc = fun(x, &acc);
    }
    acc
}

fn make_incrementer<'a>(val: &'a mut u8) -> impl FnMut(&'a u8) + 'a {
    move |n: &'a u8| {
        *val = *val + n;
    }
}

fn closures_tests() {
    println!("\n--- closures ---\n");

    let f = make_closure();
    println!(" f(1): {:?}", f(1));

    let f2 = make_closure2();
    println!(" f2(\"world\"): {:?}", f2("world"));

    let a = [1, 2, 3, 4, 5];
    println!("foldl(a, sum): {:?}", foldl(&a, &0, |x, a| x + a));
    println!("foldl(a, mult): {:?}", foldl(&a, &1, |x, a| x * a));

    let mut x = 0;
    {
        let mut incrementer = make_incrementer(&mut x);
        incrementer(&10);
        incrementer(&10);
        incrementer(&10);
    }
    println!("incremented x: {:?}", x);
}

struct Counter {
    x: u8,
    max: u8,
}

impl Counter {
    fn new(max: u8) -> Counter {
        Counter { x: 0, max: max }
    }
}

impl Iterator for Counter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.max {
            self.x += 1;
            Some(self.x)
        } else {
            None
        }
    }
}

struct PrimeIter {
    x: u32,
}

impl PrimeIter {
    fn new() -> PrimeIter {
        PrimeIter { x: 0 }
    }
    fn is_prime(x: u32) -> bool {
        let mut base = x / 2;
        while base > 1 {
            if x % base == 0 {
                break;
            }
            base -= 1
        }

        base == 1
    }
}

impl Iterator for PrimeIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        while !PrimeIter::is_prime(self.x) {
            self.x += 1;
        }
        Some(self.x)
    }
}

fn iterator_tests() {
    println!("\n--- iterators ---\n");

    let v = vec![1, 2, 3, 4, 5];
    let mut v2 = Vec::new();
    for x in v.iter() {
        v2.push(x);
    }
    println!("v: {:?}", v);
    println!("v2: {:?}", v2);

    let mut it = v.iter();
    let mut x = it.next();
    loop {
        match x {
            Some(v) => v2.push(v),
            None => break,
        };
        x = it.next();
    }
    println!("v2: {:?}", v2);

    let squares = v.iter().map(|x| x * x).filter(|x| x % 2 == 0).rev();
    let squares_vect: Vec<u8> = squares.clone().collect();
    let squares_list: std::collections::LinkedList<u8> = squares.collect();
    println!("vect: {:?} list: {:?}", squares_vect, squares_list);

    let more_values = [1, 2, 3, 4, 5, 6, 7, 8];
    let sum_more_values = more_values.iter().fold(0, |x, a| x + a);
    let sum_more_values2: u8 = more_values.iter().sum();
    println!("sums: {:?}, {:?}", sum_more_values, sum_more_values2);

    let strs = ["aaaaa", "bbbbb", "ccccc", "ddddd"];

    // into_iter for iterator taking ownership
    let heap_strs = strs.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let mut bucket: Vec<String> = Vec::new();
    heap_strs.into_iter().for_each(|s| bucket.push(s));
    println!("bucket: {:?}", bucket);

    // iter_mut for muting iterator
    let mut heap_strs = strs.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    heap_strs.iter_mut().for_each(|s| s.make_ascii_uppercase());
    println!("heap_strs: {:?}", heap_strs);

    let to_five: Vec<u8> = Counter::new(5).collect();
    println!("to_five: {:?}", to_five);

    let to_ten: Vec<u8> = Counter::new(10).collect();
    println!("to_five: {:?}", to_ten);

    let five_primes: Vec<u32> = PrimeIter::new().take(10).collect();
    println!("five_primes: {:?}", five_primes);
}
