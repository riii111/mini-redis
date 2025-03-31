use bytes::Bytes;

#[derive(Debug)]
pub(crate) enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}
