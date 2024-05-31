pub trait PinState: Copy + Clone + Default + Send + Sync {}

pub trait PinEntityKind: Copy + Clone + PartialEq + Send + Sync {}

#[derive(Clone, Debug)]
pub struct PinEntityMetadata<K>
    where K: PinEntityKind {
    pub id: String,
    pub name: String,
    pub kind: K,
    pub pin: u8,
}

#[derive(Debug)]
pub struct PinEntity<K, S>
    where K: PinEntityKind,
          S: PinState {
    pub metadata: PinEntityMetadata<K>,
    pub state: S,
}
