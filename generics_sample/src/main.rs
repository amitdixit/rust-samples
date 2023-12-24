use std::char;

fn main() {
    let item_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let largest = get_largest(item_list);

    println!("The largest item in the list is: {}", largest);

    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    let largest = get_largest(char_list);
    println!("The largest char in the list is: {}", largest);

    let point11 = Point1 { x: 1, y: 2 };
    let point12 = Point1 { x: 1.5, y: 2.6 };
    let point21 = PointNew { x: 1, y: 2.0 };

    point11.x();
    point12.x();
    point12.y();

    let p22 = PointNew { x: "Hello", y: 'a' };

    let p3 = point21.mixup(p22);

    println!("{:?}", p3);
}

fn get_largest<T: PartialOrd + Copy>(item_list: Vec<T>) -> T {
    let mut largest = item_list[0];
    for item in item_list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point1<T> {
    x: T,
    y: T,
}

impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point1<f32> {
    fn y(&self) -> f32 {
        self.y
    }
}

#[derive(Debug)]
struct PointNew<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointNew<T, U> {
    fn mixup<V, W>(self, other: PointNew<V, W>) -> PointNew<T, W> {
        PointNew {
            x: self.x,
            y: other.y,
        }
    }
}
