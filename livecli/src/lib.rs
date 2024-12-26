// convert to chars then rev it and collect it (as it is iter)
// collect it in string type
// simple explaination: reverse a string while preserving UTF-8 correctness.
pub fn reverse_string(str: &str) -> String {
    str.chars().rev().collect::<String>()
}