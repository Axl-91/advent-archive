import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
    public static void main(String[] args) {
        try {
            String input = Files.readString(Paths.get("input"));

            Pattern pattern = Pattern.compile("mul\\((\\d+),(\\d+)\\)");
            Matcher matcher = pattern.matcher(input);

            int total = 0;

            while (matcher.find()) {
                int firstValue = Integer.parseInt(matcher.group(1));
                int secondValue = Integer.parseInt(matcher.group(2));

                total += firstValue * secondValue;
            }
            System.out.println("Part 1 Result: " + total);

        } catch (IOException e) {
            System.err.println("Error reading file: " + e.getMessage());
        }
    }
}
