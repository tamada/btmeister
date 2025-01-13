package jp.cafebabe.btmeister.cli.io.formatter;

import jp.cafebabe.btmeister.cli.io.Format;
import jp.cafebabe.btmeister.cli.io.formatter.tools.DefaultFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.tools.JsonFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.tools.XmlFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.tools.YamlFormatter;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

public class ListFormatterFactoryTest {
    @Test
    public void testConstructedObject() throws Exception {
        assertEquals(FormatterFactory.listFormatter(Format.Default).getClass(), DefaultFormatter.class);
        assertEquals(FormatterFactory.listFormatter(Format.Json).getClass(), JsonFormatter.class);
        assertEquals(FormatterFactory.listFormatter(Format.Xml).getClass(), XmlFormatter.class);
        assertEquals(FormatterFactory.listFormatter(Format.Yaml).getClass(), YamlFormatter.class);
    }

    @Test
    public void testNull() {
        assertThrows(NullPointerException.class, () -> FormatterFactory.listFormatter(null));
    }
}
