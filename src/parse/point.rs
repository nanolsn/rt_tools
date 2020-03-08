use super::{
    super::point::Point,
    Parse,
};

pub fn yaml_to_point(yml: &yaml::Yaml) -> Result<Point, ()> {
    let arr = yml.as_vec().ok_or(())?;

    if arr.len() != 3 { Err(())? }
    if !arr.iter().all(|y| match y {
        yaml::Yaml::Integer(_) => true,
        _ => false,
    }) { Err(())? }

    let res = arr
        .iter()
        .map(|y| i32::parse(y).unwrap())
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse_code;

    #[test]
    fn parse() {
        let code = "{}";
        assert!(parse_code::<Point, _>(code).is_err());

        let code = "[0, 1]";
        assert!(parse_code::<Point, _>(code).is_err());

        let code = "[0, 1, 2, 3]";
        assert!(parse_code::<Point, _>(code).is_err());

        let code = "[0, 1.5, true]";
        assert!(parse_code::<Point, _>(code).is_err());

        let code = "[0, 2, -12]";
        let a: Point = parse_code(code).unwrap();
        let b = Point(0, 2, -12);

        assert_eq!(a, b);
    }
}
