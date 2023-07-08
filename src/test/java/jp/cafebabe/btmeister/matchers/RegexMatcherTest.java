package jp.cafebabe.btmeister.matchers;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class RegexMatcherTest {
    @Test
    public void testBasic() {
        var matcher = new RegexMatcher(Path.of(".*\\.ya?ml"));
        assertTrue(matcher.test(Path.of("hoge.yaml")));
        assertTrue(matcher.test(Path.of("config.yml")));
        assertFalse(matcher.test(Path.of("config.toml")));
    }
}
