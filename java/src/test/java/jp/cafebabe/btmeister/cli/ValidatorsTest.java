package jp.cafebabe.btmeister.cli;

import org.junit.jupiter.api.Test;
import picocli.CommandLine;
import picocli.CommandLine.ParameterException;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class ValidatorsTest {
    @Test
    public void testValidateRunMode() {
        var v = new Validators(new CommandLine(new Arguments()).getCommandSpec());
        v.validateRunMode(List.of("."), false);
        assertTrue(true);
        v.validateRunMode(List.of(), true);
        assertTrue(true);
        assertThrows(ParameterException.class, () -> v.validateRunMode(List.of(), false));
        assertThrows(ParameterException.class, () -> v.validateRunMode(List.of("a"), true));
    }

    @Test
    public void testValidateAppendDefs() {
        var v = new Validators(new CommandLine(new Arguments()).getCommandSpec());
        v.validateAppendDefs(false, null);
        assertTrue(true);
        v.validateAppendDefs(true, Path.of("src/test/resources/append_tools.json"));
        assertTrue(true);
        assertThrows(ParameterException.class, () -> v.validateAppendDefs(true, null));
    }

    @Test
    public void validateDefFile() {
        var v = new Validators(new CommandLine(new Arguments()).getCommandSpec());
        v.validateDefFile(null);
        assertTrue(true);
        v.validateDefFile(Path.of("src/main/resources/resources/build_tools.json"));
        assertTrue(true);
        assertThrows(ParameterException.class, () -> v.validateDefFile(Path.of("not/exist/file")));
        assertThrows(ParameterException.class, () -> v.validateDefFile(Path.of("src")));
    }

    @Test
    public void validateProjectArguments() {
        var v = new Validators(new CommandLine(new Arguments()).getCommandSpec());
        v.validateArguments(List.of("src/test/resources/fibonacci", "src/test/resources/hello"));
        assertTrue(true);
        v.validateArguments(List.of("-"));
        assertTrue(true);
        v.validateArguments(List.of("@src/test/resources/project_list.txt"));
        assertTrue(true);
    }

    @Test
    public void validateUnreadableFile() throws IOException {
        Path unreadableFile = Path.of("src/test/resources/unreadable_file");
        try {
            Files.setPosixFilePermissions(unreadableFile, PosixFilePermissions.fromString("---------"));
            var v = new Validators(new CommandLine(new Arguments()).getCommandSpec());
            assertThrows(ParameterException.class, () -> v.validateDefFile(unreadableFile));
        } catch(IOException e) {
            // ignore the exception in throwing the permission update.
        } finally {
            Files.setPosixFilePermissions(unreadableFile, PosixFilePermissions.fromString("rw-r--r--"));
        }
    }
}
