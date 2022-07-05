use std::io;

fn main() {
    temp_converter();
}

fn temp_converter() -> i32 {
    println!("Enter a unit to convert 'f' or 'c':");
    let mut unit_input = String::new();
    io::stdin()
        .read_line(&mut unit_input)
        .expect("Failed to read unit");
    let unit_input = unit_input.trim();

    println!("Enter a number(temperature) to convert:");
    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read temperature");
    let temp_input: f32 = temp_input.trim().parse().unwrap();

    println!("INPUT- unit:{}, temp:{}", unit_input, temp_input);
    if unit_input.trim() == "f" {
        println!("Converting F to C...");
        let temp: f32 = (temp_input - 32.0) * (5.0/9.0);
        println!("{} F is {} C", temp_input, temp);
    } else if unit_input.trim() == "c" {
        println!("Converting C to F...");
        let temp: f32 = (temp_input * 9.0/5.0) + 32.0;
        println!("{} C is {} F", temp_input, temp);
    } else {
        println!("Wrong unit provided");
        return -1
    };
    return 0

}