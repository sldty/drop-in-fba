use std::cmp::{PartialOrd, Ord, Ordering};

use crate::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Ballot<T: Value> {
    number: usize,
    value:  T,
}

impl<T: Value> PartialOrd for Ballot<T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }

impl<T: Value> Ord for Ballot<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.number.cmp(&other.number) {
            Ordering::Equal => (),
            order           => { return order; },
        }

        return self.value.cmp(&other.value);
    }
}
