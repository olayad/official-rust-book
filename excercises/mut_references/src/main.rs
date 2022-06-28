fn main() {
    let mut a = String::from("hello");
    let b = &mut a;
    *b = String::from("another string");
    a = String::from("world");
    println!("{}", b);
    println!("{}", a);
}