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

/// A [`Quorum`] set is a set of nodes/subsets (a [`Member`]), named `members`.
/// A quorum slice is is a subset of a [`Quorum`] set,
/// With at least `threshold` number of `members`.
#[derive(Debug)]
pub struct Quorum<T: Value> {
    threshold:      usize,
    members:        Vec<Member<T>>,
    _phantom_value: PhantomData<T>,
}

/// A Member is either a [`Node`] (referenced by a [`NodeId`]),
/// or a nested [`Quorum`] set.
#[derive(Debug)]
pub enum Member<T: Value> {
    Node(NodeId),
    Quorum(Quorum<T>),
}

impl<T: Value> Quorum<T> {
    fn needed(&self) -> usize {
        return 1 + (self.members.len() - self.threshold);
    }

    pub fn find_blocking<'a>(
        &self,
        messages:  &HashMap<NodeId, Message<T>>,
        predicate: Box<dyn Predicate<T> + 'a>,
    ) -> (HashSet<NodeId>/*, Box<dyn Predicate<T> + 'a>*/) where T: 'a {
        let (blocking, _) = Quorum::find_blocking_inner(
            self.needed(),
            &self.members,
            messages,
            predicate,
            HashSet::new(),
        );
        return blocking;
    }

    // TODO: remove unnessary clones and boxing
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

        if let Member::Node(n) = member {
            if let Some(message) = messages.get(n) {
                if let Some(new_predicate) = predicate.dupe().test(message) {
                    needed -= 1;
                    predicate = new_predicate;
                    so_far.insert(n.clone());
                }
            }
        } else if let Member::Quorum(q) = member {
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
        }

        return Quorum::find_blocking_inner(
            needed,
            remaining,
            messages,
            predicate,
            so_far,
        );
    }
}
