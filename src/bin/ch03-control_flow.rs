use std::io;

fn main() {
    println!("hi");
    temp_converter();
}

fn temp_converter() -> i32 {
    println!("enter a temp unit to convert 'f' or 'c':");
    let mut unit = string::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("failed to read unit");

    if unit != 'f'.to_string() || unit != 'c'.to_string() {
        println!("error: wrong unit provided");
        return -1;
    }
    println!("unit:{}", unit);
    if unit == 'f'.to_string() {
        println!("converting f to c...");
    } else {
        println!("converting c to f...");
    }
    return 0

}