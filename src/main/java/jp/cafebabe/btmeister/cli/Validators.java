package jp.cafebabe.btmeister.cli;

import jp.cafebabe.btmeister.utils.Pair;
import picocli.CommandLine.Model.CommandSpec;
import picocli.CommandLine.ParameterException;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Stream;

class Validators {
    private final Map<String, Pair<PathValidator, String>> maps;

    private final CommandSpec spec;

    public Validators(CommandSpec spec) {
        this.spec = spec;
        maps = Map.of(
                "exists", Pair.of(Files::exists, "%s: file or directory not found"),
                "file", Pair.of(Files::isRegularFile, "%s: not regular file"),
                "dir", Pair.of(Files::isDirectory, "%s: is not directory"),
                "readable", Pair.of(Files::isReadable, "%s: is not readable")
        );
    }

    private interface PathValidator {
        boolean validatePath(Path path) throws ParameterException;

        default void throwExceptionIfInvalidPath(CommandSpec spec, Path path, String message) {
            if(!validatePath(path))
                throw new ParameterException(spec.commandLine(), String.format(message, path));
        }
    }

    private boolean isNoOperationSpecified(List<String> paths, boolean listDefs) {
        return paths.size() == 0 && !listDefs;
    }

    private boolean isMultipleOperationSpecified(List<String> paths, boolean listDefs) {
        return paths.size() > 0 && listDefs;
    }

    public void validateRunMode(List<String> paths, boolean listDefs) throws ParameterException {
        if(isNoOperationSpecified(paths, listDefs))
            throw new ParameterException(spec.commandLine(), "no operation was specified. it needs project paths or list-defs option.");
        if(isMultipleOperationSpecified(paths, listDefs))
            throw new ParameterException(spec.commandLine(), "multiple operations were specified. it requires either project paths or list-defs option.");
    }

    public void validateAppendDefs(boolean appendDefs, Path path) throws ParameterException {
        if(appendDefs && path == null)
            throw new ParameterException(spec.commandLine(), "append-def option requires the value of definition option.");
        Optional.ofNullable(path)
                .ifPresent(p -> validatePath(p, "exists", "file", "readable"));
    }

    public void validateDefFile(Path path) throws ParameterException {
        Optional.ofNullable(path)
                .ifPresent(p -> validatePath(p, "exists", "file", "readable"));
    }

    public void validateArguments(List<String> paths) throws ParameterException {
        validateProjectArguments(paths.stream().filter(p -> !p.startsWith("@")));
        validateAtMarkArguments(paths.stream().filter(p -> p.startsWith("@")));
    }

    private void validateAtMarkArguments(Stream<String> paths) throws ParameterException {
        paths.map(p -> p.substring(1))
                .forEach(p -> validatePath(Path.of(p), "exists", "file", "readable"));
    }

    private void validateProjectArguments(Stream<String> paths) throws ParameterException {
        paths.filter(p -> !Objects.equals(p, "-"))
                .forEach(p -> validatePath(Path.of(p), "exists", "readable", "dir"));
    }

    private void validatePath(Path path, String... params) throws ParameterException {
        Arrays.stream(params)
                .map(maps::get)
                .forEach(pair -> pair.accept((v, m) -> v.throwExceptionIfInvalidPath(spec, path, m)));
    }
}
