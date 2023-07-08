package jp.cafebabe.btmeister;

import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class BuildToolManager {
    private final List<BuildTool> tools;

    BuildToolManager(List<BuildTool> tools) {
        this.tools = tools;
    }

    public Stream<BuildTool> stream() {
        return tools.stream();
    }

    public Stream<BuildTool> find(Path path) {
        return stream()
                .filter(bt -> bt.matchBuildFileName(path));
    }

    public BuildToolManager merge(BuildToolManager other) {
        return new BuildToolManager(Stream.concat(tools.stream(), other.stream())
                .collect(Collectors.toList()));
    }

    public static BuildToolManager loadManager(URL url) throws IOException {
        return BuildToolJsonParser.parse(url.openStream());
    }


    public static BuildToolManager getDefault() throws IOException {
        try(InputStream in = BuildToolManager.class.getResourceAsStream("/resources/build_tools.json")) {
            return BuildToolJsonParser.parse(in);
        }
    }
}