pub fn run() {
    println!("\n*****************************************************************");
    println!("generics");
    println!("*****************************************************************");
    vector_tests();
    string_tests();
}

#[derive(Debug)]
struct TestData {
    s: String,
    i: u32,
}

#[derive(Debug)]
enum TestEnum {
    Int(i32),
    Bool(bool),
    Data(TestData),
}

fn vector_tests() {
    println!("\n--- vector tests ---\n");
    let mut stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack: {:?}", stack);

    let v = vec!['a', 'b', 'c'];
    println!("v: {:?}", v);

    let mut msg: String = "messag".to_string();
    msg.push('e');
    let v: Vec<&str> = vec![&msg[..], "cccc"];
    println!("v: {:?}", v);
    println!("s: {:?}", msg);

    let mut s: String = "1111".to_string();
    s.push('2');
    let mut v: Vec<String> = vec![s];
    v[0] = v[0].replace("1", "2");
    println!("v: {:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];
    let i = &mut v[0];
    *i = 11;
    println!("v: {:?}", v);
    println!("v.get(0): {:?}", v.get(0));
    println!("v.get(1000): {:?}", v.get(1000));

    for i in &mut v {
        *i = *i * *i;
    }
    for i in v {
        println!("i: {}", i);
    }

    let v: Vec<TestEnum> = vec![
        TestEnum::Int(10),
        TestEnum::Bool(true),
        TestEnum::Data(TestData {
            s: msg.clone(),
            i: 22,
        }),
    ];
    println!("v: {:?}", v);
}

fn string_tests() {
    println!("\n--- string tests ---\n");
    let s1: String = "hello".to_string();
    let s2: &str = "world";
    let hw: String = s1 + &s2;
    println!("{}", hw);

    let mut s1: String = String::from("abcd");
    s1.push_str("ef");
    println!("{}", s1);
    println!("{}", &s1[1..4]);

    let s = "The quick brown fox jumps over the lazy dog";
    for c in s[..6].chars() {
        println!("{}", c);
    }
    for c in s[..6].bytes() {
        println!("{}", c);
    }
}
