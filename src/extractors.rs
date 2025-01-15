mod tar;
mod zip;

use crate::{MeisterError, Result};
use std::{ffi::OsStr, path::PathBuf};

trait Extractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>>;

    #[cfg(test)]
    fn format(&self) -> Format;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    TarZstd,
    Zip,
}

pub fn list_entries(archive_file: PathBuf) -> Result<Vec<String>> {
    match find_format(archive_file.clone()) {
        Ok(format) => match build_extractor(format) {
            Ok(extractor) => extractor.list_entries(archive_file),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn build_extractor(format: Format) -> Result<Box<dyn Extractor>> {
    use tar::{TarBz2Extractor, TarExtractor, TarGzExtractor, TarXzExtractor, TarZstdExtractor};
    use zip::ZipExtractor;

    match format {
        Format::Tar => Ok(Box::new(TarExtractor {})),
        Format::TarGz => Ok(Box::new(TarGzExtractor {})),
        Format::TarBz2 => Ok(Box::new(TarBz2Extractor {})),
        Format::TarXz => Ok(Box::new(TarXzExtractor {})),
        Format::TarZstd => Ok(Box::new(TarZstdExtractor {})),
        Format::Zip => Ok(Box::new(ZipExtractor {})),
    }
}

fn find_format(path: PathBuf) -> Result<Format> {
    find_format_impl(path.file_name())
}

fn find_format_impl(file_name: Option<&OsStr>) -> Result<Format> {
    match file_name {
        Some(file_name) => {
            let name = file_name.to_str().unwrap().to_lowercase();
            for ext in exts().iter() {
                if name.ends_with(&ext.1) {
                    return Ok(ext.0.clone());
                }
            }
            return Err(MeisterError::UnsupportedArchiveFormat(
                file_name.to_str().unwrap().to_string(),
            ));
        }
        None => Err(MeisterError::NoProjectSpecified()),
    }
}

pub(super) fn exts() -> Vec<(Format, String)> {
    vec![
        (Format::Tar, String::from(".tar")),
        (Format::TarGz, String::from(".tar.gz")),
        (Format::TarGz, String::from(".tgz")),
        (Format::TarBz2, String::from(".tar.bz2")),
        (Format::TarBz2, String::from(".tbz2")),
        (Format::TarXz, String::from(".tar.xz")),
        (Format::TarXz, String::from(".txz")),
        (Format::TarZstd, String::from(".tar.zst")),
        (Format::TarZstd, String::from(".tzst")),
        (Format::Zip, String::from(".zip")),
        (Format::Zip, String::from(".jar")),
        (Format::Zip, String::from(".war")),
        (Format::Zip, String::from(".ear")),
    ]
}
