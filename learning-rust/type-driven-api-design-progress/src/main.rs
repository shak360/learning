use std::{collections::HashSet, thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// struct Progress takes type Iter
struct Progress<Iter> {
    iter: Iter,
    i: usize, // usize an unisgned integer where the number of bits depends on the architecture
}

// the impl block associates a method with a type (the struct)
// the impl block is quantified, for all types Iter, implement Progress of Iter
impl<Iter> Progress<Iter> {
    // struct (class?) instantiation method
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1))
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    // progress(v.iter(), expensive_calculation);
    // the above is still a little intrusive

    for n in Progress::new(v.iter()) {
        expensive_calculation(n);
    }

    let mut h = HashSet::new();
    h.insert(0);
    // progress(h.iter(), expensive_calculation);
    // the above is still a little intrusive
}
