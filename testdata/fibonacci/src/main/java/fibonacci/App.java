package fibonacci;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class App {
    private List<Integer> cache = new ArrayList<>();

    public App() {
        cache.add(1);
        cache.add(1);
    }

    public void run(String[] args) {
        List<Integer> list = convert(args);
        list.stream().forEach(this::perform);
    }

    public void perform(int index) {
        int value = fibonacci(index);
        System.out.printf("fibonacci(%d) = %d%n", index, value);
    }

    private int fibonacci(int index) {
        if(cache.size() < index) {
            for(int i = cache.size(); i < index; i++) {
                cache.add(cache.get(i - 1) + cache.get(i - 2));
            }
        }
        return cache.get(index - 1);
    }

    private List<Integer> convert(String[] args) {
        return Arrays.stream(args.length == 0? new String[] { "10" }: args)
            .map(Integer::valueOf)
            .toList(); 
    }

    public static void main(String[] args) {
        new App().run(args);
    }
}
