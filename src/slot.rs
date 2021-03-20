use std::{
    time,
    collections::{HashMap, HashSet},
};

use crate::{
    node::NodeId,
    value::Value,
    message::Message,
    topic::Prepare
};

#[derive(Debug, Clone, Copy)]
pub struct SlotId(usize);

// TODO: some sort of message storage thing?
// TODO: simplify and break out

pub struct Slot<T: Value> {
    id:         SlotId,
    validator:  usize,
    phase:      Phase,
    messages:   HashMap<NodeId, Message<T>>,
    sent:       Message<T>,

    created:   time::Instant,
    nominated: HashSet<T>,
    accepted:  HashSet<T>,
    confirmed: HashSet<T>,
    prepared:  Option<Prepare<T>>,

    // what is the point of these priority peers?
    // are they like the quorum slice of this node?

    priority_peers: HashSet<NodeId>,
    priority_round: usize,
    priority_timer: time::Instant,

    // Update thing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct DummyValue(usize);

    impl Value for DummyValue {
        fn combine(this: Self, that: Self, slot_id: SlotId) -> Self {
            DummyValue(this.0 + that.0)
        }
    }

    #[test]
    fn slot_size() {
        println!("size of slot: {}", std::mem::size_of::<Slot<DummyValue>>())
    }
}

/// Represents the current phase of a slot.
/// Compare this with [`topic::Ballot`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Nominate = 0,
    NominatePrepare,
    Prepare,
    Commit,
    Externalize,
}

impl<T: Value> Slot<T> {
    pub fn new(slot_id: SlotId, node: Node) -> Slot<T> {
        Slot {
            id:          slot_id,
            validator:   node,
            phase:       Phase::Nominate,
            messages:    HashMap::new(),
            sent:        None,

            created:   time::Instant::now(),
            nominated: HashSet::new(),
            accepted:  HashSet::new(),
            confirmed: HashSet::new(),
            prepared:  None,

            priority_peers: HashSet::new(),
            priority_round: 1,
            priority_timer: todo!(),
        }

        todo!()
    }

    pub fn handle(&mut self, message: Message<T>) -> Result<Option<Message<T>>, ()> {
        // TODO: nomination phase

        message.valid()?;

        // make sure this is the most up-to-date message
        // if it isn't, retrieve the most up-to-date one we have
        let message = match self.messages.get(&message.sender) {
            Some(m) if m.topic >= message.topic => m,
            _ => { self.messages.insert(message.sender, message); &message },
        };

        // I'm not sure what the whole deal is with this
        // Nominate prepare phase.
        // I think the phases lend themselves nicely to pattern matching,
        // but this phase variant is honestly a huge wrench.

        if self.phase == Phase::Nominate
            || self.phase == Phase::NominatePrepare
                { self.nominate(message); }

        if self.phase == Phase::NominatePrepare
            || self.phase == Phase::Prepare
                { self.prepare(); }

        if self.phase == Phase::Commit
            { self.commit();}

        // I trust the quorum,
        // and count the votes I have seen.
        // We reach concensus.

        let new_message = match self.build_message() {
            m @ Some(_) if m == self.sent => None,
            unique => { self.sent = unique; unique },
        };

        return Ok(new_message);
    }

    pub fn nominate(&mut self, message: &Message<T>) {
        if self.nominated.len() == 0
        && self.priority_peers.contains(&message.sender) {
            // add nominated values to own set
            todo!()
        }

        // promote accepted values to nominated,
        // and confirmed values to nominated.
        self.update_values();

        // if a value has been confirmed to be nominated,
        // we move on to the prepare phase.
        if self.phase == Phase::Nominate {
            if self.confirmed.is_empty() {
                self.update_prepare();
                // if self.prepare.is_zero() { return; }
            }

            self.phase = Phase::NominatePrepare;
            self.update_ballot();
        }
    }

    pub fn update_ballot(&mut self) {
        if self.phase >= Phase::Commit { return; }

        if let Some(prepared) = self.prepared {
            prepared.highest
        }

        if self.confirmed.is_empty() {
            self.ballot.
        }

        todo!()
    }
}
