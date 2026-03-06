use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::error::{IoContext, Result};

/// Magic bytes appended at the very end of the executable
/// to identify that a JS payload is embedded.
const MAGIC: &[u8; 8] = b"FABELA01";

/// Total trailer size: 8 bytes (payload_size as u64) + 8 bytes (magic)
const TRAILER_SIZE: u64 = 16;

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

    pub fn extract_embedded_source() -> Result<Option<String>> {
        let exe_path = std::env::current_exe().io_context("getting current executable path")?;
        let mut file = File::open(&exe_path)
            .io_context(format!("opening executable '{}'", exe_path.display()))?;
        Self::extract_from_file(&mut file)
    }

    pub fn extract_from_file(file: &mut File) -> Result<Option<String>> {
        let file_len = file
            .metadata()
            .io_context("reading executable metadata")?
            .len();

        if file_len < TRAILER_SIZE {
            return Ok(None);
        }

        let trailer = Self::read_trailer(file)?;

        if trailer.magic != *MAGIC {
            return Ok(None);
        }

        let payload_offset = file_len
            .checked_sub(TRAILER_SIZE + trailer.payload_size)
            .ok_or_else(|| crate::error::FabelaError::Compile("invalid payload size".into()))?;

        let compressed = Self::read_payload(file, payload_offset, trailer.payload_size)?;
        let decompressed =
            zstd::decode_all(compressed.as_slice()).io_context("decompressing zstd payload")?;

        let source = String::from_utf8(decompressed)?;

        Ok(Some(source))
    }

    fn read_trailer(file: &mut File) -> Result<Trailer> {
        file.seek(SeekFrom::End(-(TRAILER_SIZE as i64)))
            .io_context("seeking trailer")?;

        let mut buf = [0u8; TRAILER_SIZE as usize];
        file.read_exact(&mut buf).io_context("reading trailer")?;

        let payload_size = u64::from_le_bytes(
            buf[0..8]
                .try_into()
                .map_err(|_| crate::error::FabelaError::Compile("invalid trailer".into()))?,
        );

        let mut magic = [0u8; 8];
        magic.copy_from_slice(&buf[8..16]);

        Ok(Trailer {
            payload_size,
            magic,
        })
    }

    fn read_payload(file: &mut File, offset: u64, size: u64) -> Result<Vec<u8>> {
        file.seek(SeekFrom::Start(offset))
            .io_context("seeking payload start")?;

        let mut buf = vec![0u8; size as usize];
        file.read_exact(&mut buf).io_context("reading payload")?;

        Ok(buf)
    }
}

struct Trailer {
    payload_size: u64,
    magic: [u8; 8],
}
