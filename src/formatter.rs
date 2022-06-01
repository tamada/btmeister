use std::io::Write;
use std::path::Path;

use super::btmeister::{BuildToolDef, BuildToolDefs};
use super::{BuildTool, Format};

pub trait Formatter {
    fn print(&self, out: &mut Box<dyn Write>, base: &Path, vector: Vec<BuildTool>) -> i32 {
        self.print_header(out, base);
        vector.iter().enumerate()
                .for_each(|item| self.print_each(out, item.1, item.0 == 0));
        self.print_footer(out);
        0
    }
    fn print_each(&self, out: &mut Box<dyn Write>, result: &BuildTool, first: bool);
    fn print_header(&self, _out: &mut Box<dyn Write>, _base: &Path) {
    }
    fn print_footer(&self, _out: &mut Box<dyn Write>) {
    }

    fn print_def(&self, out: &mut Box<dyn Write>, def: &BuildToolDef, first: bool);
    fn print_defs(&self, out: &mut Box<dyn Write>, defs: &BuildToolDefs) {
        self.print_def_header(out);
        defs.iter().enumerate()
                .for_each(|item| self.print_def(out, item.1, item.0 == 0));
        self.print_def_footer(out);
    }
    fn print_def_header(&self, _out: &mut Box<dyn Write>) {
    }
    fn print_def_footer(&self, _out: &mut Box<dyn Write>) {
    }
}

impl dyn Formatter {
    pub fn build(format: Format) -> Box<dyn Formatter> {
        match format {
            Format::Json => Box::new(JsonFormatter{}),
            Format::Default => Box::new(DefaultFormatter{}),
            Format::Xml => Box::new(XmlFormatter{}),
            Format::Yaml => Box::new(YamlFormatter{}),
        }
    }
}

pub struct JsonFormatter{
}

pub struct DefaultFormatter {
}

pub struct XmlFormatter{
}

pub struct YamlFormatter{
}

impl Formatter for DefaultFormatter {
    fn print_header(&self, out: &mut Box<dyn Write>, base: &Path) {
        writeln!(out, "{}", base.display()).unwrap();
    }

    fn print_each(&self, out: &mut Box<dyn Write>, result: &BuildTool, _first: bool) {
        writeln!(out, "  {}: {}", result.path.display(), result.def.name).unwrap();
    }
    fn print_def(&self, out: &mut Box<dyn Write>, def: &BuildToolDef, _first: bool) {
        writeln!(out, "{}: {}", def.name, def.build_files.join(", ")).unwrap();
    }
}

