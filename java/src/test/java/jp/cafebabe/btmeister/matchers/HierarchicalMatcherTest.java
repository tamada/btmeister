package jp.cafebabe.btmeister.matchers;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class HierarchicalMatcherTest {
    @Test
    public void testBasic() {
        var matcher = new HierarchicalMatcher(Path.of(".circleci/config.yml"));
        assertTrue(matcher.test(Path.of(".circleci/config.yml")));
        assertTrue(matcher.test(Path.of("src/test/resources/circleci/.circleci/config.yml")));
        assertFalse(matcher.test(Path.of("config.yml")));
    }
}
