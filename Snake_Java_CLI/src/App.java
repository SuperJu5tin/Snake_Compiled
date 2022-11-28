import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class App {
    public static void main(String[] args) throws Exception {
        BufferedReader reader = new BufferedReader(
        new InputStreamReader(System.in));
        
        System.out.println("\033[H\033[2J");
        System.out.println("Would you like to die on snake collision");
        System.out.flush();
        String difficulty = reader.readLine();

        System.out.println(name);
    }
}
