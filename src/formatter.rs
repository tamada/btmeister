use std::io::{Write, BufWriter};
use super::{BuildTool, Format};

pub trait Formatter {
    fn print(&self, out: &mut BufWriter<Box<dyn Write>>, vector: Vec<BuildTool>) -> i32 {
        self.print_header(out);
        vector.iter().enumerate()
                .for_each(|item| self.print_each(out, item.1, item.0 == 0));
        self.print_footer(out);
        0
    }
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, first: bool);
    fn print_header(&self, _out: &mut BufWriter<Box<dyn Write>>) {
    }
    fn print_footer(&self, _out: &mut BufWriter<Box<dyn Write>>) {
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
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, _first: bool) {
        writeln!(out, "{}: {}", result.path.display(), result.def.name).unwrap();
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
    fn print_header(&self, out: &mut BufWriter<Box<dyn Write>>) {
        write!(out, "[").unwrap();
    }
    fn print_footer(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "]").unwrap();
    }
}

impl Formatter for XmlFormatter{
    fn print_header(&self, out: &mut BufWriter<Box<dyn Write>>) {
        write!(out, "<?xml version=\"1.0\"?>").unwrap();
        write!(out, "<build-tools>").unwrap();
    }
    fn print_footer(&self, out: &mut BufWriter<Box<dyn Write>>) {
        writeln!(out, "</build-tools>").unwrap();
    }
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, _first: bool) {
        write!(out, "<build-tool><file-path>{}</file-path><tool-name>{}</tool-name></build-tool>",
                result.path.display(), result.def.name).unwrap();
    }
}

impl Formatter for YamlFormatter{
    fn print_each(&self, out: &mut BufWriter<Box<dyn Write>>, result: &BuildTool, _first: bool) {
        writeln!(out, "file-path: {}", result.path.display()).unwrap();
        writeln!(out, "    tool-name: {}", result.def.name).unwrap();
    }
}
