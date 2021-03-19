use std::{
    cmp::{PartialOrd, Ord, Ordering},
    collections::HashSet
};

/// A totally orderable [`Topic`], i.e. something to vote on.
/// An enumeration that represents states in the state machine
/// needed to reach concensus.
#[derive(Debug, PartialEq, Eq)]
pub enum Topic {
    Nominate(Nominate),
    // TODO: why does this even exist?
    NominatePrepare(Nominate, Prepare),
    Prepare(Prepare),
    Commit(Commit),
    Externalize(Externalize),
}

// Nominate topic implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Nominate {
    nominated: HashSet<Value>,
    // 1. A _quorum_ votes-or-accepts the same value;
    // 2. A _blocking set_ accepts it.
    accepted:  HashSet<Value>,
}

impl Ord for Nominate {
    fn cmp(&self, other: &Self) -> Ordering {
        // comapare two nominated ballots
        // one with more nominations win
        // one with more accepteds wins in case of a tie
        if self.accepted.len() == other.accepted.len() {
            self.nominated.len().cmp(&other.nominated.len())
        } else {
            self.accepted.len().cmp(&other.accepted.len())
        }
    }
}

// Prepare topic implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Prepare {
    ballot:   Ballot,
    prepared: Ballot,
    // PP: Ballot, // unused field?
    lowest:   usize, // highest number?
    highest:  usize, // current number?
}

impl Ord for Prepare {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare ballots first
        match self.ballot.cmp(other.ballot) {
            Ordering::Equal => (),
            order           => { return order; }
        }
        match self.prepared.cmp(other.prepared) {
            Ordering::Equal => (),
            order           => { return order; }
        }

        // all ballots being equal, compare sequence numbers
        return self.highest.cmp(&other.highest);
    }
}

// Commit topic implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Commit {
    ballot:   Ballot,
    prepared: usize,
    lowest:   usize,
    highest:  usize,
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare ballots first
        match self.ballot.cmp(other.ballot) {
            Ordering::Equal => (),
            order           => { return order; }
        }

        // if ballots are same, compare prepared sequence number
        match self.prepared.cmp(&other.prepared) {
            Ordering::Equal => (),
            order           => { return order; }
        }

        // all else being the same, compare highest
        return self.highest.cmp(&other.highest);
    }
}

// Externalize topic implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Externalize {
    ballot:  Ballot,
    highest: usize,
}

impl Ord for Externalize {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.highest.cmp(&other.highest);
    }
}

// Ordering implementation for topics

// Partial ordering

impl PartialOrd for Topic       { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl PartialOrd for Nominate    { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl PartialOrd for Prepare     { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl PartialOrd for Commit      { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl PartialOrd for Externalize { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }

// Total ordering for topic

impl Ord for Topic {
    fn cmp(&self, other: &Self) -> Ordering {
        use Topic::*;

        // number closure
        let number = |&t| -> u8 {match t {
            Nominate(_)           => 0,
            NominatePrepare(_, _) => 1,
            Commit(_)             => 2,
            Externalize(_)        => 3,
        }};

        return match (self, other) {
            (NominatePrepare(sn, sp), NominatePrepare(on, op)) => {
                // compare nominate topics,
                // defaulting to prepare in case of a tie.
                match Ord::cmp(sn, on) {
                    Ordering::Equal => Ord::cmp(sp, op),
                    order           => order,
                }
            },

            // defer ordering to individual structs
            (Nominate(s),    Nominate(o))    => s.cmp(o),
            (Prepare(s),     Prepare(o))     => s.cmp(o),
            (Commit(s),      Commit(o))      => s.cmp(o),
            (Externalize(s), Externalize(o)) => s.cmp(o),

            // not the same, just compare numbers
            (s, o) => number(s).cmp(&number(o))
        };
    }
}
