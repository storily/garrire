struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits { n, divisor }
    }
}

impl From<usize> for Digits {
    fn from(n: usize) -> Self {
        Self::new(n)
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}

pub fn is_palindromic(n: usize) -> bool {
    let d: Digits = n.into();
    let d: Vec<usize> = d.collect();
    let mut p = d.clone();
    p.reverse();
    p == d
}

pub fn next_palindromic(mut n: usize) -> usize {
    loop {
        n += 1;
        if is_palindromic(n) {
            return n;
        }
    }
}
