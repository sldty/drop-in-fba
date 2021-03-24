use std::{
    fmt,
    collections::HashSet,
};
use crate::{
    value::Value,
    message::Message
};

/// A predicate is a condition we use to build up and narrow down stuff.
pub trait Predicate<T: Value>: fmt::Debug {
    fn test<'s>(self: Box<Self>, message: &Message<T>)
        -> Option<Box<dyn Predicate<T> + 's>> where Self: 's;
    /// Like clone but for traits
    fn dupe<'s>(&self) -> Box<dyn Predicate<T> + 's> where Self: 's;
}

// Function predicate

#[derive(Clone)]
pub struct FnPredicate<T: Value>(fn(&Message<T>) -> bool);

impl<T: Value> Predicate<T> for FnPredicate<T> {
    fn test<'s>(self: Box<Self>, message: &Message<T>)
        -> Option<Box<dyn Predicate<T> + 's>> where Self: 's
    {
        return if (self.0)(message) {
            Some(self)
        } else {
            None
        }
    }

    fn dupe<'s>(&self) -> Box<dyn Predicate<T> + 's> where Self: 's { Box::new(self.clone()) }
}

impl<T: Value> fmt::Debug for FnPredicate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FnPredicate")
            .field(&format_args!("_"))
            .finish()
    }
}

// Value set predicate

/// A predicate that narrows down a set of values.
/// Type paramater `T`, as always, is the type of the value.
/// However, note the additional type paramater `S`,
/// which is the type of the item of the set.
/// There are two common sets used with the [`hashSetPredicate`]:
/// 1. A [`Value`] set, in which case `T = S`.
/// 2. A [`Ballot`] set, in which case `T != S`; `S = Ballot`.
/// Note that, for instance, the [`HashSetPredicate`] for
/// a [`Ballot`] set is written `HashSetPredicate<Value, Ballot<Value>>`
/// and not `HashSetPredicate<Value, HashSet<Ballot<Value>>>`.
/// The type paramater `S` should not be a `HashSet`.
#[derive(Clone)]
pub struct HashSetPredicate<'a, T: Value, S: fmt::Debug + Clone> {
    values:       HashSet<S>,
    final_values: &'a mut HashSet<S>,
    // TODO: fnmut?
    function:     fn(&Message<T>, &HashSet<S>) -> HashSet<S>,
}

impl<T: Value, S: fmt::Debug + Clone> Predicate<T> for HashSetPredicate<T, S> {
    fn test<'s>(mut self: Box<Self>, message: &Message<T>)
        -> Option<Box<dyn Predicate<T> + 's>> where Self: 's
    {
        if self.values.is_empty() { return None; }
        let next_values = (self.function)(message, &self.values);

        self.values = match next_values.is_empty() {
            true  => { return None; },
            false => { self.final_values = next_values.clone(); next_values }
        };

        return Some(self);
    }

    fn dupe<'s>(&self) -> Box<dyn Predicate<T> + 's> where Self: 's { Box::new(self.clone()) }
}

impl<T: Value, S: fmt::Debug + Clone> fmt::Debug for HashSetPredicate<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HashSetPredicate")
            .field("values",       &self.values)
            .field("final_values", &self.final_values)
            .field("function",     &format_args!("_"))
            .finish()
    }
}

// TODO: there's something funky going on with nil in this predicate and
// the hash set predicate. (the final fields)
// I need to investigate it.
// TODO: remove final fields on these two predicates?
// Okok, so I figured out what the final fields are for
// basically, when we apply a predicate, we make the final fields a mutable reference
// to a value accessible outside the predicate. One problemo: we can't do this in Rust
// because it violates the only one mutable reference rules.
// uhh, this is no bueno.
// TODO: how do we get the final values out of the predicate?
// one option is to implement a destructive function that reduces a predicate to it's final values
// that might work, actually.
// imma commit really quick in case everything goes wrong.

// Min max predicate

#[derive(Clone)]
pub struct MinMaxPredicate<T: Value> {
    min:       usize,
    max:       usize,
    final_min: usize,
    final_max: usize,
    function:  fn(&Message<T>, usize, usize) -> (bool, usize, usize),
}

impl<T: Value> Predicate<T> for MinMaxPredicate<T> {
    fn test<'s>(mut self: Box<Self>, message: &Message<T>)
        -> Option<Box<dyn Predicate<T> + 's>> where Self: 's
    {
        if self.min > self.max { return None; }

        let (res, min, max) = (self.function)(message, self.min, self.max);
        if !res { return None; }

        self.min       = min;
        self.max       = max;
        self.final_min = min;
        self.final_max = max;
        return Some(self);
    }

    fn dupe<'s>(&self) -> Box<dyn Predicate<T> + 's> where Self: 's { Box::new(self.clone()) }
}

impl<T: Value> fmt::Debug for MinMaxPredicate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MinMaxPredicate")
            .field("min",       &self.min)
            .field("max",       &self.max)
            .field("final_min", &self.final_min)
            .field("final_max", &self.final_max)
            .field("function",  &format_args!("_"))
            .finish()
    }
}
