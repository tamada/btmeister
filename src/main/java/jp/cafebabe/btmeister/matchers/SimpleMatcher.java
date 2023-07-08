package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;

public class SimpleMatcher implements BuildToolMatcher {
    private final Path name;

    public SimpleMatcher(Path name) {
        this.name = name.getFileName();
    }

    @Override
    public boolean test(Path path) {
        return path.getFileName().equals(name);
    }
}
