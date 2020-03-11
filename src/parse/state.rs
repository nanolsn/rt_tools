use super::{
    super::{
        state::*,
        model::Model,
        resource::Resource,
        shell_transform::{Shell, ShellTransformAction, apply_actions},
    },
    parse,
};

pub fn yaml_to_state(yml: &yaml::Yaml, res: &mut Resource<Model>, models: &[String])
                     -> Result<State, StateError> {
    if models.len() == 0 { Err(StateError::NoModelDefined)? }

    let model_id: usize = parse(&yml["model"]).unwrap();
    let model_path = models.get(model_id).ok_or(StateError::NoModelDefined)?;

    let mut shell = Shell::new();
    let transform: Vec<ShellTransformAction> = parse(&yml["transform"])
        .map_err(|_| StateError::TransformError)?;

    Ok(State {
        model: res.load(model_path)?,
        shell: *apply_actions(&mut shell, transform),
        layers: parse(&yml["layers"]).unwrap(),
    })
}
