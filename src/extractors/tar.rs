use std::io::Read;
use std::{fs::File, path::PathBuf};

use tar::Archive;
use xz2::read::XzDecoder;

use crate::extractors::Extractor;
use crate::{MeisterError, Result};

#[cfg(test)]
use crate::extractors::Format;

pub(super) struct TarExtractor {}
pub(super) struct TarGzExtractor {}
pub(super) struct TarBz2Extractor {}
pub(super) struct TarXzExtractor {}
pub(super) struct TarZstdExtractor {}

impl Extractor for TarExtractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        match open_tar_file(&archive_file, |f| f) {
            Ok(archive) => list_tar(archive),
            Err(e) => Err(e),
        }
    }

    #[cfg(test)]
    fn format(&self) -> Format {
        Format::Tar
    }
}

impl Extractor for TarGzExtractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        match open_tar_file(&archive_file, |f| flate2::read::GzDecoder::new(f)) {
            Ok(archive) => list_tar(archive),
            Err(e) => Err(e),
        }
    }

    #[cfg(test)]
    fn format(&self) -> Format {
        Format::TarGz
    }
}

impl Extractor for TarBz2Extractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        match open_tar_file(&archive_file, |f| bzip2::read::BzDecoder::new(f)) {
            Ok(archive) => list_tar(archive),
            Err(e) => Err(e),
        }
    }

    #[cfg(test)]
    fn format(&self) -> Format {
        Format::TarBz2
    }
}

impl Extractor for TarXzExtractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        match open_tar_file(&archive_file, |f| XzDecoder::new(f)) {
            Err(e) => Err(e),
            Ok(archive) => list_tar(archive),
        }
    }

    #[cfg(test)]
    fn format(&self) -> Format {
        Format::TarXz
    }
}

impl Extractor for TarZstdExtractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        match open_tar_file(&archive_file, |f| zstd::Decoder::new(f).unwrap()) {
            Err(e) => Err(e),
            Ok(archive) => list_tar(archive),
        }
    }

    #[cfg(test)]
    fn format(&self) -> Format {
        Format::TarZstd
    }
}

fn open_tar_file<F, R: Read>(file: &PathBuf, opener: F) -> Result<Archive<R>>
where
    F: FnOnce(File) -> R,
{
    let file = match File::open(file) {
        Ok(f) => f,
        Err(e) => return Err(MeisterError::IO(e)),
    };
    let writer = opener(file);
    Ok(Archive::new(writer))
}

fn list_tar<R: Read>(mut archive: tar::Archive<R>) -> Result<Vec<String>> {
    let mut result = Vec::<String>::new();
    for entry in archive.entries().unwrap() {
        let entry = entry.unwrap();
        let path = entry.header().path().unwrap();
        result.push(format!("{}", path.to_str().unwrap()));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tar_file() {
        let extractor = TarExtractor {};
        let file = PathBuf::from("testdata/hello.tar");
        match extractor.list_entries(file) {
            Ok(r) => {
                println!("{:?}", r);
                assert_eq!(r.len(), 2);
                assert_eq!(r.get(0), Some("hello/Cargo.toml".to_string()).as_ref());
                assert_eq!(r.get(1), Some("hello/src/main.rs".to_string()).as_ref());
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_list_tarbz2_file() {
        let extractor = TarBz2Extractor {};
        let file = PathBuf::from("testdata/hello.tar.bz2");
        match extractor.list_entries(file) {
            Ok(r) => {
                assert_eq!(r.len(), 2);
                assert_eq!(r.get(0), Some("hello/Cargo.toml".to_string()).as_ref());
                assert_eq!(r.get(1), Some("hello/src/main.rs".to_string()).as_ref());
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_list_targz_file() {
        let extractor = TarGzExtractor {};
        let file = PathBuf::from("testdata/hello.tar.gz");
        match extractor.list_entries(file) {
            Ok(r) => {
                assert_eq!(r.len(), 2);
                assert_eq!(r.get(0), Some("hello/Cargo.toml".to_string()).as_ref());
                assert_eq!(r.get(1), Some("hello/src/main.rs".to_string()).as_ref());
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_list_tarxz_file() {
        let extractor = TarXzExtractor {};
        let file = PathBuf::from("testdata/hello.tar.xz");
        match extractor.list_entries(file) {
            Ok(r) => {
                assert_eq!(r.len(), 2);
                assert_eq!(r.get(0), Some("hello/Cargo.toml".to_string()).as_ref());
                assert_eq!(r.get(1), Some("hello/src/main.rs".to_string()).as_ref());
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_list_tarzstd_file() {
        let extractor = TarZstdExtractor {};
        let file = PathBuf::from("testdata/hello.tar.zst");
        match extractor.list_entries(file) {
            Ok(r) => {
                assert_eq!(r.len(), 2);
                assert_eq!(r.get(0), Some("hello/Cargo.toml".to_string()).as_ref());
                assert_eq!(r.get(1), Some("hello/src/main.rs".to_string()).as_ref());
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_format() {
        let e1 = TarExtractor {};
        assert_eq!(e1.format(), Format::Tar);

        let e2 = TarGzExtractor {};
        assert_eq!(e2.format(), Format::TarGz);

        let e3 = TarBz2Extractor {};
        assert_eq!(e3.format(), Format::TarBz2);

        let e4 = TarXzExtractor {};
        assert_eq!(e4.format(), Format::TarXz);

        let e5 = TarZstdExtractor {};
        assert_eq!(e5.format(), Format::TarZstd);
    }
}
