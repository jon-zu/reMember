use shroom_pkt::ShroomPacket;

pub type Vec2 = euclid::default::Vector2D<i16>;
pub type Box2 = euclid::default::Box2D<i16>;
pub type Rect2 = euclid::default::Rect<i16>;
//TODO adjust this
pub type Rect2D = euclid::default::Box2D<i16>;
pub type TagPoint = euclid::default::Point2D<i32>;


#[derive(Debug, ShroomPacket, Copy, Clone)]
pub struct Range2 {
    pub low: i16,
    pub high: i16,
}

#[derive(Debug, ShroomPacket)]
pub struct Rect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[derive(Debug, ShroomPacket, Clone)]
pub struct Rect32 {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}