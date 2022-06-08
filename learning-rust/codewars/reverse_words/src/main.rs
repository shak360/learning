fn reverse_words(str: &str) -> String {
    let mut total_collector = Vec::new();
    let mut mini_collector = Vec::new();
    for (i, c) in str.chars().enumerate() {
        if c.is_whitespace() {
            if !mini_collector.is_empty() {
                mini_collector.reverse();
                total_collector.append(&mut mini_collector);
                mini_collector.truncate(0);
            }
            total_collector.push(c);
        } else {
            mini_collector.push(c);
        }

        if i == str.len() - 1 {
            mini_collector.reverse();
            total_collector.append(&mut mini_collector);
            mini_collector.truncate(0)
        }
    }
    total_collector.into_iter().collect::<String>()
}

fn main() {
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
