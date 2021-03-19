use std::fmt;
use crate::message::Message;

pub trait Predicate: fmt::Debug {
    fn test(self, message: &Message) -> Option<Box<dyn Predicate>>;
}

pub struct FnPredicate(Box<dyn Fn(&Message) -> bool>);

impl Predicate for FnPredicate {
    fn test(self, message: &Message) -> Option<Box<dyn Predicate>> {
        return if (self.0)(message) {
            Some(Box::new(self))
        } else {
            None
        }
    }
}

impl fmt::Debug for FnPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FnPredicate")
            .field(&format_args!("_"))
            .finish()
    }
}
