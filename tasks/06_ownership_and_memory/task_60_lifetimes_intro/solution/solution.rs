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

    let result = longest(sentence.as_str(), word);

    println!("Longest text: '{}'", result);

    let banner: &'static str = "SAFE SYSTEMS WITH ZERO DANGLING POINTERS";
    println!("Security Banner: {} ('static)", banner);

    println!("================================");
}

