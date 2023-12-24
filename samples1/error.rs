use std::fs::File;

fn main() {
    let data_result = File::open("data.txt");
    let data = match data_result {
        Ok(data) => data,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data: {:?}", data);
}
