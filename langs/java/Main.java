import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;

class Main {
    public static void main(String[] args) throws Exception {
        String raw = "j2h5hd;ggm;2,\"egg\">\"egg egg\";3,\"cool\">\"cool cool cool\"";

        String[] data = raw.split(";");
        String sharedToken = data[0];
        String funcName = data[1];
        List<List<Object>> cases = extractCases(raw.substring(sharedToken.length() + funcName.length() + 2)).stream()
                .map(Main::parseCase)
                .toList();

        int i = 0;
        for (List<Object> _case : cases) {
            Object correctOutput = _case.remove(cases.size());
            Object instance = Solution.class.getDeclaredConstructor()
                    .newInstance();

            Optional<Method> runFun = Arrays.stream(Solution.class.getMethods())
                    .filter(m -> m.getName()
                            .equals(funcName))
                    .findFirst();

            if (runFun.isEmpty()) {
                System.out.printf("%s;ERROR;FUNC_DEF_NOT_FOUND", sharedToken);
                break;
            }

            Object output;
            try {
                output = runFun
                        .get()
                        .invoke(instance, _case.toArray());
            } catch (IllegalArgumentException ignored) {
                System.out.printf("%s;ERROR;INVALID_FUNC_SIG", sharedToken);
                break;
            }

            System.out.print("Case #" + i++ + " (" + output.equals(correctOutput) + ") | ");
            System.out.println(output);
        }
    }

    // TODO: Combine extractCases with parseCase
    static List<String> extractCases(String raw) {
        List<String> out = new ArrayList<>();
        StringBuilder working = new StringBuilder();
        boolean inString = false;

        for (char i : raw.toCharArray()) {
            if (i == '"')
                inString ^= true;

            if (i == ';' && !inString) {
                out.add(working.toString());
                working = new StringBuilder();
                continue;
            }

            working.append(i);
        }

        if (!working.isEmpty()) out.add(working.toString());
        return out;
    }

    static List<Object> parseCase(String raw) {
        List<Object> out = new ArrayList<>();
        StringBuilder working = new StringBuilder();
        boolean inString = false;

        for (char i : raw.toCharArray()) {
            if (i == '"')
                inString ^= true;

            if (i == ',' || i == '>' && !inString) {
                out.add(parseType(working.toString()).orElseThrow());
                working = new StringBuilder();
                continue;
            }

            working.append(i);
        }

        if (!working.isEmpty()) {
            out.add(parseType(working.toString()).orElseThrow());
        }
        return out;
    }

    // TODO: Finish types
    static Optional<Object> parseType(String raw) {
        raw = raw.trim();

        // String
        if (raw.startsWith("\"") && raw.endsWith("\"")) {
            return Optional.of(raw.substring(1, raw.length() - 1));
        }

        // Bool
        if (raw.equals("true")) return Optional.of(true);
        if (raw.equals("false")) return Optional.of(false);

        // Int
        try {
            return Optional.of(Integer.parseInt(raw));
        } catch (NumberFormatException ignored) {
        }

        // Float
        try {
            return Optional.of(Float.parseFloat(raw));
        } catch (NumberFormatException ignored) {
        }

        // Array

        // List

        return Optional.empty();
    }
}
