package jp.cafebabe.btmeister.cli;

import jp.cafebabe.btmeister.cli.logging.LogFormatter;
import picocli.CommandLine.Model.CommandSpec;
import picocli.CommandLine.Option;
import picocli.CommandLine.ParameterException;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.logging.ConsoleHandler;
import java.util.logging.Level;
import java.util.logging.LogManager;
import java.util.logging.Logger;

class LoggerConfig {
    @Option(names = {"--loglevel"}, paramLabel = "LEVEL", description = "specify the log level. default is warn. available off, error, warn, info, debug, and all")
    private LogLevel level = LogLevel.WARNING;

    @Option(names = {"--logging-config"}, paramLabel = "FILE", description = "specify the configuration file for logging (java.util.logging)")
    private Path logConfig;

    public void validate(CommandSpec spec) {
        initConfig(spec);
        updateLevelForRootLogger();
    }

    private void updateLevelForRootLogger() {
        var root = Logger.getLogger("");
        root.setLevel(level.level());
    }

    private void initConfig(CommandSpec spec) {
        if(logConfig == null) {
            loadDefaultConfig();
        } else {
            loadConfig(logConfig, spec);
        }
    }

    private void loadConfig(Path path, CommandSpec spec) {
        try(InputStream in = Files.newInputStream(path)) {
            LogManager.getLogManager()
                    .readConfiguration(in);
        } catch(IOException e) {
            throw new ParameterException(spec.commandLine(), String.format("%s load error (%s)", path, e.getMessage()));
        }
    }

    private void loadDefaultConfig() {
        Logger root = Logger.getLogger("");
        root.setLevel(Level.WARNING);
        var handler = new ConsoleHandler();
        handler.setFormatter(new LogFormatter());
        root.setUseParentHandlers(false);
        Arrays.stream(root.getHandlers())
                .filter(h -> h instanceof ConsoleHandler)
                .peek(h -> System.out.println("handler: " + h))
                .forEach(h -> root.removeHandler(h));
        root.addHandler(handler);
    }

    private void updateConfiguration(InputStream in) throws IOException {
    }

    public static enum LogLevel {
        OFF(Level.OFF),
        WARNING(Level.WARNING),
        INFO(Level.INFO),
        DEBUG(Level.FINE),
        ALL(Level.ALL);
        private Level level;

        private LogLevel(Level level) {
            this.level = level;
        }
        public Level level() {
            return level;
        }
    }
}
