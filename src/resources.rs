use specs::*;

#[derive(Debug)]
pub struct DeltaTime(pub f32);

#[derive(Debug)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fire: bool,
    pub special: bool,
}
