use std::collections::HashSet;

struct Element {
    vals: HashSet<u8>,
    sum: u8,
}

impl Element {
    fn new() -> Element {
        Element {
            vals: HashSet::new(),
            sum: 0,
        }
    }

    fn insert(&mut self, value: u8) -> bool {
        if !(1..=9).contains(&value) {
            panic!("Values must be in the range [1,9] for sudoku.")
        }

        let res = self.vals.insert(value);
        if res {
            self.sum += value;
        }
        res
    }

    fn check_sum(&self) -> bool {
        self.sum == 45
    }

    pub fn evaluate(slice: &[u8]) -> bool {
        let mut el = Element::new();
        for val in slice {
            if !el.insert(*val) {
                return false;
            }
        }
        el.check_sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn too_small() {
        let mut el = Element::new();
        el.insert(0);
    }

    #[test]
    #[should_panic]
    fn too_large() {
        let mut el = Element::new();
        el.insert(10);
    }

    #[test]
    fn insert() {
        let mut el = Element::new();
        assert!(el.insert(1));
        assert_eq!(el.sum, 1);
        assert!(el.insert(2));
        assert_eq!(el.sum, 3);
        assert!(!el.insert(2));
    }

    #[test]
    fn check_sum() {
        let mut el = Element::new();
        for val in 1..=9 {
            el.insert(val);
        }
        println!("{}", el.sum);
        assert!(el.check_sum())
    }

    #[test]
    fn evaluate() {
        let row = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let col = vec![1, 2, 3, 4, 5, 6, 7, 8, 1];
        let sqr = vec![1, 2, 3, 3, 4, 5, 6, 7, 8];
        assert!(Element::evaluate(&row));
        assert!(!Element::evaluate(&col));
        assert!(!Element::evaluate(&sqr));
    }
}
