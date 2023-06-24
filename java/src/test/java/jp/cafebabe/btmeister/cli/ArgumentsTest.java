package jp.cafebabe.btmeister.cli;

import org.junit.jupiter.api.Test;
import picocli.CommandLine;
import picocli.CommandLine.ParameterException;

import static org.junit.jupiter.api.Assertions.*;

public class ArgumentsTest {
    @Test
    public void testDefFileEntryCount() {
        Main main = new Main();
        new CommandLine(main)
                .execute(".".split(" "));
        var either = main.args.manager();
        assertTrue(either.isRight());
        assertEquals(37, either.get().stream().count());
    }

    @Test
    public void testDefFileEntryCountWithAppendedDefFile() {
        Main main = new Main();
        new CommandLine(main)
                .execute("--append-defs -d src/test/resources/append_tools.json .".split(" "));
        var either = main.args.manager();
        assertTrue(either.isRight());
        assertEquals(38, either.get().stream().count());
    }

    @Test
    public void testDefFileEntryCountWithReplacedDefFile() {
        Main main = new Main();
        new CommandLine(main)
                .execute("-d src/test/resources/append_tools.json .".split(" "));
        var either = main.args.manager();
        assertTrue(either.isRight());
        assertEquals(1, either.get().stream().count());
    }

    @Test
    public void testDefault() {
        Main main = new Main();
        new CommandLine(main)
                .execute("src/test/resources/fibonacci".split(" "));
        main.validate();
        assertTrue(true);
    }

    @Test
    public void testProjectNotFound() {
        Main main = new Main();
        new CommandLine(main)
                .execute("-l src/test/resources/fibonacci".split(" "));
        assertThrows(ParameterException.class, main::validate);
    }
}
