package jp.cafebabe.btmeister;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import jp.cafebabe.btmeister.matchers.BuildToolMatcher;
import jp.cafebabe.btmeister.matchers.MatcherBuilder;

import java.net.URL;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.function.Predicate;

public class BuildTool {
    private final String name;
    private final List<String> buildFiles;
    private final URL url;

    @JsonIgnore
    private List<BuildToolMatcher> matchers;

    @JsonCreator
    public BuildTool(@JsonProperty("name") String name,
                     @JsonProperty("url") URL url,
                     @JsonProperty("build-files") String... buildFileNames) {
        this.name = name;
        this.url = url;
        this.buildFiles = Arrays.asList(buildFileNames);
        this.matchers = buildFiles.stream()
                .map(MatcherBuilder::build)
                .toList();
    }

    public URL url() {
        return url;
    }

    public List<String> buildFiles() {
        return Collections.unmodifiableList(buildFiles);
    }

    public String name() {
        return name;
    }

    public boolean match(Path path) {
        return false;
    }

    public boolean matchBuildFileName(Path path) {
        String fileName = path.getFileName().toString();
        return matchers.stream()
                .anyMatch(matcher -> matcher.test(path));
    }
}
