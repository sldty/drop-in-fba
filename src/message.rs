use crate::{
    quorum::Quorum,
    node::NodeId,
    slot::SlotId,
    topic::Topic,
};

#[derive(Debug)]
pub struct Message {
    counter: usize,
    sender:  NodeId,
    slot_id: SlotId,
    quorum:  Quorum,
    topic:   Topic,
}
