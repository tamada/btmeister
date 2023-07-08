package jp.cafebabe.btmeister.cli.io;

import java.io.PrintWriter;
import java.nio.file.Path;
import java.util.Optional;

public interface Formatter<E> {
    void header(PrintWriter out);

    void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe);

    void afterEntry(PrintWriter out);

    void formatEntry(PrintWriter out, E entry, boolean first);

    void footer(PrintWriter out);

    abstract class Base<E> implements Formatter<E> {
        @Override
        public void header(PrintWriter out) {}

        @Override
        public void beforeEntry(PrintWriter out, Path projectPath, Optional<Exception> oe) {}

        @Override
        public void afterEntry(PrintWriter out){}

        public abstract void formatEntry(PrintWriter out, E entry, boolean first);

        @Override
        public void footer(PrintWriter out){}

        protected String stripBasePath(Path projectPath, Path path) {
            return projectPath.relativize(path)
                    .toString();
        }
    }
}
