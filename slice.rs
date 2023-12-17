fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];

    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);

    let mut numbers1 = [1, 2, 3, 4, 5];
    let slice1 = &mut numbers1[1..4];

    println!("slice = {}", slice1[1]);
    slice1[1] = 100;
    println!("slice = {}", slice1[1]);

    let tuple = ("Hello", 3, 4.65);

    println!("{:?}", tuple);

    let tuple1: (&str, f32, u8) = ("Amit", 3.14, 100);

    println!("Tuple contents = {:?}", tuple1);
    println!("Value at Index 0 = {}", tuple1.0);
    println!("Value at Index 1 = {}", tuple1.1);
    println!("Value at Index 2 = {}", tuple1.2);

    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_heights);
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;
    println!("Changed tuple = {:?}", mountain_heights);

    let (message, float, mut number) = tuple1;

    println!("message = {}", message);
    println!("number = {}", number);
    println!("float = {}", float);

    number = 5;

    println!("number = {}", number);
}
