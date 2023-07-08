package jp.cafebabe.btmeister.cli.io;

import io.vavr.collection.Stream;
import jp.cafebabe.btmeister.BuildTool;

import java.io.PrintWriter;
import java.util.Optional;

public class BuildToolsPrinter {
    private final PrintWriter out;
    private final Formatter<BuildTool> formatter;

    public BuildToolsPrinter(PrintWriter printWriter, Formatter<BuildTool> formatter) {
        this.out = printWriter;
        this.formatter = formatter;
    }

    public int print(java.util.stream.Stream<BuildTool> stream) {
        formatter.header(out);
        var indexStream = Stream.from(0);
        var stream2 = indexStream.zip(stream.toList());
        stream2.forEach(tuple -> printEach(tuple._1(), tuple._2()));
        formatter.footer(out);
        out.flush();
        return 0;
    }

    private void printEach(int index, BuildTool tool) {
        formatter.beforeEntry(out, null, Optional.empty());
        formatter.formatEntry(out, tool, index == 0);
        formatter.afterEntry(out);
    }
}
