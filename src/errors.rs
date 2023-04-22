#[derive(Debug)]
pub enum BlockError {

    BincodeError,
}


impl From<Box<bincode::ErrorKind>> for BlockError {
    fn from(_: Box<bincode::ErrorKind>) -> Self {
        return BlockError::BincodeError;
    }
}
