fn main() {
    let mut number = 3;
    'outer_loop: loop{
        if number == 0 {
            println!("LIFTOFF!!");
            break 'outer_loop;
        } else {
            println!("{}!", number);
            number -= 1;
        }

    }
}

