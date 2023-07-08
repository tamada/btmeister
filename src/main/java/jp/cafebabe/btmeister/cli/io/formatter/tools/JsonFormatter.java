package jp.cafebabe.btmeister.cli.io.formatter.tools;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.io.Formatter;

import java.io.PrintWriter;
import java.util.stream.Collectors;

public class JsonFormatter extends Formatter.Base<BuildTool> {
    @Override
    public void header(PrintWriter out) {
        out.print("[");
    }

    @Override
    public void footer(PrintWriter out) {
        out.print("]");
    }

    @Override
    public void formatEntry(PrintWriter out, BuildTool tool, boolean first) {
        if(!first)
            out.print(",");
        out.printf("{\"name\":\"%s\",\"build-files\":[%s],\"url\":\"%s\"}",
                tool.name(), tool.buildFiles()
                        .stream().map(s -> "\""  + s + "\"")
                        .collect(Collectors.joining(",")), tool.url());
    }
}
