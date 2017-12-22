#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Transaction {
    _id: usize,
}

impl Transaction {
    pub fn new(id: usize) -> Transaction {
        Transaction { _id: id }
    }

    pub fn id(&self) -> usize {
        self._id
    }
}
