fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s1);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yaay");

    some_string
}

fn takes_and_gives_back(mut a_string: String) -> String {
    a_string.push_str(", world!");

    a_string
}
