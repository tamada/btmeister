package jp.cafebabe.btmeister.cli.io.formatter.tools;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.Optional;
import java.util.stream.Collectors;

public class DefaultFormatter extends Formatter.Base<BuildTool> {
    public void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe) {
        oe.ifPresent(e -> out.printf("\terror: %s%n", e.getMessage()));
    }

    @Override
    public void formatEntry(PrintWriter out, BuildTool tool, boolean first) {
        String separator = System.getProperty("line.separator");
        out.printf("%s (%s)%n", tool.name(), tool.url());
        out.printf("    %s%n", tool.buildFiles().stream().collect(Collectors.joining(", ")));
    }
}
