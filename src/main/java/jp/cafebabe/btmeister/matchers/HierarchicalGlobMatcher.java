package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;
import java.nio.file.PathMatcher;

public class HierarchicalGlobMatcher implements BuildToolMatcher {
    private final Path path;
    private final PathMatcher pattern;

    public HierarchicalGlobMatcher(Path path) {
        this.path = path.getParent();
        this.pattern = path.getFileSystem()
                .getPathMatcher("glob:" + path.getFileName());
    }

    @Override
    public boolean test(Path base) {
        var name = base.getFileName();
        return base.getParent().endsWith(path)
                && pattern.matches(name);
    }
}
