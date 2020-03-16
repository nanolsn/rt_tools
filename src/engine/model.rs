use super::{
    face::Face,
    sides::Sides,
    vertex::Vertex,
};

#[derive(Debug, PartialEq)]
pub struct Model {
    pub faces: Vec<Face>,
    pub full_sides: Sides,
}

impl Model {
    pub fn get_indexed_vertexes(&self) -> (Vec<Vertex>, Vec<u32>) {
        let min_capacity = self.faces.len() * 3;
        let mut vertexes = Vec::with_capacity(min_capacity);
        let mut indexes = Vec::with_capacity(min_capacity);

        for face in &self.faces {
            face.vertexes.extend_vertexes(&mut vertexes, &mut indexes);
        }

        (vertexes, indexes)
    }
}
