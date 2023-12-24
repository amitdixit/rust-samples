fn main() {
    let str = String::from("Hello, World!");

    // Call function with reference String value
    let len = calculate_length(&str);

    println!("The length of '{}' is {}.", str, len);

    let mut str1 = String::from("Hello");

    // before modifying the string
    println!("Before: str = {}", str1);

    // pass a mutable string when calling the function
    change(&mut str1);

    // after modifying the string
    println!("After: str = {}", str1);
}

// Function to calculate length of a string
// It takes a reference of a String as an argument
fn calculate_length(s: &String) -> usize {
    println!("Original {}", s);
    s.len()
}

fn change(s: &mut String) {
    // push a string to the mutable reference variable
    s.push_str(", World!");
}
