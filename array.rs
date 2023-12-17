fn main() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array of numbers = {:?}", arr1);

    let arr2 = [1, 2, 3, 4, 5];

    println!("array of numbers = {:?}", arr2);

    let arr3 = [1; 5];
    println!("array of numbers = {:?}", arr3);

    let colors = ["red", "green", "blue"];

    println!("First item = {}", colors[0]);

    let mut colors1 = ["red", "green", "blue"];

    println!("First item = {}", colors1[0]);

    colors1[0] = "yellow";

    println!("First item = {}", colors1[0]);

    for index in 0..colors.len() {
        println!("Index: {} -- Value: {}", index, colors[index]);
    }
}
