use std::collections::{hash_map::Entry, HashMap};

use super::super::{
    get::{Get, GetMut},
    load::Load,
    asset::Asset,
};

type LoadResult<'a, T, E> = Result<(usize, &'a T), E>;

#[derive(Debug)]
pub struct Resource<T> {
    items: Vec<T>,
    files: HashMap<String, usize>,
}

impl<T> Resource<T> {
    pub fn new() -> Self { Resource::default() }

    pub fn load_with<S>(&mut self, file: S, loader: &mut T::Loader) -> LoadResult<T, T::Error>
        where
            S: Into<String>,
            T: Load,
    {
        let idx = match self.files.entry(file.into()) {
            Entry::Occupied(en) => *en.get(),
            Entry::Vacant(en) => {
                let item = T::load(en.key(), loader)?;
                let id = self.items.len();

                self.items.push(item);
                *en.insert(id)
            }
        };

        Ok((idx, &self.items[idx]))
    }

    pub fn load<S>(&mut self, file: S) -> LoadResult<T, T::Error>
        where
            S: Into<String>,
            T: Load<Loader=()>,
    { self.load_with(file, &mut ()) }

    pub fn load_asset_with<S>(&mut self, file: S, loader: &mut T::Loader) -> LoadResult<T, T::Error>
        where
            S: AsRef<str>,
            T: Asset,
    { self.load_with(T::full_path(file), loader) }

    pub fn load_asset<S>(&mut self, file: S) -> LoadResult<T, T::Error>
        where
            S: AsRef<str>,
            T: Asset<Loader=()>,
    { self.load_asset_with(file, &mut ()) }

    pub fn get<B>(&self, by: B) -> Option<&T>
        where
            Self: Get<B, Item=T>,
    { Get::get(self, by) }

    pub fn get_mut<B>(&mut self, by: B) -> Option<&mut T>
        where
            Self: GetMut<B, Item=T>,
    { GetMut::get_mut(self, by) }

    pub fn len(&self) -> usize { self.items.len() }

    pub fn iter(&self) -> std::slice::Iter<T> { self.items.iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> { self.items.iter_mut() }
    pub fn into_iter(self) -> std::vec::IntoIter<T> { self.items.into_iter() }
}

impl<T> Get<usize> for Resource<T> {
    type Item = T;

    fn get(&self, by: usize) -> Option<&Self::Item> { self.items.get(by) }
}

impl<T> Get<&str> for Resource<T> {
    type Item = T;

    fn get(&self, by: &str) -> Option<&Self::Item> {
        let &id = self.files.get(by)?;
        self.get(id)
    }
}

impl<T> Get<String> for Resource<T> {
    type Item = T;

    fn get(&self, by: String) -> Option<&Self::Item> { self.get(by.as_str()) }
}

impl<T> GetMut<usize> for Resource<T> {
    type Item = T;

    fn get_mut(&mut self, by: usize) -> Option<&mut Self::Item> { self.items.get_mut(by) }
}

impl<T> GetMut<&str> for Resource<T> {
    type Item = T;

    fn get_mut(&mut self, by: &str) -> Option<&mut Self::Item> {
        let &id = self.files.get(by)?;
        self.get_mut(id)
    }
}

impl<T> GetMut<String> for Resource<T> {
    type Item = T;

    fn get_mut(&mut self, by: String) -> Option<&mut Self::Item> { self.get_mut(by.as_str()) }
}

impl<T> Default for Resource<T> {
    fn default() -> Self {
        Resource {
            items: Vec::new(),
            files: HashMap::new(),
        }
    }
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
    use std::rc::Rc;

    #[derive(Debug)]
    struct Tile(String);

    impl Load for Tile {
        type Error = ();
        type Loader = ();

        fn load<S>(file: S, _: &mut Self::Loader) -> Result<Self, Self::Error>
            where
                S: AsRef<str>,
        { Ok(Tile(file.as_ref().into())) }
    }

    #[test]
    fn load() {
        let mut res: Resource<Tile> = Resource::new();
        res.load("one").unwrap();
        res.load("two").unwrap();
        res.load("two").unwrap();
        res.load("one").unwrap();
        res.load("one").unwrap();
        res.load("three").unwrap();

        assert_eq!(res.get(0).unwrap().0, "one");
        assert_eq!(res.get(1).unwrap().0, "two");
        assert_eq!(res.get(2).unwrap().0, "three");
        assert!(res.get(3).is_none());

        assert_eq!(res.get("one").unwrap().0, "one");
        assert_eq!(res.get("two").unwrap().0, "two");
        assert_eq!(res.get("three").unwrap().0, "three");
        assert!(res.get("four").is_none());

        assert_eq!(res.len(), 3);

        let files = ["one", "two", "three"];
        assert!(
            res
                .iter()
                .all(|t| files.contains(&t.0.as_str()))
        );
    }

    #[derive(Debug)]
    struct TileSet {
        tiles: Vec<Rc<Tile>>,
    }

    impl Load for TileSet {
        type Error = ();
        type Loader = Resource<Rc<Tile>>;

        fn load<S>(file: S, loader: &mut Self::Loader) -> Result<Self, Self::Error>
            where
                S: AsRef<str>,
        {
            let tiles = file
                .as_ref()
                .split_whitespace()
                .map(|s| Rc::clone(loader.load(s).unwrap().1))
                .collect();

            Ok(TileSet { tiles })
        }
    }

    #[test]
    fn load_with() {
        let mut res: Resource<Rc<Tile>> = Resource::new();

        let ts: TileSet = TileSet::load("one two", &mut res).unwrap();
        assert_eq!(res.items[0].0, "one");
        assert_eq!(res.items[1].0, "two");
        assert!(ts
            .tiles
            .iter()
            .zip(["one", "two"].iter())
            .all(|(a, &b)| a.0.as_str() == b)
        );

        let ts: TileSet = TileSet::load("three one two", &mut res).unwrap();
        assert_eq!(res.items[0].0, "one");
        assert_eq!(res.items[1].0, "two");
        assert_eq!(res.items[2].0, "three");
        assert!(ts
            .tiles
            .iter()
            .zip(["three", "one", "two"].iter())
            .all(|(a, &b)| a.0.as_str() == b)
        );
    }
}
