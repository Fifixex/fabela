use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::error::IoContext;

/// Magic bytes appended at the very end of the executable
/// to identify that a JS payload is embedded.
const MAGIC: &[u8; 8] = b"FABELA01";

/// Total trailer size: 8 bytes (payload_size as u64) + 8 bytes (magic)
const TRAILER_SIZE: usize = 16;

/// Compression level for zstd (1-22, 3 is default, good balance)
const ZSTD_LEVEL: i32 = 19;

pub struct BinaryOptions<'a> {
    pub file: File,
    pub entrypoint: &'a Path,
}
pub struct Binary;

impl Binary {
    /// Create a standalone executable by:
    /// 1. Copying the base fabela binary
    /// 2. Appending the JS source compressed with zstd
    /// 3. Appending a trailer with [payload_size: u64][magic: 8 bytes]

    /// (Not implemented yet)
    pub fn bundle() {
      println!("soon");
    }

    pub fn extract_embedded_source() -> crate::error::Result<Option<String>> {
      let exe_path = std::env::current_exe().io_context("getting current exe path")?;
      let mut file = File::open(&exe_path).io_context(format!("opening exe '{}'", exe_path.display()))?;
      let file_len = file.metadata().io_context("reading executable metadata")?.len();

      if file_len < TRAILER_SIZE as u64 {
          return Ok(None);
      }

      file.seek(SeekFrom::End(-(TRAILER_SIZE as i64))).io_context("trailer not found")?;
      let mut trailer = [0u8; TRAILER_SIZE];
      file.read_exact(&mut trailer).io_context("reading trailer exe")?;

      if &trailer[8..16] != MAGIC {
        return Ok(None);
      }
      let payload_size = u64::from_le_bytes(trailer[0..8].try_into().unwrap());
      let payload_offset = file_len - TRAILER_SIZE as u64 - payload_size;

      file.seek(SeekFrom::Start(payload_offset)).io_context("reading payload start")?;

      let mut compressed = vec![0u8; payload_size as usize];
      file.read_exact(&mut compressed).io_context("reading compressed payload")?;

      let decompressed = zstd::decode_all(compressed.as_slice()).io_context("decompressing zstd payload")?;
      let source = String::from_utf8(decompressed)?;

      Ok(Some(source))
    }

}
