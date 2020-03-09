use super::{
    super::state::*,
    parse,
};
use crate::model::Model;
use crate::resource::Resource;

pub fn yaml_to_state<'m>(yml: &yaml::Yaml, res: &'m Resource<Model>) -> Result<State<'m>, StateError> {
    let model_id: usize = parse(&yml["model"]).unwrap();

    let _model = res.get(model_id).ok_or(StateError::NoModelDefined)?;

    todo!()
}
