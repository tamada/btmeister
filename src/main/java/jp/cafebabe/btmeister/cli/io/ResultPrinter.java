package jp.cafebabe.btmeister.cli.io;

import io.vavr.control.Either;
import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.utils.Pair;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.function.BooleanSupplier;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class ResultPrinter {
    private final PrintWriter out;
    private final Formatter<Pair<BuildTool, List<Path>>> formatter;

    public ResultPrinter(PrintWriter out, Formatter<Pair<BuildTool, List<Path>>> formatter) {
        this.out = out;
        this.formatter = formatter;
    }

    public int print(List<Pair<Path, Either<Exception, List<Pair<Path, BuildTool>>>>> result) {
        formatter.header(out);
        result.forEach(project -> printProject(project.left(), project.right()));
        formatter.footer(out);
        out.flush();
        return 0;
    }

    private void printProject(Path projectPath, Either<Exception, List<Pair<Path, BuildTool>>> either) {
        formatter.beforeEntry(out, projectPath, findException(either));
        var map = toMap(either.getOrElse(List::of));
        var bs = new FirstBooleanSupplier();
        map.entrySet().stream()
                .sorted(Comparator.comparing(e -> e.getKey().name()))
                .forEach(e -> formatter.formatEntry(out, Pair.of(e), bs.getAsBoolean()));
        formatter.afterEntry(out);
    }

    private Map<BuildTool, List<Path>> toMap(List<Pair<Path, BuildTool>> list) {
        return list.stream()
                .collect(Collectors.toMap(Pair::right, p -> List.of(p.left()),
                        (l1, l2) -> Stream.concat(l1.stream(), l2.stream()).collect(Collectors.toList())));
    }

    private Optional<Exception> findException(Either<Exception, List<Pair<Path, BuildTool>>> either) {
        if(either.isRight())
            return Optional.empty();
        return Optional.of(either.getLeft());
    }

    private static class FirstBooleanSupplier implements BooleanSupplier {
        private boolean first = true;

        @Override
        public boolean getAsBoolean() {
            var result = first;
            if(first)
                first = false;
            return result;
        }
    }
}
