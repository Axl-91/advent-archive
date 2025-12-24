import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
    private static void part1(String input) {
        Pattern pattern = Pattern.compile("mul\\((\\d+),(\\d+)\\)");
        Matcher matcher = pattern.matcher(input);

        int total = 0;

        while (matcher.find()) {
            int firstValue = Integer.parseInt(matcher.group(1));
            int secondValue = Integer.parseInt(matcher.group(2));

            total += firstValue * secondValue;
        }
        System.out.println("Part 1 Result: " + total);
    }

    private static void part2(String input) {
        Pattern pattern = Pattern.compile("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)");
        Matcher matcher = pattern.matcher(input);

        int total = 0;
        boolean addValue = true;

        while (matcher.find()) {
            if (matcher.group().equals("do()")) {
                addValue = true;
                continue;
            }
            if (matcher.group().equals("don't()")) {
                addValue = false;
                continue;
            }
            if (addValue) {
                int firstValue = Integer.parseInt(matcher.group(1));
                int secondValue = Integer.parseInt(matcher.group(2));

                total += firstValue * secondValue;
            }
        }
        System.out.println("Part 2 Result: " + total);
    }

    public static void main(String[] args) {
        try {
            String input = Files.readString(Paths.get("input"));

            part1(input);
            part2(input);
        } catch (IOException e) {
            System.err.println("Error reading file: " + e.getMessage());
        }
    }
}
