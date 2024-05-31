#[derive(Clone, Debug)]
pub struct PinEntityMetadata<K> where K: Copy + Clone {
    pub id: String,
    pub name: String,
    pub kind: K,
    pub pin: u8,
}

#[derive(Debug)]
pub struct PinEntity<K, S> where K: Copy + Clone, S: Copy + Clone {
    pub metadata: PinEntityMetadata<K>,
    pub state: S,
}
