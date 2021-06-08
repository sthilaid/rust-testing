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

    let v3 = v.iter().map(|x| x * x);
    println!("v3: {:?}", v3);
}
