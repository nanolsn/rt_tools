use super::{
    super::{
        face::*,
        vertex::Vertex,
        sides::Sides,
    },
    parse_default,
    parse,
};

pub fn yaml_to_face(yml: &yaml::Yaml) -> Result<Face, FaceError> {
    use glm::{Vec2, Vec3};

    let layer: u32 = parse_default(&yml["layer"]);
    let contact: Sides = parse_default(&yml["contact"]);

    let norm: Vec3 = parse(&yml["norm"])?;
    let pos: Vec<Vec3> = parse(&yml["pos"])?;
    let st: Vec<Vec2> = parse(&yml["st"])?;

    let vs: Vec<Vertex> = pos
        .into_iter()
        .zip(st)
        .map(|(pos, st)| Vertex {
            pos,
            st,
            norm,
        })
        .collect();

    Ok(Face::new(&vs, contact, layer)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse_code;
    use glm::{vec3, vec2};

    #[test]
    fn parse() {
        let code = r#"
        pos:
          - [ -0.5, 0.5, -0.5 ]
          - [ -0.5, 0.5, 0.5 ]
          - [ 0.5, 0.5, 0.5 ]
          - [ 0.5, 0.5, -0.5 ]

        st:
          - [ 0, 1 ]
          - [ 0, 0 ]
          - [ 1, 0 ]
          - [ 1, 1 ]

        norm: [ 0, 1, 0 ]
        layer: 12
        contact: .
        "#;

        let a: Face = parse_code(code).unwrap();

        let b = Face {
            vertexes: FaceVertexes::Square([
                Vertex {
                    pos: vec3(-0.5, 0.5, -0.5),
                    st: vec2(0., 1.),
                    norm: vec3(0., 1., 0.),
                },
                Vertex {
                    pos: vec3(-0.5, 0.5, 0.5),
                    st: vec2(0., 0.),
                    norm: vec3(0., 1., 0.),
                },
                Vertex {
                    pos: vec3(0.5, 0.5, 0.5),
                    st: vec2(1., 0.),
                    norm: vec3(0., 1., 0.),
                },
                Vertex {
                    pos: vec3(0.5, 0.5, -0.5),
                    st: vec2(1., 1.),
                    norm: vec3(0., 1., 0.),
                },
            ]),
            contact: Sides::all(),
            layer: 12,
        };

        assert_eq!(a, b);
    }
}
