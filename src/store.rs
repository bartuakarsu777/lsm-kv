use crate::error::Result;
use std::collections::BTreeMap;
use std::fs;
use std::ops::{Bound, RangeBounds};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct StoreOptions {
    pub data_dir: PathBuf,
    pub memtable_size: usize,
}

impl StoreOptions {
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            data_dir: data_dir.as_ref().to_path_buf(),
            memtable_size: 4 * 1024 * 1024,
        }
    }

    #[must_use]
    pub fn memtable_size(mut self, bytes: usize) -> Self {
        self.memtable_size = bytes;
        self
    }
}

pub struct SliceRange<'a>{
    start: Bound<&'a [u8]>,
    end: Bound<&'a [u8]>
}

impl<'a> RangeBounds<[u8]> for SliceRange<'a>{
    fn start_bound(&self) -> Bound<&[u8]>{
        self.start
    }

    fn end_bound(&self) -> Bound<&[u8]>{
        self.end
    }
}

pub struct Store {
    options: StoreOptions,
    map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Store {
    pub fn open(options: StoreOptions) -> Result<Self> {
        fs::create_dir_all(&options.data_dir)?;
        Ok(Self {
            options,
            map: BTreeMap::new(),
        })
    }

    pub fn put(&mut self, key: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Result<()> {
        self.map.insert(key.into(), value.into());
        Ok(())
    }

    pub fn get(&mut self, key: &[u8]) -> Result<bool> {
        Ok(self.map.remove(key).is_some())
    }

    pub fn scan<'a>(
        &'a self,
        start: Bound<&'a [u8]>,
        end: Bound<&'a [u8]>,
    ) -> impl Iterator<Item = (&'a [u8], &'a [u8])> + 'a{
        self.map.range(SliceRange{start, end}).map(|(k, v)| (k.as_slice(), v.as_slice()))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn options(&self) -> &StoreOptions {
        &self.options
    }
}