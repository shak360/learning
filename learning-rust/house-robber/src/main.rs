pub fn rob(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold((0, 0), |(last, last_last), x| {
            (std::cmp::max(last, last_last + x), last)
        })
        .0
}

fn main() {
    println!("{}", rob([1, 2, 3, 1].to_vec()));
    assert_eq!(rob([1, 2, 3, 1].to_vec()), 4);

    println!("{}", rob([2, 7, 9, 3, 1].to_vec()));
    assert_eq!(rob([2, 7, 9, 3, 1].to_vec()), 12);

    println!("{}", rob([2, 1, 1, 2].to_vec()));
    assert_eq!(rob([2, 1, 1, 2].to_vec()), 4);

    println!("{}", rob([2, 1, 1, 2, 1, 2].to_vec()));
    assert_eq!(rob([2, 1, 1, 2, 1, 2].to_vec()), 6);

    println!("{}", rob([2, 1, 1, 2, 10, 1].to_vec()));
    assert_eq!(rob([2, 1, 1, 2, 10, 1].to_vec()), 13);
}
