#[derive(Debug)]
#[repr(usize)]
#[allow(dead_code)] // TODO: remove this
pub enum Category {
    Unknown,
    Effect,
    Synth,
    Analysis,
    Matering,
    Spacializer,
    RoomEffect,
    SurroundEffect,
    Restoration,
    OfflineProcess,
    Container,
    Generator,
}
impl_clike!(Category);