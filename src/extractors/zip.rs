use std::fs::File;
use std::path::PathBuf;

use crate::extractors::Extractor;
use crate::Result;

#[cfg(test)]
use crate::extractors::Format;

pub(super) struct ZipExtractor {}

impl Extractor for ZipExtractor {
    fn list_entries(&self, archive_file: PathBuf) -> Result<Vec<String>> {
        let zip_file = File::open(archive_file).unwrap();
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();

        let mut result = Vec::<String>::new();
        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap();
            result.push(file.name().to_string());
            // std::io::copy(&mut file, &mut std::io::stdout()).unwrap();
        }
        Ok(result)
    }

    #[cfg(test)]
    fn format(&self) -> Format {
        Format::Zip
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_list_archives() {
        let extractor = ZipExtractor {};
        let file = PathBuf::from("testdata/fibonacci.zip");
        match extractor.list_entries(file) {
            Ok(r) => {
                assert_eq!(r.len(), 16);
                assert_eq!(r.get(6), Some("fibonacci/.gitignore".to_string()).as_ref());
                assert_eq!(
                    r.get(7),
                    Some("fibonacci/build.gradle".to_string()).as_ref()
                );
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_format() {
        let e = ZipExtractor {};
        assert_eq!(e.format(), Format::Zip);
    }
}
