use std::fmt;
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

impl<T: Value> Clone for FnPredicate<T> {
    fn clone(&self) -> Self {
        FnPredicate(self.0)
    }
}

impl<T: Value> fmt::Debug for FnPredicate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FnPredicate")
            .field(&format_args!("_"))
            .finish()
    }
}
