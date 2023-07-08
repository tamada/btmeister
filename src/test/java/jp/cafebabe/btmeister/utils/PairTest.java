package jp.cafebabe.btmeister.utils;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.Objects;
import java.util.function.Predicate;

public class PairTest {
    @Test
    public void testUnify() {
        var pair = Pair.of(10, "ten");
        Assertions.assertEquals(10, pair.unify((l, r) -> l).intValue());
        Assertions.assertEquals("ten", pair.unify((l, r) -> r));
        Assertions.assertEquals(10, pair.left());
        Assertions.assertEquals("ten", pair.right());
    }

    @Test
    public void testSwap() {
        var pair = Pair.of("ten", 10).swap();
        Assertions.assertEquals(10, pair.left());
        Assertions.assertEquals("ten", pair.right());
    }

    @Test
    public void testMap() {
        var pair = Pair.of(10, "ten")
                .map(l -> l * 10, r -> r + "-" + r);
        Assertions.assertEquals(100, pair.left());
        Assertions.assertEquals("ten-ten", pair.right());
    }

    @Test
    public void testTest() {
        var pair = Pair.of(10, "ten");
        Assertions.assertEquals(pair, Pair.of(5 * 2, "ten"));
    }

    @Test
    public void testTestAnd() {
        var pair = Pair.of(10, "ten");
        Assertions.assertTrue(pair.testAnd(predicate(10), predicate("ten")));
        Assertions.assertFalse(pair.testAnd(predicate(10), predicate("one")));
        Assertions.assertFalse(pair.testAnd(predicate(90), predicate("ten")));
        Assertions.assertFalse(pair.testAnd(predicate(90), predicate("one")));
    }

    @Test
    public void testTestOr() {
        var pair = Pair.of(10, "ten");
        Assertions.assertTrue(pair.testOr(predicate(10), predicate("ten")));
        Assertions.assertTrue(pair.testOr(predicate(10), predicate("one")));
        Assertions.assertTrue(pair.testOr(predicate(90), predicate("ten")));
        Assertions.assertFalse(pair.testOr(predicate(90), predicate("one")));
    }

    @Test
    public void testTest2() {
        var pair = Pair.of(10, "ten");
        Assertions.assertEquals(Pair.of(true, true), pair.test(predicate(10), predicate("ten")));
        Assertions.assertEquals(Pair.of(true, false), pair.test(predicate(10), predicate("one")));
        Assertions.assertEquals(Pair.of(false, true), pair.test(predicate(90), predicate("ten")));
        Assertions.assertEquals(Pair.of(false, false), pair.test(predicate(90), predicate("one")));
    }

    private <T> Predicate<T> predicate(T value) {
        return v -> Objects.equals(value, v);
    }

    @Test
    public void testEquals() {
        var pair = Pair.of(10, "ten");
        Assertions.assertNotEquals(pair, new Object());
        Assertions.assertNotEquals(pair, Pair.of(5, "ten"));
        Assertions.assertNotEquals(pair, Pair.of(10, "notTen"));
        Assertions.assertNotEquals(pair, Pair.of(5, "ten"));
        Assertions.assertEquals(pair, Pair.of(10, "ten"));
    }

    @Test
    public void testToString() {
        var pair = Pair.of(10, "ten");
        Assertions.assertEquals("(10, ten)", pair.toString());
    }

    @Test
    public void testInstantiateFailedEitherIsNull() {
        Assertions.assertThrows(NullPointerException.class,
                () -> Pair.of(null, "ten"));
        Assertions.assertThrows(NullPointerException.class,
                () -> Pair.of(10, null));
    }
}