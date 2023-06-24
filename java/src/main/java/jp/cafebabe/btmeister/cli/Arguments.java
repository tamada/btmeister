package jp.cafebabe.btmeister.cli;

import io.vavr.control.Either;
import io.vavr.control.Try;
import jp.cafebabe.btmeister.BuildToolManager;
import jp.cafebabe.btmeister.cli.io.Format;
import jp.cafebabe.diranger.Config;
import picocli.CommandLine.Mixin;
import picocli.CommandLine.Model.CommandSpec;
import picocli.CommandLine.Option;
import picocli.CommandLine.ParameterException;
import picocli.CommandLine.Parameters;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Optional;
import java.util.logging.Logger;
import java.util.stream.Stream;

class Arguments {
    @Option(names = {"-f", "--format"}, paramLabel = "FORMAT", description = "specify the output format")
    private Format format = Format.Default;

    @Option(names = {"-d", "--definition"}, paramLabel = "BUILD_FILE_DEFS", description = "specify the definitions of build files.")
    private Path buildFileDef = null;

    @Option(names = {"--append-defs"}, description = "set to additional definitions by --definition option. This option requires --definition option.")
    private boolean appendDefs = false;

    @Option(names = {"-l", "--list-defs"}, description = "list definitions and exit.")
    private boolean listDefs = false;

    @Mixin
    private LoggerConfig logger = new LoggerConfig();

    @Mixin
    private TraverseConfig config = new TraverseConfig();

    @Parameters(arity = "0..*", description = "target projects. If \"-\" is given, reads from stdin, and \"@\" in the first character, reads project list from the file", paramLabel = "PROJECT...")
    private List<String> paths = new ArrayList<>();

    public Stream<String> paths() {
        return Optional.ofNullable(paths)
                .stream().flatMap(Collection::stream);
    }

    public boolean listMode() {
        return listDefs;
    }

    boolean validate(CommandSpec spec) {
        logger.validate(spec);
        constructValidators(spec)
                .forEach(v -> v.validate(this));
        Logger.getLogger(Btmeister.NAME)
                .info("validate arguments done");
        return true;
    }

    public Config buildConfig() {
        return config.build();
    }

    @FunctionalInterface
    private interface Validator {
        void validate(Arguments args) throws ParameterException;
    }

    private List<Validator> constructValidators(CommandSpec spec) {
        Validators v = new Validators(spec);
        Validator firstValidator = args -> v.validateRunMode(args.paths, args.listDefs);
        return List.of(
                firstValidator,
                args -> v.validateAppendDefs(args.appendDefs, args.buildFileDef),
                args -> v.validateDefFile(args.buildFileDef),
                args -> v.validateArguments(args.paths)
        );
    }

    public Either<Throwable, BuildToolManager> manager() {
        var defaultManager = loadDefault();
        var buildDefs = Optional.ofNullable(buildFileDef)
                .map(this::loadManager);
        return buildDefs.map(either -> either.map(btm -> merge(btm, defaultManager)))
                .orElseGet(() -> Either.right(defaultManager.get()));
    }

    private BuildToolManager merge(BuildToolManager base, Optional<BuildToolManager> other) {
        if(other.isEmpty())
            return base;
        return base.merge(other.get());
    }

    private Optional<BuildToolManager> loadDefault() {
        if(appendDefs || buildFileDef == null)
            return Try.of(BuildToolManager::getDefault)
                    .onFailure(Exception.class, e -> System.out.println(e.getMessage()))
                    .toJavaOptional();
        return Optional.empty();
    }

    private Either<Throwable, BuildToolManager> loadManager(Path path) {
        return Try.of(() -> BuildToolManager.loadManager(path.toUri().toURL()))
                .toEither();
    }

    public Format format() {
        return format;
    }
}
