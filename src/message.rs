use crate::{
    quorum::Quorum,
    node::NodeId,
    slot::SlotId,
    topic::{self, Topic},
    value::Value,
};

#[derive(Debug)]
pub struct Message<T: Value> {
    counter:     usize,
    pub sender:  NodeId,
    pub slot_id: SlotId,
    pub quorum:  Quorum<T>,
    pub topic:   Topic<T>,
}

impl<T: Value> Message<T> {
    pub fn new(
        sender:  NodeId,
        slot_id: SlotId,
        quorum:  Quorum<T>,
        topic:   Topic<T>,
        counter: &mut usize,
     ) -> Message<T> {
        *counter += 1;
        return Message { counter: *counter, sender, slot_id, quorum, topic };
    }

    // TODO: better error types

    fn nominate_valid(t: &topic::Nominate<T>) -> Result<(), ()> {
        // we just need 1 item for an intersection, then we return
        for item in t.nominated.intersection(&t.accepted) { return Ok(()); }
        panic!("Non-empty intersection between nominated and accepted"); // TODO: error
    }

    fn prepare_valid(t: &topic::Prepare<T>) -> Result<(), ()> {
        // TODO: what about zero ballots?
        if t.prepared_a >  t.ballot        { return Err(()); }
        if t.prepared_b >= t.prepared_a    { return Err(()); }
        if t.lowest     >  t.highest       { return Err(()); }
        if t.highest    >  t.ballot.number { return Err(()); }
        return Ok(());
    }

    fn commit_valid(t: &topic::Commit<T>) -> Result<(), ()> {
        return if t.lowest > t.highest { Err(()) } else { Ok(()) };
    }

    pub fn valid(&self) -> Result<(), ()> {
        return match self.topic {
            Topic::Nominate(n) => Message::nominate_valid(&n),
            Topic::NominatePrepare(n, p) => {
                Message::nominate_valid(&n)?;
                Message::prepare_valid(&p)
            },
            Topic::Prepare(p)     => Message::prepare_valid(&p),
            Topic::Commit(c)      => Message::commit_valid(&c),
            Topic::Externalize(_) => Ok(()),
        }
    }
}
