package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;

public class HierarchicalMatcher implements BuildToolMatcher {
    private final Path path;

    public HierarchicalMatcher(Path path) {
        this.path = path;
    }

    @Override
    public boolean test(Path base) {
        return base.endsWith(path);
    }
}
