fn main() {
    let x = 5;

    if x == 5 {
        println!("x is 5");
    } else {
        println!("x is not 5");
    }

    let mut number: i32 = 0;
    loop {
        println!("Current Counter {}", number);
        number += 1;

        if number >= 10 {
            break;
        }
    }

    number = 0;

    while number < 10 {
        println!("Current Counter usimg WHile {}", number);
        number += 1;
    }

    for i in 0..10 {
        println!("Current Counter using For {}", i);
    }

    for i in (0..10).step_by(2) {
        println!("Current Counter using For {}", i);
    }

    for i in (0..10).rev().step_by(1) {
        println!("Current Counter using For {}", i);
    }
}
