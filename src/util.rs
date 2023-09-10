use hecs::Entity;
use serde_derive::Serialize;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Size(pub i32, pub i32);

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Transform(pub i32, pub i32);

impl std::ops::Mul for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Transform {
        Transform(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<Pos> for Transform {
    type Output = Pos;
    fn mul(self, rhs: Pos) -> Pos {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Parent {
    pub entity: Entity,
    pub transform: Transform
}