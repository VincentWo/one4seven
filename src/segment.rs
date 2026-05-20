pub trait Driver {
    type Error;
    fn set_state(&mut self, pos: u8, state: State) -> Result<(), Self::Error>;
    fn set_segment(&mut self, pos: u8, segments: Segment, enabled: bool)
    -> Result<(), Self::Error>;
    fn set_colon(&mut self, enabled: bool) -> Result<(), Self::Error>;
    fn update(&mut self) -> Result<(), Self::Error>;
}

pub enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
    Dot,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct State {
    pub top: bool,
    pub top_left: bool,
    pub top_right: bool,
    pub middle: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
    pub bottom: bool,
    pub dot: bool,
}
