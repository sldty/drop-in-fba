use std::collections::HashMap;

use crate::{
    value::Value,
    quorum::Quorum,
    slot::{Slot, SlotId},
    topic::{self, Topic},
    message::Message,
};

// TODO: make NodeId something unique, like a public key,
// or something generic, like a T: Value

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(String);

// TODO: wrap in Rc or something with SlotId as weakref
// because they both try to hold reference to each other
// "But Isaac!" I hear you say, "Rc is *slow*"
// "Not to worry" I console,
// "We'll not use Rc and instead, idk, remove the backref from Slot
// and have nodes be passed in as a function paramater."
// (or something).

pub struct Node<T: Value> {
    pub id:       NodeId,
    pub quorum:   Quorum<T>,
    pending:      HashMap<SlotId, Slot<T>>,
    externalized: HashMap<SlotId, topic::Externalize<T>>,

    /// A fraction from 0/255 (never) to 255/255 (always) that represents
    /// the chance of a message being ignored. Used for testing.
    _fake_drop: u8,
}

impl<T: Value> Node<T> {
    /// Build a new node.
    /// We explicitly pass in `externalized`
    /// so we can recover from disk, say.
    /// We don't pass in `pending`,
    /// Because an in-progrees slot shouldn't really exist
    /// outside of a running program.
    /// (The turnaround time is to fast for it to be reasonable).
    pub fn new(
        id:           NodeId,
        quorum:       Quorum<T>,
        externalized: HashMap<SlotId, topic::Externalize<T>>,
    ) -> Node<T> {
        return Node {
            id,
            quorum,
            pending: HashMap::new(),
            externalized,
            _fake_drop: 0
        };
    }

    pub fn run(&mut self, context: ()) {
        // delay until
        todo!();

        // loop {
        //     // read the next command
        //     match command {
        //         Command::Message  => (),
        //         Command::Delay    => (),
        //         Command::Defer    => (),
        //         Command::NewRound => (),
        //         Command::Rehandle => (),
        //     }
        // }
    }

    // TODO: have the return result be our response.
    // TODO: clean up logic around externalized messages.

    /// Handles a message, optionally returning a response.
    pub fn handle(&mut self, message: &Message<T>) -> Result<Option<Message<T>>, ()> {
        // we've already externalized the topic, so we don't need to do any more thinking
        // (unless someone else messaged us they externalized the topic as well)
        if let Some(externalized) = self.externalized.get(&message.slot_id) {
            if let Topic::Externalize(e) = message.topic {
                // the externalized value disagrees with what we think! oh no!
                if externalized.ballot.value != e.ballot.value {
                    eprintln!(
                        "Ahh! consensus failure! Inbound {:?} disagrees with own {:?}",
                        e.ballot.value,
                        externalized.ballot.value
                    );
                    panic!();
                }
            } else {
                return Ok(Some(Message::new(
                    self.id,
                    message.slot_id,
                    self.quorum,
                    Topic::Externalize(*externalized),
                    todo!(),
                )));
            }
            return Ok(None);
        }

        // create a new slot if we haven't already
        let slot = match self.pending.get_mut(&message.slot_id) {
            Some(s) => s,
            None => {
                let slot = Slot::new(message.slot_id, self);
                self.pending.insert(message.slot_id, slot);
                slot
            },
        };

        // run consensus and handle the message
        let outbound = slot.handle(message)?;

        // if the slot was externalized, move it to the externalized set
        if let Some(Topic::Externalize(e)) = outbound.topic {
            self.externalized.insert(slot.id, e);
            self.pending.remove(slot.id);
        }

        return outbound;
    }
}
