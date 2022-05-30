use std::io::{Write, BufWriter};
use std::path::PathBuf;

use super::btmeister::{BuildToolDef, BuildToolDefs};
use super::{BuildTool, Format};

pub trait Formatter {
    fn print(&self, out: &mut BufWriter<Box<dyn Write>>, base: &PathBuf, vector: Vec<BuildTool>) -> i32 {
        self.print_header(out, base);
        vector.iter().enumerate()
                .for_each(|item| self.print_each(out, item.1, item.0 == 0));
        self.print_footer(out);
        0
    }
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, first: bool);
    fn print_header(&self, _out: &mut BufWriter<Box<dyn Write>>, _base: &PathBuf) {
    }
    fn print_footer(&self, _out: &mut BufWriter<Box<dyn Write>>) {
    }

    fn print_def(&self, out: &mut BufWriter<Box<dyn Write>>, def: &BuildToolDef, first: bool);
    fn print_defs(&self, out: &mut BufWriter<Box<dyn Write>>, defs: &BuildToolDefs) {
        self.print_def_header(out);
        defs.iter().enumerate()
                .for_each(|item| self.print_def(out, item.1, item.0 == 0));
        self.print_def_footer(out);
    }
    fn print_def_header(&self, _out: &mut BufWriter<Box<dyn Write>>) {
    }
    fn print_def_footer(&self, _out: &mut BufWriter<Box<dyn Write>>) {
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
    fn print_header(&self, out: &mut BufWriter<Box<dyn Write>>, base: &PathBuf) {
        writeln!(out, "{}", base.display()).unwrap();
    }

    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, _first: bool) {
        writeln!(out, "  {}: {}", result.path.display(), result.def.name).unwrap();
    }
    fn print_def(&self, out: &mut BufWriter<Box<dyn Write>>, def: &BuildToolDef, _first: bool) {
        writeln!(out, "{}: {}", def.name, def.build_files.join(", ")).unwrap();
    }
}

impl Formatter for JsonFormatter{
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, first: bool) {
        if !first {
            write!(out, ",").unwrap();
        }
        write!(out, "{{\"file-path\":\"{}\",\"tool-name\":\"{}\"}}",
                result.path.display(), result.def.name).unwrap();
    }
    fn print_header(&self, out: &mut BufWriter<Box<dyn Write>>, base: &PathBuf) {
        write!(out, "{{\"base\":\"{}\",\"build-tools\":[", base.display()).unwrap();
    }
    fn print_footer(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "]}}").unwrap();
    }
    fn print_def_header(&self, out: &mut BufWriter<Box<dyn Write>>) {
        write!(out, "[").unwrap();
    }
    fn print_def_footer(&self, out: &mut BufWriter<Box<dyn Write>>) {
        write!(out, "]").unwrap();
    }
    fn print_def(&self, out: &mut BufWriter<Box<dyn Write>>, def: &BuildToolDef, first: bool) {
        if !first {
            write!(out, ",").unwrap();
        }
        write!(out, "{{\"name\":\"{}\",\"url\":\"{}\",\"build-files\":[", def.name, def.url).unwrap();
        for (i, element) in def.build_files.iter().enumerate() {
            if i != 0 {
                write!(out, ",").unwrap();
            }
            write!(out, "\"{}\"", element).unwrap();
        }
        write!(out, "]}}").unwrap();
    }
}

impl Formatter for XmlFormatter{
    fn print_header(&self, out: &mut BufWriter<Box<dyn Write>>, base: &PathBuf) {
        writeln!(out, "<?xml version=\"1.0\"?>").unwrap();
        write!(out, "<build-tools><base>{}</base>", base.display()).unwrap();
    }
    fn print_footer(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "</build-tools>").unwrap();
    }
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, _first: bool) {
        write!(out, "<build-tool><file-path>{}</file-path><tool-name>{}</tool-name></build-tool>",
                result.path.display(), result.def.name).unwrap();
    }

    fn print_def_header(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "<?xml version=\"1.0\"?>").unwrap();
        write!(out, "<build-tool-defs>").unwrap();
    }
    fn print_def_footer(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "</build-tool-defs>").unwrap();
    }
    fn print_def(&self, out: &mut BufWriter<Box<dyn Write>>, def: &BuildToolDef, _first: bool) {
        write!(out, "<build-tool-def><name>{}</name><url>{}</url><build-files>", def.name, def.url).unwrap();
        def.build_files.iter()
                .for_each(|item| write!(out, "<file-name>{}</file-name>", item).unwrap());
        write!(out, "</build-files></build-tool-def>").unwrap();
    }
}

impl Formatter for YamlFormatter{
    fn print_header(&self, out: &mut BufWriter<Box<dyn Write>>, base: &PathBuf) {
        writeln!(out, "base: {}", base.display()).unwrap();
    }
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, _first: bool) {
        writeln!(out, "  - file-path: {}", result.path.display()).unwrap();
        writeln!(out, "    tool-name: {}", result.def.name).unwrap();
    }

    fn print_def_header(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "build-tools-defs").unwrap();
    }

    fn print_def(&self, out: &mut BufWriter<Box<dyn Write>>, def: &BuildToolDef, _first: bool) {
        writeln!(out, "  - name: {}", def.name).unwrap();
        writeln!(out, "    url: {}", def.url).unwrap();
        writeln!(out, "    file-names:").unwrap();
        def.build_files.iter().enumerate()
                .for_each(|(index, file_name)| self.print_file_name(out, index, file_name));
    }
}

impl YamlFormatter {
    fn print_file_name(&self, out: &mut BufWriter<Box<dyn Write>>, index: usize, file_name: &String) {
        if index == 0 {
            write!(out, "      - ").unwrap();
        } else {
            write!(out, "        ").unwrap();
        };
        writeln!(out, "{}", file_name).unwrap();
    }
}
