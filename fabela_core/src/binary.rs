use std::fs::File;
use std::path::Path;

use crate::error::IoContext;

pub struct BinaryOptions<'a> {
    pub file: File,
    pub entrypoint: &'a Path,
}
pub struct Binary;

impl Binary {
    pub fn bundle() {}
    pub fn extract_embedded_source() -> crate::error::Result<Option<String>> {
      let exe_path = std::env::current_exe().io_context("Obteniendo ruta del ejecutable actual")?;
      let file = File::open(&exe_path).io_context(format!("Abriendo ejecutable '{}'", exe_path.display()))?;
      let file_len = file.metadata().io_context("Leyendo metadata del ejecutable")?.len();

      println!("wtf is this {file_len}");

      let source = format!("soon");
      Ok(Some(source))
    }

}

// fn write_binary_bytes(
//     mut file_writer: File,
//     original_binary: Vec<u8>,
//     data_section_bytes: Vec<u8>,
// ) -> Result<(), Box<dyn Error>> {
//     if cfg!(windows) {
//         let pe = libsui::PortableExecutable::from(&original_binary)?;
//         pe.write_resource("hello.txt", data_section_bytes)?
//             .build(&mut file_writer)?;
//     }
//     Ok(())
// }
