#[derive(Clone, Copy, Debug)]
pub enum GpioEntityState {
    Low,
    High,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpioEntityKind {
    Light,
    Fan,
    Generic,
}

#[derive(Clone, Debug)]
pub struct GpioEntityMetadata {
    pub id: String,
    pub name: String,
    pub kind: GpioEntityKind,
    pub pin: u8,
}

#[derive(Debug)]
pub struct GpioEntity {
    pub metadata: GpioEntityMetadata,
    pub state: GpioEntityState,
}
