fn main() {
    let emp = Employee {
        name: String::from("Amit"),
        age: 25,
        salary: 555.36,
        department: String::from("IT"),
        is_permanent: true,
    };

    println!("{:?}", emp);

    println!("Employee age = {}", emp.age);

    let Employee {
        name, age, salary, ..
    } = emp;

    println!("Employee name = {}", name);
    println!("Employee age = {}", age);
    println!("Employee salary = {}", salary);

    // println!("{:?}", emp);

    greet_employee(name);

    add(8, 11);
    println!("Sum is {}", get_sum(8, 11));

    let (sum, diff) = addsub(4, 1);
    println!("Sum = {}, Difference = {}", sum, diff);

    closure_example();
}

fn greet_employee(name: String) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of a and b = {}", sum);
}

fn addsub(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a - b);
}

fn get_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn closure_example() {
    // closure that prints a text
    let print_text = || println!("Hello, World!");

    print_text();

    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;

    // call closure and store the result in a variable
    let result = add_one(2);

    println!("Result = {}", result);

    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {
        let mut sum: i32 = x + y;
        let mut result: i32 = sum * sum;
        return result;
    };

    println!("Result = {}", squared_sum(5, 3));
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
    salary: f32,
    department: String,
    is_permanent: bool,
}
