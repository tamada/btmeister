package jp.cafebabe.btmeister.matchers;

import java.nio.file.Path;
import java.util.function.Predicate;

public interface BuildToolMatcher extends Predicate<Path> {
}
