package jp.cafebabe.btmeister.utils;

import java.util.Map;
import java.util.Objects;
import java.util.function.*;

/**
 * The instance from this class holds two different instance values.
 *
 * to get the value of this instance, use {@link #unify <code>unify</code>} method.
 * For example,
 * <code>
 *     Pair<L, R> pair = Pair.of(...);
 *     L left = pair.unify((l, r) -> l);
 * </code>
 */
public class Pair<L, R> {
    private L left;
    private R right;

    public static <L, R> Pair<L, R> of(L left, R right) {
        return new Pair<>(left, right);
    }

    public static <K, V> Pair<K, V> of(Map.Entry<K, V> entry) {
        return new Pair<>(entry.getKey(), entry.getValue());
    }

    public Pair<R, L> swap() {
        return of(right, left);
    }

    public L left() {
        return left;
    }

    public R right() {
        return right;
    }

    public void accept(BiConsumer<L, R> action) {
        action.accept(left, right);
    }

    public void accept(Consumer<L> leftAction, Consumer<R> rightAction) {
        leftAction.accept(left);
        rightAction.accept(right);
    }

    public <K> K unify(BiFunction<L, R, K> mapper) {
        return mapper.apply(left, right);
    }

    public <LL, RR> Pair<LL, RR> map(Function<L, LL> leftMapper, Function<R, RR> rightMapper) {
        return of(leftMapper.apply(left),
                rightMapper.apply(right));
    }

    public boolean testAnd(Predicate<L> leftPredicate, Predicate<R> rightPredicate) {
        return leftPredicate.test(left) && rightPredicate.test(right);
    }

    public boolean testOr(Predicate<L> leftPredicate, Predicate<R> rightPredicate) {
        return leftPredicate.test(left) || rightPredicate.test(right);
    }

    public boolean test(BiPredicate<L, R> predicate) {
        return predicate.test(left, right);
    }

    public Pair<Boolean, Boolean> test(Predicate<L> leftPredicate, Predicate<R> rightPredicate) {
        return map(l -> leftPredicate.test(l),
                r -> rightPredicate.test(r));
    }

    @Override
    public int hashCode() {
        return Objects.hash(left, right);
    }

    @Override
    @SuppressWarnings("unchecked")
    public boolean equals(Object other) {
        return (other instanceof Pair) &&
                ((Pair) other).test((l, r) -> Objects.equals(l, left) && Objects.equals(r, right));
    }

    @Override
    public String toString() {
        return String.format("(%s, %s)", left, right);
    }

    private Pair(L left, R right) {
        this.left = Objects.requireNonNull(left);
        this.right = Objects.requireNonNull(right);
    }
}