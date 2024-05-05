fn main() {
    let mut s = String::from("hello");

    let r1: &mut String = &mut s;
    r1.push_str("fff");
    
    println!("{}, {}", r1, r1);
    let r2: &mut String = &mut s;
    println!("{}, {}", r2, r2);

}