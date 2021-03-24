use std::{
    cmp::{PartialOrd, Ord, Ordering},
    collections::HashSet
};

use crate::{ballot::Ballot, value::Value};

/// A totally orderable [`Topic`], i.e. something to vote on.
/// An enumeration that represents states in the state machine
/// needed to reach consensus.
#[derive(Debug, PartialEq, Eq)]
pub enum Topic<T: Value> {
    Nominate(Nominate<T>),
    // TODO: why does this even exist?
    NominatePrepare(Nominate<T>, Prepare<T>),
    Prepare(Prepare<T>),
    Commit(Commit<T>),
    Externalize(Externalize<T>),
}

// Nominate topic implementation

#[derive(Debug)]
pub struct Nominate<T: Value> {
    pub nominated: HashSet<T>,
    // 1. A _quorum_ votes-or-accepts the same value;
    // 2. A _blocking set_ accepts it.
    pub accepted:  HashSet<T>,
}

// equality is length-based - we never actually compare the sets.
impl<T: Value> PartialEq for Nominate<T> {
    fn eq(&self, other: &Self) -> bool {
          self.nominated.len() == other.nominated.len()
        && self.accepted.len() == other.accepted.len()
    }
}

impl<T: Value> Eq for Nominate<T> {}

impl<T: Value> Ord for Nominate<T> {
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
pub struct Prepare<T: Value> {
    pub ballot:      Ballot<T>,
    pub prepared_a:  Ballot<T>,
    pub prepared_b:  Ballot<T>,
    pub highest: usize, // current number?
    pub lowest:  usize, // highest number?
}

impl<T: Value> Ord for Prepare<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare ballots first
        match self.ballot.cmp(&other.ballot) {
            Ordering::Equal => (),
            order           => { return order; }
        }
        match self.prepared_a.cmp(&other.prepared_a) {
            Ordering::Equal => (),
            order           => { return order; }
        }
        match self.prepared_b.cmp(&other.prepared_b) {
            Ordering::Equal => (),
            order           => { return order; }
        }

        // all ballots being equal, compare sequence numbers
        return self.highest.cmp(&other.highest);
    }
}

// Commit topic implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Commit<T: Value> {
    pub ballot:   Ballot<T>,
    pub prepared: usize,
    pub highest:  usize,
    pub lowest:   usize,
}

impl<T: Value> Ord for Commit<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare ballots first
        match self.ballot.cmp(&other.ballot) {
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
pub struct Externalize<T: Value> {
    pub ballot:  Ballot<T>,
    pub highest: usize,
}

impl<T: Value> Ord for Externalize<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.highest.cmp(&other.highest);
    }
}

// Ordering implementation for topics

// Partial ordering

impl<T: Value> PartialOrd for Topic      <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl<T: Value> PartialOrd for Nominate   <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl<T: Value> PartialOrd for Prepare    <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl<T: Value> PartialOrd for Commit     <T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl<T: Value> PartialOrd for Externalize<T> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }

// Total ordering for topic

impl<T: Value> Ord for Topic<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Topic::*;

        // number closure
        let number = |t: &Topic<T>| -> u8 {match t {
            Nominate(_)           => 0,
            NominatePrepare(_, _) => 1,
            Prepare(_)            => 2,
            Commit(_)             => 3,
            Externalize(_)        => 4,
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
