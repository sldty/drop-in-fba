use crate::{
    quorum::Quorum,
    node::NodeId,
    slot::SlotId,
};

#[derive(Debug)]
pub struct Message {
    counter: usize,
    sender:  NodeId,
    slot_id: SlotId,
    quorum:  Quorum,
    topic:   Topic,
}
