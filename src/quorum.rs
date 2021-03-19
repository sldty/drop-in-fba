use std::collections::{HashMap, HashSet};

use crate::{
    predicate::Predicate,
    node::NodeId,
    message::Message,
};

/// A [`Quorum`] set is a set of nodes/subsets (a [`Member`]), named `members`.
/// A quorum slice is is a subset of a [`Quorum`] set,
/// With at least `threshold` number of `members`.
#[derive(Debug)]
pub struct Quorum {
    threshold: usize,
    members:   Vec<Member>,
}

/// A Member is either a [`Node`] (referenced by a [`NodeId`]),
/// or a nested [`Quorum`] set.
#[derive(Debug)]
pub enum Member {
    Node(NodeId),
    Quorum(Quorum),
}

impl Quorum {
    pub fn find_blocking(
        &self,
        messages:  &HashMap<NodeId, Message>,
        predicate: &Box<dyn Predicate>,
    ) -> () {
        todo!()
    }

    fn find_blocking_inner(
        mut needed:    usize,
        members:       &[Member],
        messages:      &HashMap<NodeId, Message>,
        mut predicate: Box<dyn Predicate>,
        mut so_far:    HashSet<NodeId>,
    ) -> (HashSet<NodeId>, Box<dyn Predicate>) {
        // base cases
        if needed == 0 { return (so_far, predicate) }
        if needed > members.len() { return (HashSet::new(), predicate) }

        // TODO: safe to unwrap?
        let (member, remaining) = members.split_first().unwrap();

        if let Member::Node(n) = member {
            if let Some(message) = messages.get(n) {
                if let Some(new_predicate) = predicate.test(message) {
                    needed -= 1;
                    predicate = new_predicate;
                    so_far.insert(*n);
                }
            }
        } else if let Member::Quorum(q) = member {
            let (new_so_far, new_predicate) = Quorum::find_blocking_inner(
                1 + (q.members.len() - q.threshold), &q.members,
                messages,
                predicate,
                so_far,
            );

            if !new_so_far.is_empty() {
                needed -= 1;
                predicate = new_predicate;
                so_far = new_so_far;
            }
        }

        return Quorum::find_blocking_inner(
            needed,
            remaining,
            messages,
            predicate,
            so_far
        );
    }
}
