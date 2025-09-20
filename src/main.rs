const TEST_PASSAGES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "Never underestimate the power of a good book and a warm cup of coffee.",
    "Programming in Rust can be challenging yet incredibly rewarding for many.",
    "Learning by doing is an effective way to master new skills and concepts.",
    "A journey of a thousand miles begins with a single step towards the horizon.",
];

fn main() {
    for passage in TEST_PASSAGES {
        println!("{}", passage);
    }
}