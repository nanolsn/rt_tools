use super::{
    resource::Resource,
    load::Load,
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

    fn load<P>(file: P) -> Self
        where
            P: AsRef<std::path::Path>,
    {
        TexturePath(
            file
                .as_ref()
                .to_string_lossy()
                .into()
        )
    }
}

#[derive(Debug, Default)]
pub struct Atlas {
    images: Resource<TexturePath>,
}

impl Atlas {
    pub fn new() -> Self { Atlas::default() }

    pub fn add<S>(&mut self, file: S) -> usize
        where
            S: Into<String>,
    { self.images.load(file) }

    pub fn stitch_sprites(self, size: u32) -> Result<image::DynamicImage, AtlasError> {
        self.stitch(size, image::open)
    }

    pub fn stitch<F>(self, size: u32, mut f: F) -> Result<image::DynamicImage, AtlasError>
        where
            F: FnMut(String) -> image::ImageResult<image::DynamicImage>,
    {
        use image::{GenericImageView, GenericImage};

        let sprites_side = (self.images.len() as f32).sqrt().ceil() as u32;
        let texture_size = sprites_side * size;
        let mut map = image::DynamicImage::new_rgba8(texture_size, texture_size);

        let it = self
            .images
            .into_iter()
            .map(|file| {
                let img = f(file.0)?;

                if img.width() != size || img.height() != size {
                    Err(AtlasError::IncorrectSpriteSize)
                } else {
                    Ok(img)
                }
            });

        for (i, res) in it.enumerate() {
            let img = res?;
            let x = (i as u32 % sprites_side) * size;
            let y = (i as u32 / sprites_side) * size;
            map.copy_from(&img, x, y)?;
        }

        Ok(map)
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

        let mut atlas = Atlas::new();
        atlas.add("red");
        atlas.add("green");
        atlas.add("blue");
        atlas.add("red");
        atlas.add("green");
        atlas.add("blue");
        atlas.add("white");

        let img = atlas
            .stitch(SIZE, make_texture)
            .unwrap();

        assert_eq!(img.get_pixel(0, 0), image::Rgba(RED));
        assert_eq!(img.get_pixel(SIZE - 1, SIZE - 1), image::Rgba(RED));
        assert_eq!(img.get_pixel(SIZE, 0), image::Rgba(GREEN));
        assert_eq!(img.get_pixel(0, SIZE), image::Rgba(BLUE));
        assert_eq!(img.get_pixel(SIZE, SIZE), image::Rgba(WHITE));

        assert_eq!(img.width(), SIZE * 2);
        assert_eq!(img.height(), SIZE * 2);
    }
}
