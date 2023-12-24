fn main() {
    let v = vec![1, 2, 3];
    let v1: Vec<u8> = vec![1, 2, 3];

    println!("v= {:?}", v);

    println!("v1= {:?}", v1);

    let mut colors = vec!["blue", "red", "green"];

    println!("original vector = {:?}", colors);

    colors.push("black");
    colors.push("white");

    println!("changed vector = {:?}", colors);

    let mut v2: Vec<i32> = Vec::new();

    v2.push(10);
    v2.push(20);

    println!("v2 = {:?}", v2);

    let mut word = String::new();

    println!("original string = {}", word);

    word.push_str("Hello, World!");

    println!("changed string = {}", word);

    word = "ABCD".to_string();
    println!("changed string = {}", word);

    word.push_str("Hello, DE!");

    println!("changed string = {}", word);
}
