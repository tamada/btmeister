package jp.cafebabe.btmeister.cli.io.formatter;

import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.cli.Btmeister;
import jp.cafebabe.btmeister.cli.io.Format;
import jp.cafebabe.btmeister.cli.io.Formatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.DefaultFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.JsonFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.XmlFormatter;
import jp.cafebabe.btmeister.cli.io.formatter.results.YamlFormatter;
import jp.cafebabe.btmeister.utils.Pair;

import java.nio.file.Path;
import java.util.List;
import java.util.logging.Logger;

public class FormatterFactory {
    public static Formatter<Pair<BuildTool, List<Path>>> resultFormatter(Format format) {
        var logger = Logger.getLogger(Btmeister.NAME);
        logger.info(() -> String.format("build ResultFormatter (format: {})", format));
        return switch (format) {
            case Json -> new JsonFormatter();
            case Xml -> new XmlFormatter();
            case Yaml -> new YamlFormatter();
            case Default -> new DefaultFormatter();
            default -> throw new InternalError("Illegal format: " + format);
        };
    }

    public static Formatter<BuildTool> listFormatter(Format format) {
        var logger = Logger.getLogger(Btmeister.NAME);
        logger.info(() -> String.format("build ListFormatter (format: {})", format));
        return switch (format) {
            case Json -> new jp.cafebabe.btmeister.cli.io.formatter.tools.JsonFormatter();
            case Xml -> new jp.cafebabe.btmeister.cli.io.formatter.tools.XmlFormatter();
            case Yaml -> new jp.cafebabe.btmeister.cli.io.formatter.tools.YamlFormatter();
            case Default -> new jp.cafebabe.btmeister.cli.io.formatter.tools.DefaultFormatter();
            default -> throw new InternalError("illegal format: " + format);
        };
    }
}
