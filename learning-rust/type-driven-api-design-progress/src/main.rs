use std::{collections::HashSet, thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// struct Progress takes type Iter
struct Progress<Iter> {
    iter: Iter,
    i: usize, // usize an unisgned integer where the number of bits depends on the architecture
    bound: Option<usize>,
    delims: (char, char),
}

// the impl block associates a method with a type (the struct)
// the impl block is quantified, for all types Iter, implement Progress of Iter
impl<Iter> Progress<Iter> {
    // struct (class?) instantiation method
    pub fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: None,
            delims: ('[', ']'),
        }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => {
                println!(
                    "[{}{}{}{}]",
                    self.delims.0,
                    "*".repeat(self.i),
                    " ".repeat(bound - self.i),
                    self.delims.1
                );
            }
            None => {
                println!("{}{}{}", self.delims.0, "*".repeat(self.i), self.delims.1);
            }
        }

        self.i += 1;
        self.iter.next()
    }
}

// add another method when there is a bound
impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

// be able to customize the delimiters
impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.delims = delims;
        self
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

// for all types Iter, implement the trait ProgressIteratorExt
// for that quantified type
// this has to be imported for use in other modules
impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn main() {
    fn expensive_calculation(_n: &i32) {
        sleep(Duration::from_secs(1))
    }

    let v: Vec<i32> = vec![1, 2, 3];
    // progress(v.iter(), expensive_calculation);
    // the above is still a little intrusive

    for n in v.iter().progress().with_bound().with_delims(('<', '>')) {
        expensive_calculation(n);
    }

    let mut h = HashSet::new();
    h.insert(0);
    // progress(h.iter(), expensive_calculation);
    // the above is still a little intrusive

    for n in h.iter().progress().with_bound() {
        expensive_calculation(n);
    }

    // this should error out because with_bound() isn't implemented for iterators without a bound
    // the trait bound for ExactSizeIterator was not satisfied
    // for n in (0..).progress().with_bound() {
    //     expensive_calculation(&n);
    // }
}
