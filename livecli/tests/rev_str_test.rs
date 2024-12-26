use std::collections::HashMap;
use livecli::reverse_string;

#[test]
fn reverse_string_test() {
    let strings = HashMap::from([
        ("test", "tset"),
        ("madam", "madam"),
        ("test1", "1tset"),
        ("bittu", "uttib"),
        ("dev", "ved"),
    ]);

    for (key, value) in strings {
        assert_eq!(reverse_string(key), value);
    }
}