use std::{
    collections::{HashMap, hash_map::Entry},
    path::Path,
};

pub trait Load {
    const DIR: &'static str;

    fn load<P>(file: P) -> Self
        where
            P: AsRef<Path>;
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
    pub fn new() -> Self { Resource::default() }

    pub fn load<S>(&mut self, file: S) -> &T
        where
            S: Into<String>,
    {
        let id = match self.files.entry(file.into()) {
            Entry::Occupied(en) => *en.get(),
            Entry::Vacant(en) => {
                let path = Path::new(T::DIR).join(en.key());
                let item = T::load(path);

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

    pub fn len(&self) -> usize { self.items.len() }

    pub fn iter(&self) -> std::slice::Iter<T> { self.items.iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> { self.items.iter_mut() }
}

impl<T> Default for Resource<T> {
    fn default() -> Self { Resource { items: Vec::new(), files: HashMap::new() } }
}

impl<T> AsRef<[T]> for Resource<T> {
    fn as_ref(&self) -> &[T] { &*self.items }
}

impl<T> AsMut<[T]> for Resource<T> {
    fn as_mut(&mut self) -> &mut [T] { &mut *self.items }
}

impl<T> Into<Vec<T>> for Resource<T> {
    fn into(self) -> Vec<T> { self.items }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Tile {
        name: String,
    }

    impl Load for Tile {
        const DIR: &'static str = "tiles";

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

        assert_eq!(res.len(), 3);

        let files = ["one.tl", "two.tl", "three.tl"];
        assert!(
            res
                .iter()
                .all(|t| files.contains(&t.name.as_str()))
        );
    }
}
