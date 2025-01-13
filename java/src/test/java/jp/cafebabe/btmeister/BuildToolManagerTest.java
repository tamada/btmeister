package jp.cafebabe.btmeister;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class BuildToolManagerTest {
    @Test
    public void testReadJson() {
        var manager = assertDoesNotThrow(BuildToolManager::getDefault);
        assertTrue(manager.stream().findAny().isPresent());
    }
}
