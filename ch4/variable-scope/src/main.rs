fn main() {
    let mut s = String::from("hello");
    let s2 = s.clone();

    s.push_str(", world!");

    println!("{}", s);
}
