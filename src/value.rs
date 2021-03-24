use std::{
    fmt,
    collections::HashSet,
    hash::Hash,
};
use crate::slot::SlotId;

/// This is quite the supertrait.
/// [`Value`] represents any arbitrary data that
/// the network is trying to reach consensus on.
/// In addition to all the traits the data must implement,
/// it must implement [`Value::combine`]
/// which must combine itself with another to form
/// a new Value in a deterministic and communative manner.
/// (e.g. taking the union of two sets, or using the older item).
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
