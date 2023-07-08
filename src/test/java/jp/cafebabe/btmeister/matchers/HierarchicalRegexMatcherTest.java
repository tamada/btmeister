package jp.cafebabe.btmeister.matchers;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class HierarchicalRegexMatcherTest {
    @Test
    public void testBasic() {
        var matcher = new HierarchicalGlobMatcher(Path.of(".github/workflows/*.y{a,}ml"));
        assertTrue(matcher.test(Path.of(".github/workflows/build.yaml")));
        assertTrue(matcher.test(Path.of(".github/workflows/release.yml")));
        assertFalse(matcher.test(Path.of(".github/ISSUE_TEMPLATE/bug_report.md")));
        assertFalse(matcher.test(Path.of(".github/workflows/config.toml")));
        assertFalse(matcher.test(Path.of(".circleci/config.yml")));
    }
}
