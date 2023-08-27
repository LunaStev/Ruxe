fn greet(name: &str) {
    println!("안녕하세요. {}", name);
}

fn main() {
    let mut a = 1;
    a = 2;

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    greet("Hello\n");
    println!("{}", a);
}