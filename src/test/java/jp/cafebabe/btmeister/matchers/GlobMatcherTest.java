package jp.cafebabe.btmeister.matchers;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class GlobMatcherTest {
    @Test
    public void testBasic() {
        var matcher = new GlobMatcher(Path.of(".github/workflows/*.y{a,}ml"));
        assertTrue(matcher.test(Path.of(".github/workflows/build.yaml")));
        assertTrue(matcher.test(Path.of(".github/workflows/release.yml")));
        assertFalse(matcher.test(Path.of(".github/ISSUE_TEMPLATE/bug_report.md")));
        assertFalse(matcher.test(Path.of(".github/workflows/config.toml")));
        assertFalse(matcher.test(Path.of(".circleci/config.yml")));
    }

    @Test
    public void testSimple() {
        var matcher = new GlobMatcher(Path.of("**/Makefile"));
        assertTrue(matcher.test(Path.of("under/some/dir/Makefile")));
        assertTrue(matcher.test(Path.of("docs/Makefile")));
        assertFalse(matcher.test(Path.of("Makefile")));
    }
}
