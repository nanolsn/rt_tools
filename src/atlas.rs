use super::{
    load::Load,
    resource::Resource,
    sprite_map::SpriteMap,
};

#[derive(Debug)]
pub enum AtlasError {
    ImageError(image::ImageError),
    IncorrectSpriteSize,
}

impl From<image::ImageError> for AtlasError {
    fn from(err: image::ImageError) -> Self { AtlasError::ImageError(err) }
}

#[derive(Debug)]
struct TexturePath(String);

impl Load for TexturePath {
    const DIR: &'static str = "textures";

    type Error = ();
    type Loader = ();

    fn load<P>(file: P, _: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    {
        Ok(TexturePath(
            file
                .as_ref()
                .to_string_lossy()
                .into()
        ))
    }
}

#[derive(Debug)]
pub struct Atlas {
    images: Resource<TexturePath>,
    size: u32,
}

impl Atlas {
    pub fn new(size: u32) -> Self {
        Atlas {
            images: Resource::new(),
            size,
        }
    }

    pub fn add<S>(&mut self, file: S) -> Option<usize>
        where
            S: Into<String>,
    { self.images.load(file).ok() }

    pub fn stitch_sprites(self) -> Result<SpriteMap, AtlasError> { self.stitch(image::open) }

    pub fn stitch<F>(self, mut f: F) -> Result<SpriteMap, AtlasError>
        where
            F: FnMut(String) -> image::ImageResult<image::DynamicImage>,
    {
        use image::{GenericImage, GenericImageView};

        let Atlas { images, size } = self;
        let map_size = (images.len() as f32).sqrt().ceil() as u32;
        let pixel_size = map_size * size;
        let mut map = image::DynamicImage::new_rgba8(pixel_size, pixel_size);

        let it = images
            .into_iter()
            .map(|TexturePath(file)| {
                let img = f(file)?;

                if img.width() != size || img.height() != size {
                    Err(AtlasError::IncorrectSpriteSize)
                } else {
                    Ok(img)
                }
            });

        for (i, res) in it.enumerate() {
            let img = res?;
            let x = (i as u32 % map_size) * size;
            let y = (i as u32 / map_size) * size;
            map.copy_from(&img, x, y)?;
        }

        Ok(SpriteMap::new(map, map_size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 8;

    const RED: [u8; 4] = [255, 0, 0, 255];
    const GREEN: [u8; 4] = [0, 255, 0, 255];
    const BLUE: [u8; 4] = [0, 0, 255, 255];
    const WHITE: [u8; 4] = [255, 255, 255, 255];
    const BLACK: [u8; 4] = [0, 0, 0, 255];

    fn make_texture(s: String) -> image::ImageResult<image::DynamicImage> {
        let img: image::RgbaImage = image::ImageBuffer::from_fn(SIZE, SIZE, |_, _| {
            let color = match s.as_str() {
                _ if s.ends_with("red") => RED,
                _ if s.ends_with("green") => GREEN,
                _ if s.ends_with("blue") => BLUE,
                _ if s.ends_with("white") => WHITE,
                _ => BLACK,
            };

            image::Rgba(color)
        });

        Ok(image::DynamicImage::ImageRgba8(img))
    }

    #[test]
    fn stitch() {
        use image::GenericImageView;

        let mut atlas = Atlas::new(SIZE);
        atlas.add("red");
        atlas.add("green");
        atlas.add("blue");
        atlas.add("red");
        atlas.add("green");
        atlas.add("blue");
        atlas.add("white");

        let map = atlas
            .stitch(make_texture)
            .unwrap();

        let img = map.map();

        assert_eq!(img.get_pixel(0, 0), image::Rgba(RED));
        assert_eq!(img.get_pixel(SIZE - 1, SIZE - 1), image::Rgba(RED));
        assert_eq!(img.get_pixel(SIZE, 0), image::Rgba(GREEN));
        assert_eq!(img.get_pixel(0, SIZE), image::Rgba(BLUE));
        assert_eq!(img.get_pixel(SIZE, SIZE), image::Rgba(WHITE));

        assert_eq!(img.width(), SIZE * 2);
        assert_eq!(img.height(), SIZE * 2);
    }
}
