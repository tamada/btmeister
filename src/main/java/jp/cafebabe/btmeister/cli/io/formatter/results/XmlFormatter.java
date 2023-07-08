package jp.cafebabe.btmeister.cli.io.formatter.results;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;
import jp.cafebabe.btmeister.utils.Pair;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class XmlFormatter extends Formatter.Base<Pair<BuildTool, List<Path>>> {
    private Path projectPath;

    @Override
    public void header(PrintWriter out) {
        out.printf("<?xml version=\"1.0\"?>%n<projects>");
    }

    @Override
    public void footer(PrintWriter out) {
        out.print("</projects>");
    }


    @Override
    public void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe) {
        this.projectPath = projectPath;
        out.printf("<project><path>%s</path>%s", projectPath,
                oe.map(e -> String.format(",\"error-message\":\"%s\"", e.getMessage()))
                        .orElse(""));
    }

    @Override
    public void afterEntry(PrintWriter out) {
        out.print("</build-tools></project>");
        this.projectPath = null;
    }

    @Override
    public void formatEntry(PrintWriter out, Pair<BuildTool, List<Path>> pair, boolean first) {
        if(first)
            out.print("<build-tools>");
        out.printf("<build-tool><tool-name>%s</tool-name><url>%s</url><path-list>%s</path-list></build-tool>",
                pair.left().name(), pair.left().url(), pair.right().stream()
                        .map(p -> stripBasePath(projectPath, p))
                        .collect(Collectors.joining("</path><path>", "<path>", "</path>")));
    }
}
