pub mod quorum;
pub mod predicate;
pub mod node;
pub mod message;
pub mod slot;
pub mod topic;
pub mod ballot;
pub mod value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
