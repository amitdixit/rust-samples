fn main(){

    let int :i32=56;
    let sint:i8=-56;
    let uint :u8=56;
    let long :i64=56;
    let ulong :u8=56;
    let short :u8=56;
    let float :f32=56.7;
    let char :char='A';
    let bool :bool=true;
   

    println!("{}",int);
    println!("{}",sint);
    println!("{}",uint);
    println!("{}",long);
    println!("{}",ulong);
    println!("{}",short);
    println!("{}",float);
    println!("{}",char);
    println!("{}",bool);   


    let decimal :f32 = 97.65;
    let integer :i32 = decimal as i32;

    println!("decimal = {} integer = {}", decimal,integer);

    let character: char = 'A';
    let integer = char as u8;

    println!("character = {} integer = {}", character,integer);


    let integer: u8 = 65;  
    let character = integer as char;

    println!("integer = {} character = {}" , integer, character);
 
}