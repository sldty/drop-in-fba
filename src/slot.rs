use std::{
    time,
    collections::{HashMap, HashSet},
};

use crate::{
    node::NodeId,
    value::{self, Value},
    message::Message,
    // topic::Prepare,
    ballot::Ballot,
    predicate::Predicate
};

#[derive(Debug, Clone, Copy)]
pub struct SlotId(usize);

// TODO: some sort of message storage thing?
// TODO: simplify and break out

pub struct Slot<T: Value> {
    id:         SlotId,
    node:       Node,
    phase:      Phase,
    messages:   HashMap<NodeId, Message<T>>,
    sent:       Option<Message<T>>,

    created:   time::Instant,
    nominated: HashSet<T>,
    accepted:  HashSet<T>,
    confirmed: HashSet<T>,

    ballot:     Ballot<T>,
    prepared_a: Ballot<T>,
    prepared_b: Ballot<T>,
    highest:    Ballot<T>,
    lowest:     Ballot<T>,

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

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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
            node:        node,
            phase:       Phase::Nominate,
            messages:    HashMap::new(),
            sent:        None,

            created:   time::Instant::now(),
            nominated: HashSet::new(),
            accepted:  HashSet::new(),
            confirmed: HashSet::new(),

            ballot: (),
            prepared_a: (),
            prepared_b: (),
            highest: (),
            lowest: (),

            priority_peers: HashSet::new(),
            priority_round: 1,
            priority_timer: todo!(),
        };

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

        // haiku:
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
                todo!()
                // if self.prepare.is_zero() { return; }
            }

            self.phase = Phase::NominatePrepare;
            self.update_ballot();
        }
    }

    // TODO: simplify
    pub fn update_ballot(&mut self) {
        if self.phase >= Phase::Commit { panic!(); }

        if !self.highest.is_zero() {
            self.ballot.value = self.highest.value;
        } else if !self.confirmed.is_empty() {
            // TODO: is unwrap ok here?
            self.ballot.value = value::combine(self.accepted, self.slot_id).unwrap();
        } else if !self.prepared_a.is_zero() {
            self.ballot.value = self.prepared_a.value;
        }

        todo!()
    }

    fn determine_quorum() {}

    fn find_quorum(&self, predicate: Box<dyn Predicate<T>>) -> HashSet<NodeId> {
        return self.node.quorum.find_quorum(self.node.id, self.messages, predicate);
    }

    pub fn update_values(&mut self) {
        // move values from nominated to accepted

        let mut to_promote = HashSet::new();
        let node_ids = self.accept(Slot::<T>::determine_quorum);

        // TODO: is this check redundant?
        if !node_ids.is_empty() {
            for value in to_promote.drain() {
                self.accepted.insert(value);
            }
        }

        for value in self.nominated.iter() {
            if self.accepted.contains(value) {
                self.nominated.remove(value);
            }
        }

        // move values from accepted to confirmed
        let to_promote = HashSet::new();
        let node_ids = self.find_quorum(todo!());

        // TODO: is this check redundant?
        if !node_ids.is_empty() {
            for value in to_promote.drain() {
                self.confirmed.insert(value);
            }
        }
    }
}
