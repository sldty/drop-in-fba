use std::{
    fmt,
    collections::HashSet,
    hash::Hash,
};
use crate::slot::SlotId;

pub trait Value: Hash + Eq + Ord + fmt::Debug + Clone {
    fn combine(this: Self, that: Self, slot_id: SlotId) -> Self;
}

pub fn combine<T: Value>(value_set: HashSet<T>, slot_id: &SlotId) -> Option<T> {
    let mut acc = None;
    for value in value_set.into_iter() {
        acc = if let Some(so_far) = acc {
            Some(Value::combine(so_far, value, *slot_id))
        } else {
            Some(value)
        };
    }
    return acc;
}
