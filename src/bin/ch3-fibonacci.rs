use std::io;

fn main() {
    get_nth_fib()
}

fn fib_input() -> u32 {
    println!("Enter the fib number:");
    let mut input :String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read unit");
    return input.trim().parse().unwrap();
}

fn get_nth_fib() {
    let target: u32 = fib_input();
    //nth_fibonacci_dom(0, 1, target, 1);
    nth_fibonacci(0, 1, target);
}

// first DOM implementation
fn nth_fibonacci_dom(second_last:u32, last:u32, target: u32, count: u32) -> u32 {
    println!("target:{}, count:{} f0:{}, f1:{}", target, count, second_last, last);
    if target == 0 {
        return second_last
    };
    if target == 1 {
        return last
    };
    while count != target {
        nth_fibonacci_dom(last, last + second_last, target, count + 1);
        break
    }
    println!("Resulting nth fib:{} for {}", last +second_last, target);
    return last + second_last
}

fn nth_fibonacci(mut second_last:u32, mut last:u32, target:u32) -> u32 {
    if target <= 1 {
        println!("Fib number is: {}", last);
        return last
    } else {
        let new_last: u32 = second_last + last;
        let _:u32 = nth_fibonacci(last, new_last, target-1);
    }
    return 0
}