#[derive(Debug)]
struct ImportantException<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Hi there. I am Remfath Chan");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantException {
        part: first_sentence
    };
    println!("{:?}", i);
}