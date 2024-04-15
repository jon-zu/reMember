use probly_search::{score::zero_to_one, Index};
use serde::Serialize;
use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub trait TextIndexable {
    type Key: Eq + PartialEq + std::hash::Hash + Clone + std::fmt::Debug;

    fn name<'a>(&'a self, key: &'a Self::Key) -> &'a str;
}

pub struct TextIndex<T: TextIndexable> {
    index: Index<usize>,
    keys: Vec<T::Key>,
}

fn tokenizer(s: &str) -> Vec<Cow<str>> {
    s.split(' ').map(|s| s.to_lowercase()).map(Cow::from).collect::<Vec<_>>()
}

struct Doc<'a, T: TextIndexable> {
    key: &'a T::Key,
    data: &'a T,
}

impl<'a, T: TextIndexable> Doc<'a, T> {
    fn new(key: &'a T::Key, data: &'a T) -> Self {
        Self { key, data }
    }

    fn name_extract(&self) -> Vec<&str> {
        vec![self.data.name(self.key)]
    }
}

pub struct QueryResult<'a, T: TextIndexable, V> {
    res: Vec<probly_search::QueryResult<usize>>,
    ix: usize,
    keys: &'a [T::Key],
    values: &'a V,
}

impl<'a, T: TextIndexable + 'a, V: std::ops::Index<&'a T::Key>> Iterator for QueryResult<'a, T, V> 
    where V::Output: 'a
{
    type Item = (&'a T::Key, &'a V::Output, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix < self.res.len() {
            let qres = &self.res[self.ix];
            let key = &self.keys[qres.key];
            let value = &self.values[key];
            self.ix += 1;
            Some((key, value, qres.score))
        } else {
            None
        }
    }
}

impl<T: TextIndexable> TextIndex<T> {
    pub fn build<'a>(data: impl Iterator<Item = (&'a T::Key, &'a T)>) -> Self
    where
        T: 'a,
    {
        let mut index = Index::new(1);
        let mut keys = Vec::new();
        for (i, (key, data)) in data.enumerate() {
            let doc = Doc::new(key, data);
            index.add_document(&[Doc::name_extract], tokenizer, i, &doc);
            keys.push(key.clone());
        }

        Self { index, keys }
    }

    pub fn query<'a, V: std::ops::Index<&'a T::Key>>(
        &'a self,
        q: &'a str,
        v: &'a V,
    ) -> QueryResult<'a, T, V> {
        let res = self
            .index
            .query(q, &mut zero_to_one::new(), tokenizer, &[1.]);
        QueryResult {
            res,
            ix: 0,
            keys: &self.keys,
            values: v,
        }
    }
}

pub struct SearchMap<V: TextIndexable> {
    map: HashMap<V::Key, V>,
    index: TextIndex<V>,
}

impl<V: TextIndexable + std::fmt::Debug> std::fmt::Debug for SearchMap<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchMap").field("map", &self.map).finish()
    }
}

impl<V: TextIndexable> Serialize for SearchMap<V>
where
    V::Key: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.map.serialize(serializer)
    }
}

impl<'de, V: TextIndexable> serde::Deserialize<'de> for SearchMap<V>
where
    V::Key: serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<V::Key, V>::deserialize(deserializer)?;
        Ok(Self::build(map))
    }
}

impl<V: TextIndexable> Deref for SearchMap<V> {
    type Target = HashMap<V::Key, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<V: TextIndexable> DerefMut for SearchMap<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<V: TextIndexable> SearchMap<V> {
    pub fn build(map: HashMap<V::Key, V>) -> Self {
        let index = TextIndex::build(map.iter());
        Self { map, index }
    }

    pub fn query<'a, 'b>(&'a self, q: &'a str) -> QueryResult<'a, V, HashMap<V::Key, V>>
    where
        'a: 'b,
    {
        self.index.query(q, &self.map)
    }

    pub fn get_or_query<'a, 'b>(
        &'a self,
        q: &'a str,
    ) -> Result<&'b V, QueryResult<'a, V, HashMap<V::Key, V>>>
    where
        'a: 'b,
        'b: 'a,
        V::Key: Borrow<str>,
    {
        self.map
            .get(q)
            .map(Ok)
            .unwrap_or_else(|| Err(self.query(q)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct Item {
        id: usize,
        name: String,
    }

    impl Item {
        pub fn new(id: usize, name: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
            }
        }
    }

    impl TextIndexable for Item {
        type Key = usize;

        fn name(&self, _key: &Self::Key) -> &str {
            &self.name
        }
    }

    /*#[test]
    fn search() {
        let items = vec![
            Item::new(0, "apple"),
            Item::new(1, "banana"),
            Item::new(2, "orange"),
            Item::new(3, "orangee"),
            Item::new(4, "grape"),
            Item::new(5, "kiwi"),
        ];

        let index = TextIndex::build(items.iter().map(|item| (&item.id, item)));
        let results: Vec<_> = index.query("bana", &items).collect();
        assert_eq!(results.len(), 1);
    }*/

    #[test]
    fn search_map() {
        let items = vec![
            Item::new(0, "apple"),
            Item::new(1, "banana"),
            Item::new(2, "orange"),
            Item::new(3, "orangee"),
            Item::new(4, "grape"),
            Item::new(5, "kiwi"),
        ];

        let mut map = HashMap::new();
        for item in items {
            map.insert(item.id, item);
        }

        let search_map = SearchMap::build(map);
        let results: Vec<_> = search_map.query("bana").collect();
        assert_eq!(results.len(), 1);
    }
}
