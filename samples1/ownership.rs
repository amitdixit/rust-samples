fn main() {
    let fruit = String::from("Apple"); // fruit comes into scope

    // ownership of fruit moves into the function
    print_fruit(fruit);

    // fruit is moved to the function so is no longer available here
    // error
    // println!("fruit = {}", fruit);
}

fn print_fruit(str: String) {
    // str comes into scope
    println!("str = {}", str);
} // str goes out of scope and is dropped, plus memory is freed
