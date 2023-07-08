package jp.cafebabe.btmeister.cli.io.formatter.tools;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.stream.Collectors;

public class XmlFormatter extends Formatter.Base<BuildTool> {
    private Path projectPath;

    @Override
    public void header(PrintWriter out) {
        out.printf("<?xml version=\"1.0\"?>%n<build-tools>");
    }

    @Override
    public void footer(PrintWriter out) {
        out.print("</build-tools>");
    }

    @Override
    public void formatEntry(PrintWriter out, BuildTool tool, boolean first) {
        out.printf("<build-tool><name>%s</name><url>%s</url><build-files>%s</build-files></build-tool>",
                tool.name(), tool.url(), tool.buildFiles().stream()
                        .collect(Collectors.joining("</matcher><matcher>", "<matcher>", "</matcher>")));
    }
}
