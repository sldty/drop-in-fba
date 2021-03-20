use crate::{
    quorum::Quorum,
    node::NodeId,
    slot::SlotId,
    topic::Topic,
    value::Value,
};

#[derive(Debug)]
pub struct Message<T: Value> {
    counter: usize,
    pub sender:  NodeId,
    slot_id: SlotId,
    quorum:  Quorum<T>,
    pub topic:   Topic<T>,
}
