package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;

public class MatcherBuilder {
    private MatcherBuilder() {
    }

    public static BuildToolMatcher build(String pattern) {
        return new MatcherBuilder().buildImpl(pattern);
    }

    private BuildToolMatcher buildImpl(String pattern) {
        Path path = Path.of(pattern);
        return switch(findPattern(pattern)) {
            case GLOB -> new GlobMatcher(path);
            case REGEXP -> new RegexMatcher(path);
            case HIERARCHICAL -> new HierarchicalMatcher(path);
            case HIERARCHICAL_REGEX_PATH -> new HierarchicalGlobMatcher(path);
            case SIMPLE -> new SimpleMatcher(path);
        };
    }

    private static enum MatcherPattern {
        GLOB,
        REGEXP,
        HIERARCHICAL,
        HIERARCHICAL_REGEX_PATH,
        SIMPLE
    }

    private MatcherPattern findPattern(String pattern) {
        if(isHierarchical(pattern)) {
            if(isGlob(pattern))
                return MatcherPattern.HIERARCHICAL_REGEX_PATH;
            return MatcherPattern.HIERARCHICAL;
        }
        else if(isGlob(pattern))
            return MatcherPattern.GLOB;
        return MatcherPattern.SIMPLE;
    }

    private boolean isHierarchical(String pattern) {
        return pattern.chars().anyMatch(c -> c == '/');
    }

    private boolean isGlob(String pattern) {
        return pattern.chars()
                .anyMatch(c -> "*?{".chars()
                        .anyMatch(cc -> cc == c));
    }
}
