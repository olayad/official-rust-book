use std::io;

fn main() {
    get_nth_fib()
}

fn get_nth_fib() {
    let target: u32 = fib_input();
    nth_fibonacci(0, 1, target, 1);
}

fn fib_input() -> u32 {
    println!("Enter the fib number:");
    let mut input :String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read unit");
    return input.trim().parse().unwrap();
}

fn nth_fibonacci(f0:u32, f1:u32, target: u32, count: u32) -> u32 {
    println!("target:{}, count:{} f0:{}, f1:{}", target, count, f0, f1);
    if target == 0 {
        return f0
    };
    if target == 1 {
        return f1
    };
    while count != target {
        nth_fibonacci(f1, f1 + f0, target, count + 1);
        println!("Resulting nth fib:{} for {}", f1+f0, target);
        break
    }
    return f1 + f0
}
