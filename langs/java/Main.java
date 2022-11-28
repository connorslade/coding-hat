import java.io.UnsupportedEncodingException;
import java.lang.reflect.Method;
import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;

class Main {
    public static void main(String[] args) throws Exception {
        String raw = urlDecode(System.getenv("DATA"));
        // TODO: clear DATA env var

        String[] data = raw.split(";");
        String sharedToken = data[0];
        String funcName = data[1];
        List<List<Object>> cases = extractCases(raw.substring(sharedToken.length() + funcName.length() + 2)).stream()
                .map(Main::parseCase)
                .toList();

        List<Pair<Boolean, String>> out = new ArrayList<>();
        for (List<Object> _case : cases) {
            Object correctOutput = _case.remove(_case.size() - 1);
            Object instance = Solution.class.getDeclaredConstructor()
                    .newInstance();

            Optional<Method> runFun = Arrays.stream(Solution.class.getMethods())
                    .filter(m -> m.getName()
                            .equals(funcName))
                    .findFirst();

            if (runFun.isEmpty()) {
                System.err.printf("%s;ERROR;FUNC_DEF_NOT_FOUND\n", sharedToken);
                break;
            }

            Object output;
            try {
                output = runFun
                        .get()
                        .invoke(instance, _case.toArray());
            } catch (IllegalArgumentException ignored) {
                System.err.printf("%s;ERROR;INVALID_FUNC_SIG\n", sharedToken);
                break;
            }

            out.add(new Pair<>(output.equals(correctOutput), stringifyType(output)));
        }

        StringBuilder outStr = new StringBuilder();
        for (Pair<Boolean, String> pair : out)
            outStr.append(pair.key ? "P" : "F")
                    .append(";");
        for (Pair<Boolean, String> pair : out)
            outStr.append(pair.value)
                    .append(";");

        if (outStr.length() > 0) outStr.deleteCharAt(outStr.length() - 1);
        System.err.printf("%s;RESULT;%s", sharedToken, outStr);
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

    static String stringifyType(Object obj) {
        if (obj instanceof String) return "\"" + obj + "\"";
        if (obj instanceof Boolean) return obj.toString();
        if (obj instanceof Integer) return obj.toString();
        if (obj instanceof Float) return obj.toString();

        // TODO: Array + List

        return obj.toString();
    }

    static String urlDecode(String value) {
        return URLDecoder.decode(value, StandardCharsets.UTF_8);
    }

    static class Pair<K, V> {
        public K key;
        public V value;

        public Pair(K key, V value) {
            this.key = key;
            this.value = value;
        }
    }
}
