#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    // Case 1: Equal
    if a == b {
        return Comparison::Equal;
    }

    // Helper: check if small list is contained inside big list
    fn is_sublist<T: PartialEq>(small: &[T], big: &[T]) -> bool {
        if small.is_empty() {
            return true;
        }

        if small.len() > big.len() {
            return false;
        }

        big.windows(small.len()).any(|window| window == small)
    }

    // Case 2: A is sublist of B
    if is_sublist(a, b) {
        return Comparison::Sublist;
    }

    // Case 3: A is superlist of B
    if is_sublist(b, a) {
        return Comparison::Superlist;
    }

    // Case 4: None of the above
    Comparison::Unequal
}