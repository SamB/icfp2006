use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::io::Error;

use byteorder::*;

pub type Array = Vec<u32>;

pub fn array_from_file<P: AsRef<Path>>(path: P) -> Result <Array, Error> {
    let mut f = File::open(path)?;
    let platters = (f.metadata()?.size() / 4).try_into().expect("file needs to fit virtual address space");

    let mut array = Array::new();
    array.resize(platters, 0);
    f.read_u32_into::<BigEndian>(array.as_mut_slice())?;

    Ok(array)
}
