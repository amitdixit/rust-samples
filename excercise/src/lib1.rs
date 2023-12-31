pub fn sample_1() {
    // println!("Sample 1");

    //  println!("{}", is_palindrome_string("malayalam1"));

    // println!("{}", reverse_string1("amit"));
    println!("{}", is_valid_anagram("listen", "Silent"));
}

fn is_palindrome_string(str: &str) -> bool {
    let rev = reverse_string(str);
    rev == str
}

fn reverse_string(str: &str) -> String {
    let mut rev = String::new();

    for chr in str.chars().rev() {
        rev.push(chr);
    }

    return rev;
}

fn reverse_string1(str: &str) -> String {
    let mut rev = String::new();
    let str_array: Vec<char> = str.chars().collect();

    println!("{}", str_array[2]);

    for index in (0..str_array.len()).rev() {
        rev.push(str_array[index]);
    }

    return rev;
}

fn is_valid_anagram(str1: &str, str2: &str) -> bool {
    let mut str1_array: Vec<char> = str1.to_lowercase().chars().collect();
    let mut str2_array: Vec<char> = str2.to_lowercase().chars().collect();

    str1_array.sort();
    str2_array.sort();

    str1_array == str2_array
}
