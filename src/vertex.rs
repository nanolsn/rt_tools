use glm::{Vec3, Vec2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub pos: Vec3,
    pub st: Vec2,
    pub norm: Vec3,
}
