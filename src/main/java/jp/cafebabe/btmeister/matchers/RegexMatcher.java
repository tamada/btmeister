package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;

public class RegexMatcher implements BuildToolMatcher {
    private final String pattern;

    public RegexMatcher(Path name) {
        this.pattern = name.getFileName().toString();
    }

    @Override
    public boolean test(Path path) {
        var name = path.getFileName().toString();
        return name.matches(pattern);
    }
}
