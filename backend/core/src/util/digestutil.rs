use std::path::Path;

use chin_tools::AResult;

pub(crate) fn file_blake3_sum<P: AsRef<Path>>(file: P) -> AResult<String> {
    let mut hasher = blake3::Hasher::new();
    // TODO: use mmap method, `update_mmap_rayon`
    let hasher = hasher.update_reader(std::fs::File::open(&file)?)?;
    let hash = hasher.finalize().to_string();
    Ok(hash)
}
