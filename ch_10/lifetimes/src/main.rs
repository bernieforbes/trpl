fn main() {
    let bar = foo("hello", "world");

    println!("{}", bar);
}

fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x == "hello" {
        return x;
    } else {
        return y;
    }
}
