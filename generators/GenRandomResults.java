import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.io.IOException;
import java.io.FileWriter;
import java.io.BufferedWriter;
import java.nio.file.Path;
import java.nio.file.Files;

public class GenRandomResults {

    static long seed = 12345;
    static Path outputPath = Path.of("./generated");

    public static void main(String[] args) {
        System.out.println("Running random data generator");
        // Generate output folder
        if (!Files.isDirectory(outputPath)) {
            try {
                Files.createDirectory(outputPath);
            } catch (IOException ioException) {
                System.err.println("couldn't create the output folder");
                System.err.println(ioException);
            }
        }

        Random rand = new Random(seed);
        List<String> integers = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            integers.add(String.valueOf(rand.nextInt()));
        }

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputPath.resolve("integers.data").toFile()))) {
            writer.write(String.format("[%s]", String.join(",", integers)));
        } catch (IOException ioException) {
            System.err.println("writing generated test data failed");
            System.err.println(ioException);
        }

        rand = new Random(seed);
        List<String> longs = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            longs.add(String.valueOf(rand.nextLong()));
        }

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputPath.resolve("longs.data").toFile()))) {
            writer.write(String.format("[%s]", String.join(",", longs)));
        } catch (IOException ioException) {
            System.err.println("writing generated test data failed");
            System.err.println(ioException);
        }

        rand = new Random(seed);
        List<String> floats = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            floats.add(String.valueOf(rand.nextFloat()));
        }

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputPath.resolve("floats.data").toFile()))) {
            writer.write(String.format("[%s]", String.join(",", floats)));
        } catch (IOException ioException) {
            System.err.println("writing generated test data failed");
            System.err.println(ioException);
        }

        rand = new Random(seed);
        List<String> doubles = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            doubles.add(String.valueOf(rand.nextDouble()));
        }

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputPath.resolve("doubles.data").toFile()))) {
            writer.write(String.format("[%s]", String.join(",", doubles)));
        } catch (IOException ioException) {
            System.err.println("writing generated test data failed");
            System.err.println(ioException);
        }

        rand = new Random(seed);
        List<String> booleans = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            booleans.add(String.valueOf(rand.nextBoolean()));
        }

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputPath.resolve("booleans.data").toFile()))) {
            writer.write(String.format("[%s]", String.join(",", booleans)));
        } catch (IOException ioException) {
            System.err.println("writing generated test data failed");
            System.err.println(ioException);
        }

        rand = new Random(seed);
        byte[] bytes = new byte[10];
        rand.nextBytes(bytes);

        try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputPath.resolve("bytes.data").toFile()))) {
            writer.write("[");
            for (byte b : bytes) {
                writer.write(String.valueOf(b));
                writer.write(",");
            }
            writer.write("]");
        } catch (IOException ioException) {
            System.err.println("writing generated test data failed");
            System.err.println(ioException);
        }
    }

}