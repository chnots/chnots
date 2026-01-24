use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bytes::Bytes;
use flate2::Compression;
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use std::io::prelude::*;
use std::path::Path;

use chin_tools::AResult;
use tokio_util::bytes;

#[inline]
pub(crate) fn file_blake3_sum<P: AsRef<Path>>(file: P) -> AResult<String> {
    let mut hasher = blake3::Hasher::new();
    // TODO: use mmap method, `update_mmap_rayon`
    let hasher = hasher.update_reader(std::fs::File::open(&file)?)?;
    let hash = hasher.finalize().to_string();
    Ok(hash)
}

pub struct GzipdBase64Blake3 {
    pub blake3: String,
    pub b64: String,
}

#[inline]
pub(crate) fn gzip_base64_blake3(bytes: Bytes) -> AResult<GzipdBase64Blake3> {
    let mut enc = GzEncoder::new(Vec::new(), Compression::default());
    enc.write_all(&bytes)?;
    let compressed_bytes = enc.finish()?;
    let b64 = base64::Engine::encode(&BASE64_STANDARD, compressed_bytes);

    let mut hasher = blake3::Hasher::new();
    let hasher = hasher.update(&b64.as_bytes());
    let hash = hasher.finalize().to_string();

    Ok(GzipdBase64Blake3 { blake3: hash, b64 })
}

#[inline]
pub(crate) fn de_gzip_base64_blake3(gbb: GzipdBase64Blake3) -> AResult<bytes::Bytes> {
    let mut hasher = blake3::Hasher::new();
    let hasher = hasher.update(gbb.b64.as_bytes());
    let hash = hasher.finalize().to_string();
    if hash != gbb.blake3 {
        anyhow::bail!(
            "The stored blake3 value is not same to computed {} -- {}",
            hash,
            gbb.blake3
        );
    }

    let gbb = BASE64_STANDARD.decode(gbb.b64.as_bytes())?;

    let mut d = GzDecoder::new(gbb.as_slice());
    let mut decompressed = Vec::new();

    d.read_to_end(&mut decompressed)?;
    Ok(bytes::Bytes::from(decompressed))
}

#[inline]
pub(crate) fn blake3_sum(s: &str) -> AResult<String> {
    let mut hasher = blake3::Hasher::new();
    let hasher = hasher.update(s.as_bytes());
    let hash = hasher.finalize().to_string();
    Ok(hash)
}
