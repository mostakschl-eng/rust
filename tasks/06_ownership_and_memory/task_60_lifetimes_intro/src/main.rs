// TODO 1: Implement longest<'a>(x: &'a str, y: &'a str) -> &'a str
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("=== LIFETIME COMPARISON GATE ===");

    let sentence = String::from("The Rust Programming Language");
    let word = "Rust";

    // TODO 2: Call longest with sentence.as_str() and word
    let result = longest(sentence.as_str(), word);

    println!("Longest text: '{}'", result);

    // TODO 3: Declare 'static banner
    let banner: &'static str = "SAFE SYSTEMS WITH ZERO DANGLING POINTERS";
    println!("Security Banner: {} ('static)", banner);

    println!("================================");
}

