package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;
import java.nio.file.PathMatcher;

public class GlobMatcher implements BuildToolMatcher {
    private final PathMatcher matcher;

    public GlobMatcher(Path name) {
        this.pattern = name;
        this.matcher = name.getFileSystem().getPathMatcher("glob:" + name.toString());
    }

    @Override
    public boolean test(Path path) {
        return matcher.matches(path);
    }
}
