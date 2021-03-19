pub enum Topic {
    Nominate(Nominate),
    NominatePrepare(Nominate, Prepare),
    Prepare(Prepare),
    Commit(Commit),
    Externalize(Externalize),
}

pub struct Nominate {
    nominated: HashSet<Value>,
    // 1. A _quorum_ votes-or-accepts the same value;
    // 2. A _blocking set_ accepts it.
    accepted:  HashSet<Value>,
}

pub struct Prepare {
    
}
