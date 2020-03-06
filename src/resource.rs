use std::collections::HashMap;
use std::path::Path;
use std::collections::hash_map::Entry;

pub trait Load {
    fn load<P>(file: P) -> Self
        where
            P: AsRef<Path>;

    fn dir() -> &'static str;
}

#[derive(Debug)]
pub struct Resource<T> {
    items: Vec<T>,
    files: HashMap<String, usize>,
}

impl<T> Resource<T>
    where
        T: Load,
{
    pub fn new() -> Self { Resource { items: Vec::new(), files: HashMap::new() } }

    pub fn load<S>(&mut self, file: S) -> &T
        where
            S: Into<String>,
    {
        let file = file.into();
        let path = Path::new(T::dir()).join(&*file);
        let item = T::load(path);

        let id = match self.files.entry(file) {
            Entry::Occupied(en) => {
                *en.get()
            }
            Entry::Vacant(en) => {
                let id = self.items.len();
                self.items.push(item);
                en.insert(id);
                id
            }
        };

        &self.items[id]
    }

    pub fn get(&self, id: usize) -> Option<&T> { self.items.get(id) }

    pub fn get_file<S>(&self, file: S) -> Option<&T>
        where
            S: Into<String>,
    {
        self.files
            .get(&file.into())
            .and_then(|&id| self.get(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Tile {
        name: String,
    }

    impl Load for Tile {
        fn load<P>(file: P) -> Self
            where
                P: AsRef<Path>,
        {
            Tile {
                name: file
                    .as_ref()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into()
            }
        }

        fn dir() -> &'static str { "tiles" }
    }

    #[test]
    fn load() {
        let mut res: Resource<Tile> = Resource::new();
        res.load("one.tl");
        res.load("two.tl");
        res.load("two.tl");
        res.load("one.tl");
        res.load("one.tl");
        res.load("three.tl");

        assert_eq!(res.get(0).unwrap().name, "one.tl");
        assert_eq!(res.get(1).unwrap().name, "two.tl");
        assert_eq!(res.get(2).unwrap().name, "three.tl");
        assert!(res.get(3).is_none());

        assert_eq!(res.get_file("one.tl").unwrap().name, "one.tl");
        assert_eq!(res.get_file("two.tl").unwrap().name, "two.tl");
        assert_eq!(res.get_file("three.tl").unwrap().name, "three.tl");
        assert!(res.get_file("four.tl").is_none());
    }
}