impl Formatter for JsonFormatter{
    fn print_each(&self, out: &mut Box<dyn Write>, result: &BuildTool, first: bool) {
        if !first {
            write!(out, ",").unwrap();
        }
        write!(out, r#"{{"file-path":"{}","tool-name":"{}"}}"#,
                result.path.display(), result.def.name).unwrap();
    }
    fn print_header(&self, out: &mut Box<dyn Write>, base: &Path) {
        write!(out, r#"{{"base":"{}","build-tools":["#, base.display()).unwrap();
    }
    fn print_footer(&self, out: &mut Box<dyn Write>) {
        writeln!(out, "]}}").unwrap();
    }
    fn print_def_header(&self, out: &mut Box<dyn Write>) {
        write!(out, "[").unwrap();
    }
    fn print_def_footer(&self, out: &mut Box<dyn Write>) {
        writeln!(out, "]").unwrap();
    }
    fn print_def(&self, out: &mut Box<dyn Write>, def: &BuildToolDef, first: bool) {
        if !first {
            write!(out, ",").unwrap();
        }
        write!(out, r#"{{"name":"{}","url":"{}","build-files":["#, def.name, def.url).unwrap();
        for (i, element) in def.build_files.iter().enumerate() {
            if i != 0 {
                write!(out, ",").unwrap();
            }
            write!(out, r#""{}""#, element).unwrap();
        }
        write!(out, "]}}").unwrap();
    }
}

impl Formatter for XmlFormatter{
    fn print_header(&self, out: &mut Box<dyn Write>, base: &Path) {
        writeln!(out, "<?xml version=\"1.0\"?>").unwrap();
        write!(out, "<build-tools><base>{}</base>", base.display()).unwrap();
    }
    fn print_footer(&self, out: &mut Box<dyn Write>) {
        writeln!(out, "</build-tools>").unwrap();
    }
    fn print_each(&self, out: &mut Box<dyn Write>, result: &BuildTool, _first: bool) {
        write!(out, "<build-tool><file-path>{}</file-path><tool-name>{}</tool-name></build-tool>",
                result.path.display(), result.def.name).unwrap();
    }

    fn print_def_header(&self, out: &mut Box<dyn Write>) {
        writeln!(out, "<?xml version=\"1.0\"?>").unwrap();
        write!(out, "<build-tool-defs>").unwrap();
    }
    fn print_def_footer(&self, out: &mut Box<dyn Write>) {
        writeln!(out, "</build-tool-defs>").unwrap();
    }
    fn print_def(&self, out: &mut Box<dyn Write>, def: &BuildToolDef, _first: bool) {
        write!(out, "<build-tool-def><name>{}</name><url>{}</url><build-files>", def.name, def.url).unwrap();
        def.build_files.iter()
                .for_each(|item| write!(out, "<file-name>{}</file-name>", item).unwrap());
        write!(out, "</build-files></build-tool-def>").unwrap();
    }
}

impl Formatter for YamlFormatter{
    fn print_header(&self, out: &mut Box<dyn Write>, base: &Path) {
        writeln!(out, "base: {}", base.display()).unwrap();
    }
    fn print_each(&self, out: &mut Box<dyn Write>, result: &BuildTool, _first: bool) {
        writeln!(out, "  - file-path: {}", result.path.display()).unwrap();
        writeln!(out, "    tool-name: {}", result.def.name).unwrap();
    }

    fn print_def_header(&self, out: &mut Box<dyn Write>) {
        writeln!(out, "build-tools-defs").unwrap();
    }

    fn print_def(&self, out: &mut Box<dyn Write>, def: &BuildToolDef, _first: bool) {
        writeln!(out, "  - name: {}", def.name).unwrap();
        writeln!(out, "    url: {}", def.url).unwrap();
        writeln!(out, "    file-names:").unwrap();
        def.build_files.iter().enumerate()
                .for_each(|(index, file_name)| self.print_file_name(out, index, file_name));
    }
}

impl YamlFormatter {
    fn print_file_name(&self, out: &mut Box<dyn Write>, index: usize, file_name: &str) {
        if index == 0 {
            write!(out, "      - ").unwrap();
        } else {
            write!(out, "        ").unwrap();
        };
        writeln!(out, "{}", file_name).unwrap();
    }
}

#[cfg(test)]
mod test_print_defs {
    use super::*;
    use super::super::*;
    use std::str::FromStr;
    use std::fs::{File, remove_file, read_to_string};

    fn write_and_read(format: Format, path: &str) -> String {
        {
            let defs = construct(Some(PathBuf::from_str("testdata/append_def.json").unwrap()), None).unwrap();
            let f = <dyn Formatter>::build(format);
            let mut dest: Box<dyn Write> = Box::new(BufWriter::new(File::create(path).unwrap()));
            f.print_defs(&mut dest, &defs);
        }
        let r = read_to_string(path).unwrap();
        let _ = remove_file(path);
        r.trim().to_string()
    }


    #[test]
    fn test_json() {
        let result = write_and_read(Format::Json, "dest2.json");
        assert_eq!(r#"[{"name":"go","url":"https://go.dev/","build-files":["go.mod"]},{"name":"webpack","url":"https://webpack.js.org/","build-files":["webpack.config.js"]}]"#, result);
    }

    #[test]
    fn test_default() {
        let result = write_and_read(Format::Default, "dest2.txt");
        assert_eq!(r#"go: go.mod
webpack: webpack.config.js"#, result);
    }

    #[test]
    fn test_xml() {
        let result = write_and_read(Format::Xml, "dest2.xml");
        assert_eq!(r#"<?xml version="1.0"?>
<build-tool-defs><build-tool-def><name>go</name><url>https://go.dev/</url><build-files><file-name>go.mod</file-name></build-files></build-tool-def><build-tool-def><name>webpack</name><url>https://webpack.js.org/</url><build-files><file-name>webpack.config.js</file-name></build-files></build-tool-def></build-tool-defs>"#, result);
    }

    #[test]
    fn test_yaml() {
        let result = write_and_read(Format::Yaml, "dest2.yaml");
        assert_eq!(r#"build-tools-defs
  - name: go
    url: https://go.dev/
    file-names:
      - go.mod
  - name: webpack
    url: https://webpack.js.org/
    file-names:
      - webpack.config.js"#, result);
    }
}

#[cfg(test)]
mod test_print_result {
    use super::*;
    use super::super::*;
    use std::path::PathBuf;
    use std::fs::{File, remove_file, read_to_string};
    use std::io::{Write, BufWriter};
    use std::str::FromStr;

    fn setup() -> Vec<BuildTool> {
        let defs = construct(None, None).unwrap();
        let def1 = defs.get(11).unwrap();
        let def2 = defs.get(8).unwrap();
        let bt1 = BuildTool::new(PathBuf::from_str("testdata/fibonacci/build.gradle").unwrap(), def1.clone());
        let bt2 = BuildTool::new(PathBuf::from_str("testdata/hello/Cargo.toml").unwrap(), def2.clone());
        vec![bt1, bt2]
    }

    fn write_and_read(format: Format, path: &str) -> String {
        {
            let vec = setup();
            let f = <dyn Formatter>::build(format);
            let mut dest: Box<dyn Write> = Box::new(BufWriter::new(File::create(path).unwrap()));
            f.print(&mut dest, &PathBuf::from_str("testdata").unwrap(), vec);
        }
        let r = read_to_string(path).unwrap();
        let _ = remove_file(path);
        r.trim().to_string()
    }

    #[test]
    fn test_json() {
        let result = write_and_read(Format::Json, "dest1.json");
        assert_eq!(r#"{"base":"testdata","build-tools":[{"file-path":"testdata/fibonacci/build.gradle","tool-name":"Gradle"},{"file-path":"testdata/hello/Cargo.toml","tool-name":"Cargo"}]}"#, result);
    }

    #[test]
    fn test_xml() {
        let result = write_and_read(Format::Xml, "dest1.xml");
        assert_eq!(r#"<?xml version="1.0"?>
<build-tools><base>testdata</base><build-tool><file-path>testdata/fibonacci/build.gradle</file-path><tool-name>Gradle</tool-name></build-tool><build-tool><file-path>testdata/hello/Cargo.toml</file-path><tool-name>Cargo</tool-name></build-tool></build-tools>"#, result);
    }

    #[test]
    fn test_yaml() {
        let result = write_and_read(Format::Yaml, "dest1.yaml");
        assert_eq!(r#"base: testdata
  - file-path: testdata/fibonacci/build.gradle
    tool-name: Gradle
  - file-path: testdata/hello/Cargo.toml
    tool-name: Cargo"#, result);
    }

    #[test]
    fn test_default() {
        let result = write_and_read(Format::Default, "dest1.txt");
        assert_eq!(r#"testdata
  testdata/fibonacci/build.gradle: Gradle
  testdata/hello/Cargo.toml: Cargo"#, result);
    }
}