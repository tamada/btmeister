package jp.cafebabe.btmeister.matchers;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertTrue;

public class SimpleMatcherTest {
    @Test
    public void testBasic() {
        var matcher = new SimpleMatcher(Path.of("Makefile"));
        assertTrue(matcher.test(Path.of("Makefile")));
        assertTrue(matcher.test(Path.of("docs/Makefile")));
    }
}
