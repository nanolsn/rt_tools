#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub pos: glm::Vec3,
    pub st: glm::Vec2,
    pub norm: glm::Vec3,
}
