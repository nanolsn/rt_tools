use super::{
    load::{Load, LoadDir},
    resource::Resource,
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

impl LoadDir for TexturePath {
    const DIR: &'static str = "textures";
}

impl Load for TexturePath {
    type Error = ();

    fn load<P>(file: P) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
    {
        Ok(TexturePath(
            file
                .as_ref()
                .to_string_lossy()
                .into()
        ))
    }
}

type AtlasResult = Result<image::DynamicImage, AtlasError>;

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

    pub fn stitch_sprites(self) -> AtlasResult { self.stitch(image::open) }

    pub fn stitch<F>(self, mut f: F) -> AtlasResult
        where
            F: FnMut(String) -> image::ImageResult<image::DynamicImage>,
    {
        use image::{GenericImage, GenericImageView};

        let Atlas { images, size } = self;
        let sprites_side = (images.len() as f32).sqrt().ceil() as u32;
        let map_size = sprites_side * size;
        let mut map = image::DynamicImage::new_rgba8(map_size, map_size);

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

        let mut atlas = Atlas::new(SIZE);
        atlas.add("red");
        atlas.add("green");
        atlas.add("blue");
        atlas.add("red");
        atlas.add("green");
        atlas.add("blue");
        atlas.add("white");

        let img = atlas
            .stitch(make_texture)
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
