use glm::{Vec2, vec2};

pub struct SpriteMap {
    map: image::DynamicImage,
    map_size: u32,
}

impl SpriteMap {
    pub fn new(map: image::DynamicImage, map_size: u32) -> Self { SpriteMap { map, map_size } }

    pub fn map(&self) -> &image::DynamicImage { &self.map }

    pub fn st(&self, sprite: u32, src: Vec2) -> Vec2 {
        self.st_iter(sprite, [src].iter()).next().unwrap()
    }

    pub fn st_iter<'a, I>(&self, sprite: u32, src: I) -> impl Iterator<Item=Vec2> + 'a
        where
            I: IntoIterator<Item=&'a Vec2> + 'a,
    {
        let inv_size = 1. / self.map_size as f32;
        let x = sprite % self.map_size;
        let y = sprite / self.map_size;

        src
            .into_iter()
            .map(move |s| (s + vec2(x as f32, y as f32)) * inv_size)
            .map(flip_t)
    }
}

fn flip_t(v: Vec2) -> Vec2 { vec2(v.x, 1. - v.y) }

#[cfg(test)]
mod tests {
    use super::*;

    const SPRITE_SIZE: u32 = 8;
    const MAP_SIZE: u32 = 3;

    #[test]
    fn get_st() {
        let size = SPRITE_SIZE * MAP_SIZE;
        let img = image::DynamicImage::new_rgba8(size, size);

        let m = SpriteMap::new(img, MAP_SIZE);

        assert_eq!(m.st(0, vec2(0., 0.)), flip_t(vec2(0., 0.)));
        assert_eq!(m.st(0, vec2(1., 1.)), flip_t(vec2(1. / 3., 1. / 3.)));

        assert_eq!(m.st(1, vec2(0., 0.)), flip_t(vec2(1. / 3., 0.)));
        assert_eq!(m.st(1, vec2(1., 1.)), flip_t(vec2(2. / 3., 1. / 3.)));

        assert_eq!(m.st(3, vec2(0., 0.)), flip_t(vec2(0., 1. / 3.)));
        assert_eq!(m.st(3, vec2(1., 1.)), flip_t(vec2(1. / 3., 2. / 3.)));

        assert_eq!(m.st(8, vec2(0., 0.)), flip_t(vec2(2. / 3., 2. / 3.)));
        assert_eq!(m.st(8, vec2(1., 1.)), flip_t(vec2(1., 1.)));

        assert_eq!(m.st(0, vec2(0.5, 0.5)), flip_t(vec2(1. / 6., 1. / 6.)));
        assert_eq!(m.st(1, vec2(0.5, 0.5)), flip_t(vec2(3. / 6., 1. / 6.)));
        assert_eq!(m.st(3, vec2(0.5, 0.5)), flip_t(vec2(1. / 6., 3. / 6.)));
    }

    #[test]
    fn get_sts() {
        let size = SPRITE_SIZE * MAP_SIZE;
        let img = image::DynamicImage::new_rgba8(size, size);

        let m = SpriteMap::new(img, MAP_SIZE);
        let vs = [vec2(0., 0.), vec2(1., 1.)];

        let mut it = m.st_iter(0, &vs);
        assert_eq!(it.next(), Some(flip_t(vec2(0., 0.))));
        assert_eq!(it.next(), Some(flip_t(vec2(1. / 3., 1. / 3.))));

        let mut it = m.st_iter(1, &vs);
        assert_eq!(it.next(), Some(flip_t(vec2(1. / 3., 0.))));
        assert_eq!(it.next(), Some(flip_t(vec2(2. / 3., 1. / 3.))));
    }
}
