use std::fs::File;
use std::ops::{Index, IndexMut};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::io::Result;

use byteorder::*;
use ra_ap_rustc_index::bit_set::MixedBitSet;
use ra_ap_rustc_index::IndexVec;

pub type Platter = u32;
pub type Array = IndexVec<Platter, Platter>;

pub struct Memory {
    arrays: IndexVec<Platter, Option<Array>>,
    free_indices: MixedBitSet<Platter>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            arrays: IndexVec::new(),
            free_indices: MixedBitSet::new_empty(0x1_0000_0000),
        }
    }
    pub fn load(&mut self, array0: Array) {
        self.arrays.insert(0, array0);
    }
    pub fn alloc(&mut self, size: Platter) -> Platter {
        let aid = self.free_indices.iter().next().unwrap_or_else(||self.arrays.next_index());
        self.arrays.insert(aid, IndexVec::from_elem_n(0, size as usize));
        self.free_indices.remove(aid);
        aid
    }
    pub fn free(&mut self, aid: Platter) {
        self.arrays.remove(aid);
        self.free_indices.insert(aid);
    }
    pub fn read(&self, aid: Platter, idx: Platter) -> Platter {
        self.arrays[aid].as_ref().expect("array should exist")[idx]
    }
    pub fn write(&mut self, aid: Platter, idx: Platter, val: Platter) {
        self.arrays[aid].as_mut().expect("array should exist")[idx] = val;
    }
}

impl Index<Platter> for Memory {
    type Output = Array;

    fn index(&self, index: Platter) -> &Self::Output {
        self.arrays.index(index).as_ref().expect("array should exist")
    }
}

impl IndexMut<Platter> for Memory {
    fn index_mut(&mut self, index: Platter) -> &mut Self::Output {
        self.arrays.index_mut(index).as_mut().expect("array should exist")
    }
}

pub fn array_from_file<P: AsRef<Path>>(path: P) -> Result <Array> {
    let mut f = File::open(path)?;
    let platters = (f.metadata()?.size() / 4).try_into().expect("file needs to fit virtual address space");

    let mut array = Array::new();
    array.resize(platters, 0);
    f.read_u32_into::<BigEndian>(&mut array.as_mut_slice().raw)?;

    Ok(array)
}
