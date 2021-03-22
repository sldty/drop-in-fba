use std::collections::HashMap;

use crate::{
    value::Value,
    quorum::Quorum,
    slot::{Slot, SlotId}, topic,
};

// TODO: make NodeId something unique, like a public key,
// or something generic, like a T: Value

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(String);

// TODO: wrap in Rc or something with SlotId as weakref
// because they both try to hold reference to each other

// TODO: ok, so the original go implementation uses a separate goroutine
// (basically like a thread that you can pass messages in and out of)
// but the issue with this is that Rust doesn't have Goroutines
// (or rustroutines, for that manner)
// Now, I could be a simp and use Tokio
// but then everyone who uses this would have to pull in tokio.
// I'm not sure, but I think that just implementing Send+Sync
// and then allow people to bring-your-own-runtime it (Tokio included).
// but I need to finish implementing slot first.

pub struct Node<T: Value> {
    id:           NodeId,
    quorum:       Quorum<T>,
    pending:      HashMap<SlotId, Slot<T>>,
    externalized: HashMap<SlotId, topic::Externalize<T>>,

    // TODO: channel field for sending messages.

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

    pub fn handle() {
        todo!()
    }
}
