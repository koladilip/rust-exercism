#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn default() -> Palindrome {
        Palindrome {
            value: 0,
            factors: Vec::new(),
        }
    }

    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b))
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }
}

fn reverse_digits(num: u64) -> u64 {
    let mut reverse_num = 0;
    let mut given_num = num;
    while given_num > 0 {
        reverse_num = reverse_num * 10 + given_num % 10;
        given_num = given_num / 10;
    }
    reverse_num
}

fn is_palindrome(num: u64) -> bool {
    num == reverse_digits(num)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_palindrome = Palindrome::default();
    let mut max_palindrome = Palindrome::default();
    for i in min..=max {
        for j in i..=max {
            let val = i * j;

            if is_palindrome(val) {
                if min_palindrome.is_empty() || min_palindrome.value() > val {
                    min_palindrome = Palindrome::new(i, j)
                } else if min_palindrome.value() == val {
                    min_palindrome.insert(i, j)
                }

                if max_palindrome.is_empty() || max_palindrome.value() < val {
                    max_palindrome = Palindrome::new(i, j)
                } else if max_palindrome.value() == val {
                    max_palindrome.insert(i, j)
                }
            }
        }
    }
    match min_palindrome.factors.is_empty() {
        true => None,
        _ => Some((min_palindrome, max_palindrome)),
    }
}
