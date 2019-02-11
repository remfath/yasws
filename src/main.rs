fn main() {
    let novel = String::from("Hi there. I am Remfath Chan");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("{}", first_sentence);
}