use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // hashmap_example();

    //   hashset_example();

    iterator_example();
}

fn hashmap_example() {
    let mut info: HashMap<i32, String> = HashMap::new();

    info.insert(1, "A".to_string());
    info.insert(2, String::from("Banana"));

    println!("HashMap = {:?}", info);

    println!("Value of key 1 = {:?}", info.get(&1));

    info.insert(1, String::from("Mango"));
    println!("HashMap = {:?}", info);

    println!("HashMap = {:?}", info.keys());

    println!("HashMap = {:?}", info.values());

    if info.contains_key(&3) {
        println!("HashMap contains key 1");
    } else {
        info.insert(3, String::from("Cherry"));
    }

    println!("HashMap = {:?}", info);

    for i in info.values() {
        println!("{}", i)
    }
}

fn hashset_example() {
    let numbers = HashSet::from([2, 7, 8, 10]);
    let mut info: HashSet<i32> = HashSet::new();

    info.insert(1);
    info.insert(2);
    info.insert(3);

    println!("HashSet = {:?}", info);

    if info.contains(&1) {
        println!("HashSet contains key 1");
    }

    info.remove(&1);
    println!("HashSet = {:?}", info);

    let mut colors: HashSet<&str> = HashSet::new();

    // insert values in a HashSet
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("colors = {:?}", colors);

    let mut emps: HashSet<Employee> = HashSet::new();

    emps.insert(Employee {
        name: String::from("John"),
        age: 30,
        salary: 5000,
    });

    println!("emps = {:?}", emps);

    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);

    // Union of hashsets
    let result: HashSet<_> = hashset1.union(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);

    // Intersection of hashsets
    let result: HashSet<_> = hashset1.intersection(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("intersection = {:?}", result);

    let hashset1 = HashSet::from([1, 2, 3, 4]);
    let hashset2 = HashSet::from([4, 3, 2]);

    // Difference between hashsets
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();

    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result);
}

fn iterator_example() {
    let numbers = [2, 1, 17, 99, 34, 56];

    // iterator
    let numbers_iterator = numbers.iter();

    for number in numbers_iterator {
        println!("{}", number);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Employee {
    name: String,
    age: u8,
    salary: i32,
}
