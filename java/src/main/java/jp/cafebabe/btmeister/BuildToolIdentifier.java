package jp.cafebabe.btmeister;

import io.vavr.control.Either;
import jp.cafebabe.btmeister.cli.Btmeister;
import jp.cafebabe.btmeister.utils.Pair;
import jp.cafebabe.diranger.Config;
import jp.cafebabe.diranger.Entry;
import jp.cafebabe.diranger.Ranger;
import jp.cafebabe.diranger.RangerBuilder;

import java.io.IOException;
import java.nio.file.Path;
import java.util.List;
import java.util.logging.Logger;
import java.util.stream.Stream;

public class BuildToolIdentifier {
    private final BuildToolManager manager;

    public BuildToolIdentifier(BuildToolManager manager) {
        this.manager = manager;
    }

    public Either<Exception, List<Pair<Path, BuildTool>>> identify(Path path, Config config) {
        Ranger ranger = RangerBuilder.build(RangerBuilder.Type.Simple);
        try {
            return Either.right(findTools(ranger.stream(path, config)));
        } catch(IOException e) {
            return Either.left(e);
        }
    }

    private List<Pair<Path, BuildTool>> findTools(Stream<Entry> stream) throws IOException {
        return stream
                .flatMap(e -> findTool(e.path()))
                .toList();
    }

    private Stream<Pair<Path, BuildTool>> findTool(Path path) {
        Logger.getLogger(Btmeister.NAME)
                .info(() -> String.format("findTool(%s)", path));
        return manager.find(path)
                .map(tool -> Pair.of(path, tool));
    }
}
