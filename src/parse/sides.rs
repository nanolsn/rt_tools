use super::super::sides::Sides;

pub fn yaml_to_sides(yml: &yaml::Yaml) -> Sides {
    yml
        .as_str()
        .unwrap_or_default()
        .into()
}
