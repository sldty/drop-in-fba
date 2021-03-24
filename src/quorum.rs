use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::{
    predicate::Predicate,
    node::NodeId,
    message::Message,
    value::Value,
};

// TODO: quorum value type <T> where T: Value?
// TODO: we're basically implementing a backtracking algorithm
// but the problem is it makes a copy on each recursive call.
// better solution would be to use a mutable datatype
// with an 'undo' operation
// that only stores the mutations
// and can be called out into a full datatype upon backtracking completing.

/// A [`Quorum`] set is a set of nodes/subsets (a [`Member`]), named `members`.
/// A quorum slice is is a subset of a [`Quorum`] set,
/// With at least `threshold` number of `members`.
#[derive(Debug, PartialEq, Eq)]
pub struct Quorum<T: Value> {
    threshold:      usize,
    members:        Vec<Member<T>>,
    _phantom_value: PhantomData<T>,
}

/// A Member is either a [`Node`] (referenced by a [`NodeId`]),
/// or a nested [`Quorum`] set.
#[derive(Debug, PartialEq, Eq)]
pub enum Member<T: Value> {
    Node(NodeId),
    Quorum(Quorum<T>),
}

// TODO: find blocking and find quorum are very similar; refactor?

impl<T: Value> Quorum<T> {
    fn needed(&self) -> usize {
        return 1 + (self.members.len() - self.threshold);
    }

    pub fn find_blocking<'a>(
        &self,
        messages:  &HashMap<NodeId, Message<T>>,
        predicate: Box<dyn Predicate<T> + 'a>,
    ) -> (HashSet<NodeId>, Box<dyn Predicate<T> + 'a>) where T: 'a {
        return Quorum::find_blocking_inner(
            self.needed(),
            &self.members,
            messages,
            predicate,
            HashSet::new(),
        );
    }

    // TODO: remove unnessary clones, dupes, and boxing
    // would also have to change the Predicate trait

    fn find_blocking_inner<'a>(
        mut needed:    usize,
        members:       &[Member<T>],
        messages:      &HashMap<NodeId, Message<T>>,
        mut predicate: Box<dyn Predicate<T> + 'a>,
        mut so_far:    HashSet<NodeId>,
    ) -> (HashSet<NodeId>, Box<dyn Predicate<T> + 'a>) where T: 'a {
        // base cases
        if needed == 0 { return (so_far, predicate) }
        if needed > members.len() { return (HashSet::new(), predicate) }

        // TODO: safe to unwrap?
        let (member, remaining) = members.split_first().unwrap();

        match member {
            Member::Node(n) => {
                if let Some(message) = messages.get(n) {
                    if let Some(new_predicate) = predicate.dupe().test(message) {
                        needed -= 1;
                        predicate = new_predicate;
                        so_far.insert(n.clone());
                    }
                }
            },
            Member::Quorum(q) => {
                let (new_so_far, new_predicate) = Quorum::find_blocking_inner(
                    q.needed(),
                    &q.members,
                    messages,
                    predicate.dupe(),
                    so_far.clone(),
                );

                // backtrack here, which is why we make a copy of predicate
                if !new_so_far.is_empty() {
                    needed -= 1;
                    predicate = new_predicate;
                    so_far = new_so_far;
                }
            },
        }

        return Quorum::find_blocking_inner(
            needed,
            remaining,
            messages,
            predicate,
            so_far,
        );
    }

    pub fn find_quorum<'a>(
        &self,
        node_id:   NodeId,
        messages:  &HashMap<NodeId, Message<T>>,
        predicate: Box<dyn Predicate<T> + 'a>,
    ) -> (HashSet<NodeId>, Box<dyn Predicate<T> + 'a>) where T: 'a {
        todo!()
    }

    pub fn find_quorum_inner<'a>(
        mut threshold: usize,
        members:       &[Member<T>],
        messages:      &HashMap<NodeId, Message<T>>,
        mut predicate: Box<dyn Predicate<T> + 'a>,
        mut so_far:    HashSet<NodeId>,
    ) -> (HashSet<NodeId>, Box<dyn Predicate<T> + 'a>) where T: 'a {
        // base cases
        if threshold == 0 { return (so_far, predicate); }
        if threshold > members.len() { return (HashSet::new(), predicate); }

        // TODO: safe to unwrap?
        let (member, remaining) = members.split_first().unwrap();

        match member {
            // TODO: refactor this out.
            // note that we basically call find_blocking_inner in both branches
            Member::Node(n) => {
                if so_far.contains(n) {
                    threshold -= 1;
                } else if let Some(message) = messages.get(n) {
                    if let Some(new_predicate) = predicate.dupe().test(message) {
                        let mut new_so_far = so_far.clone();
                        new_so_far.insert(message.sender.clone());

                        let (new_so_far, new_predicate) = Quorum::find_blocking_inner(
                            message.quorum.needed(),
                            &message.quorum.members,
                            messages,
                            predicate.dupe(),
                            new_so_far,
                        );

                        if !new_so_far.is_empty() {
                            threshold -= 1;
                            predicate = new_predicate;
                            so_far = new_so_far;
                        }
                    }
                }
            },
            Member::Quorum(q) => {
                let (new_so_far, new_predicate) = Quorum::find_quorum_inner(
                    q.threshold,
                    &q.members,
                    messages,
                    predicate.dupe(),
                    so_far.clone(),
                );

                // backtrack here, which is why we make a copy of predicate
                if !new_so_far.is_empty() {
                    threshold -= 1;
                    predicate = new_predicate;
                    so_far = new_so_far;
                }
            }
        }

        return Quorum::find_quorum_inner(
            threshold,
            remaining,
            messages,
            predicate,
            so_far,
        );
    }
}
