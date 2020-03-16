pub fn calc_normal_vec(a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) -> glm::Vec3 {
    (b - a).cross(&(c - a)).normalize()
}

pub fn calc_normal<A, B, C>(a: A, b: B, c: C) -> glm::Vec3
    where
        A: Into<glm::Vec3>,
        B: Into<glm::Vec3>,
        C: Into<glm::Vec3>,
{ calc_normal_vec(&a.into(), &b.into(), &c.into()) }
