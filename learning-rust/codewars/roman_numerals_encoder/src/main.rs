/// Converts a number to a string representating roman numeral.
fn num_as_roman(num: i32) -> String {
    const ROMANS: [(&str, &str, &str); 4] = [
        ("I", "V", "X"), // when dealing with 0..9
        ("X", "L", "C"), // when dealing with 10..99
        ("C", "D", "M"), // when dealing with 100..999
        ("M", "V", "X"), // when dealing with 1000..9999
    ];
    let mut res: Vec<String> = Vec::new(); // holds the solution
    num.to_string() // convert number to string
        .chars() // turn string into an iterator of chars
        .rev() // reverse the iterator to start from the lowest digit
        .filter_map(|c| c.to_digit(10)) // convert char to digit
        .enumerate() // iterate over index and digit
        .for_each(|(i, d)| {
            // for each index and digit
            res.push(match d {
                // pattern match the digit
                0..=3 => ROMANS[i].0.repeat(d as usize), // 0..3 is always III, XXX, CCC, ...
                4 => format!("{}{}", ROMANS[i].0, ROMANS[i].1), // 4 is always IV, XL, CD, ...
                5..=8 => format!("{}{}", ROMANS[i].1, ROMANS[i].0.repeat(d as usize - 5)),
                9 => format!("{}{}", ROMANS[i].0, ROMANS[i].2), // 9 is always IX, XC, CM, ...
                _ => "".to_string(),
            })
        });
    res.reverse(); // reverse the vector
    res.join("") // join and return as string
}


fn main() {
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
