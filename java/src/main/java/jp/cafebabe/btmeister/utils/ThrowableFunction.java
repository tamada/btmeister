package jp.cafebabe.btmeister.utils;

public interface ThrowableFunction<P, R, E extends Exception> {
    R map(P param) throws E;
}
