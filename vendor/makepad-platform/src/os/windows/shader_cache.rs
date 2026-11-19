//! CrawRAD patch: an on-disk cache of compiled HLSL shaders.
//!
//! Makepad 1.0 runs `D3DCompile` for every shader at every start, and nothing
//! of it is kept: in the CrawRAD IDE that was about 13 of the 16.5 s before
//! the window appeared. Each compiled blob is now stored under
//! `%LOCALAPPDATA%\CrawRAD\shader-cache` (or `$MAKEPAD_SHADER_CACHE`), named by
//! a hash of the target and the source, and holding the full source, so a
//! hash collision reads as a miss rather than as the wrong shader.
//!
//! Every failure (no folder, a torn file, a full disk) falls back to
//! compiling, which is what Makepad did before.

use std::path::PathBuf;

/// Bumped when the file layout changes; old files then just miss.
const MAGIC: &[u8; 8] = b"CRAWHLS1";

fn cache_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("MAKEPAD_SHADER_CACHE") {
        return Some(PathBuf::from(dir));
    }
    let base = std::env::var_os("LOCALAPPDATA")?;
    Some(PathBuf::from(base).join("CrawRAD").join("shader-cache"))
}

/// FNV-1a, spelled out: the name must stay the same across Rust releases,
/// which `DefaultHasher` does not promise.
fn fnv1a(parts: &[&[u8]]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for part in parts {
        for byte in part.iter() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn entry_path(target: &str, source: &str) -> Option<PathBuf> {
    let hash = fnv1a(&[target.as_bytes(), source.as_bytes()]);
    Some(cache_dir()?.join(format!("{:016x}.bin", hash)))
}

/// `MAGIC`, the source length as 8 little-endian bytes, the source, the blob.
fn encode(source: &str, blob: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(MAGIC.len() + 8 + source.len() + blob.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(source.len() as u64).to_le_bytes());
    out.extend_from_slice(source.as_bytes());
    out.extend_from_slice(blob);
    out
}

/// The blob, when `bytes` is an entry for exactly this `source`.
fn decode<'a>(bytes: &'a [u8], source: &str) -> Option<&'a [u8]> {
    let rest = bytes.strip_prefix(MAGIC.as_slice())?;
    let (len, rest) = rest.split_at_checked(8)?;
    let len = u64::from_le_bytes(len.try_into().ok()?) as usize;
    let (stored, blob) = rest.split_at_checked(len)?;
    (stored == source.as_bytes() && !blob.is_empty()).then_some(blob)
}

/// The compiled bytes stored for `source` built for `target`, if any.
pub fn load(target: &str, source: &str) -> Option<Vec<u8>> {
    let bytes = std::fs::read(entry_path(target, source)?).ok()?;
    decode(&bytes, source).map(<[u8]>::to_vec)
}

/// Keeps `blob` for the next start. Written beside the entry and renamed
/// over it, so two IDEs starting together never read half a file.
pub fn store(target: &str, source: &str, blob: &[u8]) {
    let Some(path) = entry_path(target, source) else { return };
    let Some(dir) = path.parent() else { return };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    if std::fs::write(&temp, encode(source, blob)).is_ok() && std::fs::rename(&temp, &path).is_err() {
        let _ = std::fs::remove_file(&temp);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn an_entry_reads_back_only_for_its_own_source() {
        let bytes = encode("float4 main()", &[1, 2, 3]);
        assert_eq!(decode(&bytes, "float4 main()"), Some(&[1u8, 2, 3][..]));
        assert_eq!(decode(&bytes, "float4 other()"), None, "a collision is a miss");
        assert_eq!(decode(&bytes[..bytes.len() - 3], "float4 main()"), None, "no blob, no hit");
        assert_eq!(decode(&bytes[..10], "float4 main()"), None, "a torn file is a miss");
        assert_eq!(decode(b"something else", "float4 main()"), None);
    }

    #[test]
    fn the_name_depends_on_target_and_source() {
        let a = fnv1a(&[b"vs_5_0", b"src"]);
        assert_eq!(a, fnv1a(&[b"vs_5_0", b"src"]));
        assert_ne!(a, fnv1a(&[b"ps_5_0", b"src"]));
        assert_ne!(fnv1a(&[b"ab", b"c"]), fnv1a(&[b"a", b"bc"]), "parts are kept apart");
    }

    #[test]
    fn store_then_load_round_trips_through_the_folder() {
        let dir = std::env::temp_dir().join(format!("crawrad-shader-cache-{}", std::process::id()));
        std::env::set_var("MAKEPAD_SHADER_CACHE", &dir);
        assert_eq!(load("vs_5_0", "src"), None);
        store("vs_5_0", "src", &[9, 8, 7]);
        assert_eq!(load("vs_5_0", "src"), Some(vec![9, 8, 7]));
        assert_eq!(load("ps_5_0", "src"), None);
        std::env::remove_var("MAKEPAD_SHADER_CACHE");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
