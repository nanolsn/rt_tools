use super::{
    super::{
        model::{Model, ModelError},
        face::*,
    },
    parse_or_default,
    parse,
};

pub fn yaml_to_model(yml: &yaml::Yaml) -> Result<Model, ModelError> {
    let faces = parse(&yml["faces"])
        .map_err(|e: Option<FaceError>| match e {
            None => ModelError::FacesError,
            Some(err) => err.into(),
        })?;

    let full_sides = parse_or_default(&yml["full_sides"]);

    Ok(Model { faces, full_sides })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{
        super::{
            vertex::Vertex,
            sides::*,
        },
        parse_code,
    };
    use glm::{vec3, vec2};

    #[test]
    fn parse() {
        let code = r#"
        faces:
          - pos:
              - [ 1.0, 0.0, -1.0 ]
              - [ 1.0, 0.0, 1.0 ]
              - [ 1.0, 0.0, 1.0 ]
              - [ 1.0, 0.0, -1.0 ]

            st:
              - [ 0, 1 ]
              - [ 0, 0 ]
              - [ 1, 0 ]
              - [ 1, 1 ]

            norm: [ 0, 1, 0 ]
            layer: 6
            contact: lr

          - pos:
              - [ 1.0, 0.0, -1.0 ]
              - [ 1.0, 0.0, 1.0 ]
              - [ 1.0, 0.0, 1.0 ]

            st:
              - [ 0, 1 ]
              - [ 0, 0 ]
              - [ 1, 0 ]

            norm: [ 0, 1, 0 ]
            layer: 5
            contact: fb

        full_sides: ud
        "#;

        let a: Model = parse_code(code).unwrap();

        let x = Vertex {
            pos: vec3(1.0, 0.0, -1.0),
            st: vec2(0., 1.),
            norm: vec3(0., 1., 0.),
        };

        let y = Vertex {
            pos: vec3(1.0, 0.0, 1.0),
            st: vec2(0., 0.),
            norm: vec3(0., 1., 0.),
        };

        let z = Vertex {
            pos: vec3(1.0, 0.0, 1.0),
            st: vec2(1., 0.),
            norm: vec3(0., 1., 0.),
        };

        let w = Vertex {
            pos: vec3(1.0, 0.0, -1.0),
            st: vec2(1., 1.),
            norm: vec3(0., 1., 0.),
        };

        let b = Model {
            faces: vec![
                Face {
                    vertexes: FaceVertexes::Square([x, y, z, w]),
                    contact: Left | Right,
                    layer: 6,
                },
                Face {
                    vertexes: FaceVertexes::Triangle([x, y, z]),
                    contact: Front | Back,
                    layer: 5,
                },
            ],
            full_sides: Up | Down,
        };

        assert_eq!(a, b);
    }
}
