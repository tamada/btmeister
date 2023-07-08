package jp.cafebabe.btmeister.cli.io.formatter;

import jp.cafebabe.btmeister.cli.io.Format;
import jp.cafebabe.btmeister.cli.io.formatter.results.DefaultFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.JsonFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.XmlFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.YamlFormatter;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

public class ResultsFormatterFactoryTest {
    @Test
    public void testConstructedObject() throws Exception {
        assertEquals(FormatterFactory.resultFormatter(Format.Default).getClass(), DefaultFormatter.class);
        assertEquals(FormatterFactory.resultFormatter(Format.Json).getClass(), JsonFormatter.class);
        assertEquals(FormatterFactory.resultFormatter(Format.Xml).getClass(), XmlFormatter.class);
        assertEquals(FormatterFactory.resultFormatter(Format.Yaml).getClass(), YamlFormatter.class);
    }

    @Test
    public void testNull() {
        assertThrows(NullPointerException.class, () -> FormatterFactory.resultFormatter(null));
    }
}
