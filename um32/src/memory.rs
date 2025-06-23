use std::fs::File;
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
        self.arrays[0] = Some(array0);
    }
    pub fn alloc(&mut self, size: Platter) -> Platter {
        let idx = self.free_indices.iter().next().unwrap_or_else(||self.arrays.next_index());
        self.arrays.insert(idx, IndexVec::from_elem_n(0, size as usize));
        self.free_indices.remove(idx);
        idx
    }
    pub fn free(&mut self, idx: Platter) {
        self.arrays.remove(idx);
        self.free_indices.insert(idx);
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
