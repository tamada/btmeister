package jp.cafebabe.btmeister.cli;

import io.vavr.control.Either;
import jp.cafebabe.btmeister.BuildTool;
import jp.cafebabe.btmeister.BuildToolIdentifier;
import jp.cafebabe.btmeister.BuildToolManager;
import jp.cafebabe.btmeister.cli.io.BuildToolsPrinter;
import jp.cafebabe.btmeister.cli.io.ResultPrinter;
import jp.cafebabe.btmeister.cli.io.formatter.FormatterFactory;
import jp.cafebabe.btmeister.utils.Pair;
import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.Mixin;
import picocli.CommandLine.Model.CommandSpec;
import picocli.CommandLine.ParameterException;
import picocli.CommandLine.Spec;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.List;
import java.util.concurrent.Callable;
import java.util.logging.Logger;
import java.util.stream.Collectors;

@Command(name="btmeister", mixinStandardHelpOptions = true, versionProvider = Btmeister.class)
public class Main implements Callable<Integer> {
    @Mixin
    Arguments args;
    private BuildToolManager manager;

    @Spec
    private CommandSpec spec;

    public BuildToolIdentifier identifier() {
        manager = args.manager()
                .getOrElseThrow(e -> new InternalError(e));
        Logger.getLogger(Btmeister.NAME)
                .info("build BuildToolManager done");
        return new BuildToolIdentifier(manager);
    }

    boolean validate() throws ParameterException {
        return args.validate(spec);
    }

    @Override
    public Integer call() {
        validate();
        var identifier = identifier();
        if (args.listMode())
            return listDefs();
        return findAndPrintTools(identifier);
    }

    private Integer listDefs() {
        return new BuildToolsPrinter(new PrintWriter(System.out), FormatterFactory.listFormatter(args.format()))
                .print(manager.stream());
    }

    private Integer findAndPrintTools(BuildToolIdentifier identifier) {
        var results = args.paths().parallel()
                .map(Path::of)
                .map(path -> Pair.of(path, survey(path, identifier)))
                .collect(Collectors.toList());
        return printAll(results);
    }

    private int printAll(List<Pair<Path, Either<Exception, List<Pair<Path, BuildTool>>>>> list) {
        return new ResultPrinter(new PrintWriter(System.out), FormatterFactory.resultFormatter(args.format()))
                .print(list);
    }

    private Either<Exception, List<Pair<Path, BuildTool>>> survey(Path path, BuildToolIdentifier identifier) {
        return identifier.identify(path, args.buildConfig());
    }

    public static void main(String[] args) {
        new CommandLine(new Main())
                .setCaseInsensitiveEnumValuesAllowed(true)
                .setParameterExceptionHandler(new UsageHandler())
                .execute(args);
    }
}
