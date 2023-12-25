use traits_sample_oops::{Button, Draw, Human, Pilot, Screen, Wizard};
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 10,
                height: 5,
                label: String::from("Submit"),
            }),
        ],
    };

    screen.run();

    let human = Human;
    Pilot::fly(&human);
    Wizard::fly(&human);
    human.fly();

    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);

    let answer = do_twice1(add_one, 15);
    println!("The answer is {}", answer);
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox  Drawn");
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice1<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn returns_closure1() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_closure2(a: i32) -> Box<dyn Fn(i32) -> i32> {
    //  Box::new(|x| x + 1)
    if a > 0 {
        Box::new(move |x| a + x)
    } else {
        Box::new(move |x| a - x)
    }
}
