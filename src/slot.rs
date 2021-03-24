use std::{
    time,
    collections::{HashMap, HashSet},
};

use crate::{
    node::{Node, NodeId},
    value::{self, Value},
    message::Message,
    // topic::Prepare,
    ballot::Ballot,
    predicate::{self, Predicate},
    topic::{self, Topic}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotId(usize);

// TODO: some sort of message storage thing?
// TODO: simplify and break out

pub struct Slot<T: Value> {
    id:         SlotId,
    node:       Node<T>,
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
    // they are not, hmmm...

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
    pub fn new(slot_id: SlotId, node: Node<T>) -> Slot<T> {
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

            ballot: todo!(),
            prepared_a: todo!(),
            prepared_b: todo!(),
            highest: todo!(),
            lowest: todo!(),

            priority_peers: HashSet::new(),
            priority_round: 1,
            priority_timer: todo!(),
        };

        todo!()
    }

    // TODO: simplify building out Topics

    pub fn build_message(&self) -> Option<Message<T>> {
        let topic = match self.phase {
            Phase::Nominate => {
                if self.nominated.is_empty() && self.accepted.is_empty() { return None; }
                Topic::Nominate(topic::Nominate {
                    nominated: self.nominated,
                    accepted:  self.accepted,
                })
            },
            Phase::NominatePrepare => Topic::NominatePrepare(
                topic::Nominate { nominated: self.nominated, accepted: self.accepted },
                topic::Prepare {
                    ballot:     self.ballot,
                    prepared_a: self.prepared_a,
                    prepared_b: self.prepared_b,
                    highest:    self.highest.number,
                    lowest:     self.lowest.number,
                },
            ),
            Phase::Prepare => Topic::Prepare(topic::Prepare {
                ballot:     self.ballot,
                prepared_a: self.prepared_a,
                prepared_b: self.prepared_b,
                highest:    self.highest.number,
                lowest:     self.lowest.number,
            }),
            Phase::Commit => Topic::Commit(topic::Commit {
                ballot:   self.ballot,
                prepared: self.prepared_a.number,
                highest:  self.highest.number,
                lowest:   self.lowest.number,
            }),
            Phase::Externalize => Topic::Externalize(topic::Externalize {
                ballot:  self.lowest,
                highest: self.highest.number,
            }),
        };

        let mut message = Message::new(self.node.id, self.id, self.node.quorum, topic, todo!());
    }

    pub fn handle(&mut self, message: Message<T>) -> Result<Option<Message<T>>, ()> {
        // TODO: handle self messages

        // check message validity
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
                self.update_prepared();
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
            self.ballot.value = value::combine(self.accepted, &self.id).unwrap();
        } else if !self.prepared_a.is_zero() {
            self.ballot.value = self.prepared_a.value;
        }

        todo!()
    }

    fn find_blocking<'a>(&self, predicate: Box<dyn Predicate<T> + 'a>) -> HashSet<NodeId> {
        return self.node.quorum.find_blocking(&self.messages, predicate).0;
    }

    fn find_quorum<'a>(&self, predicate: Box<dyn Predicate<T> + 'a>) -> HashSet<NodeId> {
        return self.node.quorum.find_quorum(self.node.id, &self.messages, predicate).0;
    }

    // TODO: just pass in two predicates?
    fn accept<'a>(&self, f: &dyn Fn(bool) -> Box<dyn Predicate<T> + 'a>) -> HashSet<NodeId> {
        // if this slot's node already accepts the predicate we're done
        let predicate = f(false);
        if let Some(message) = self.sent {
            if let Some(_) = predicate.test(&message) {
                let mut accepting = HashSet::new();
                accepting.insert(self.node.id);
                return accepting;
            }
        }

        // if there is a blocking set that accepts we accept
        let blocking = self.find_blocking(predicate);
        if !blocking.is_empty() { return blocking; }

        // if there quorum that votes or accepts we accept
        let vote_predicate = f(true);
        if let Some(message) = self.sent {
            if let Some(_) = vote_predicate.test(&message) {
                return self.find_quorum(vote_predicate);
            }
        }

        // nobody accepts :(
        return HashSet::new();
    }

    pub fn update_values(&mut self) {
        // move values from nominated to accepted

        let mut to_promote = HashSet::new();
        let node_ids = self.accept(&|is_quorum| {
            Box::new(predicate::HashSetPredicate {
                values:       self.nominated,
                final_values: &mut to_promote,
                function:     |message, values| {
                    
                },
            })
        });

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
