public class Main {
    public static void main(String[] args) {
        int num1 = 0;
        int num2 = 1;
        long sum = 0;

        while (num2 < 4_000_000) {
            int fibonacci = num1 + num2;
            num1 = num2;
            num2 = fibonacci;

            if (fibonacci % 2 == 0) {
                sum += fibonacci;
            }
        }
        System.out.println(sum);
    }
}
