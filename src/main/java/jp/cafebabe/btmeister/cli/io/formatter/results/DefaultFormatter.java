package jp.cafebabe.btmeister.cli.io.formatter.results;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;
import jp.cafebabe.btmeister.utils.Pair;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class DefaultFormatter extends Formatter.Base<Pair<BuildTool, List<Path>>> {
    private Path projectPath;

    @Override
    public void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe) {
        this.projectPath = projectPath;
        out.printf("%s%n", projectPath);
        oe.ifPresent(e -> out.printf("\terror: %s%n", e.getMessage()));
    }

    @Override
    public void formatEntry(PrintWriter out, Pair<BuildTool, List<Path>> pair, boolean first) {
        String separator = System.getProperty("line.separator");
        out.printf("  %s (%s)%n", pair.left().name(), pair.left().url());
        out.printf("    %s%n", pair.right().stream()
                .map(p -> stripBasePath(projectPath, p))
                .map(item -> String.format("    %s", item))
                .collect(Collectors.joining(separator)).trim());
    }
}
