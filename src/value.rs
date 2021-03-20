use std::fmt;
use crate::slot::SlotId;

pub trait Value: Eq + Ord + fmt::Debug {
    fn combine(this: Self, that: Self, slot_id: SlotId) -> Self;
}
