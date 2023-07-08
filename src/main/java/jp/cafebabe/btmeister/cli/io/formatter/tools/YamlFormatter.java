package jp.cafebabe.btmeister.cli.io.formatter.tools;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;

import java.io.PrintWriter;
import java.util.stream.Collectors;

public class YamlFormatter extends Formatter.Base<BuildTool> {
    @Override
    public void formatEntry(PrintWriter out, BuildTool tool, boolean first) {
        var separator = System.getProperty("line.separator");
        out.printf("- tool-name: \"%s\"%n", tool.name());
        out.printf("  url: \"%s\"%n", tool.url());
        out.printf("  build-files:%n");
        out.println(tool.buildFiles().stream()
                .collect(Collectors.joining(separator + "      ", "    - ", "")));
    }
}
