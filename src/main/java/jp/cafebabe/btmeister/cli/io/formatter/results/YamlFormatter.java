package jp.cafebabe.btmeister.cli.io.formatter.results;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;
import jp.cafebabe.btmeister.utils.Pair;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.List;
import java.util.Optional;

public class YamlFormatter extends Formatter.Base<Pair<BuildTool, List<Path>>> {
    private Path projectPath;

    @Override
    public void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe) {
        this.projectPath = projectPath;
        out.printf("- project-path: \"%s\"%n%s  build-tools:%n", projectPath,
                oe.map(e -> String.format("  error-message: \"%s\"%n", e.getMessage())).orElse(""));
    }

    @Override
    public void formatEntry(PrintWriter out, Pair<BuildTool, List<Path>> pair, boolean first) {
        out.printf("  - tool-name: \"%s\"%n", pair.left().name());
        out.printf("    url: \"%s\"%n", pair.left().url());
        out.printf("    files:%n");
        pair.right().stream()
                .forEach(p -> out.printf("    - \"%s\"%n", stripBasePath(projectPath, p)));
    }
}
