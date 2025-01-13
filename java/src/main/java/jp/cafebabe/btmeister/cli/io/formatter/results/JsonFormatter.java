package jp.cafebabe.btmeister.cli.io.formatter.results;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;
import jp.cafebabe.btmeister.utils.Pair;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class JsonFormatter extends Formatter.Base<Pair<BuildTool, List<Path>>> {
    private Path projectPath;

    @Override
    public void header(PrintWriter out) {
        out.print("[");
    }

    @Override
    public void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe) {
        this.projectPath = projectPath;
        out.printf("{\"project-path\":\"%s\"%s,\"build-tools\":[", projectPath,
                oe.map(e -> String.format(",\"error-message\":\"%s\"", e.getMessage()))
                        .orElse(""));
    }

    @Override
    public void afterEntry(PrintWriter out) {
        out.print("]}");
        this.projectPath = null;
    }

    @Override
    public void formatEntry(PrintWriter out, Pair<BuildTool, List<Path>> entry, boolean first) {
        if(!first)
            out.print(",");
        out.printf("{\"tool-name\":\"%s\",\"url\":\"%s\",\"files\":[%s]}",
                entry.left().name(), entry.left().url(), entry.right().stream()
                        .map(p -> stripBasePath(projectPath, p))
                        .collect(Collectors.joining("\",\"", "\"", "\"")));
    }

    @Override
    public void footer(PrintWriter out) {
        out.println("]");
        out.flush();
    }
}
