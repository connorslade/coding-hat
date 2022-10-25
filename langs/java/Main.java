class Main {
  public static void main(String[] args) throws Exception {
    String raw = "j2h5hd;ggm;\"mia\",\"egg\">\"egg mia\"";
    
    String[] data = raw.split(";");
    String sharedToken = data[0];
    String funcName = data[1];

    Object instance = Solution.class.getDeclaredConstructor().newInstance();
    Object output = Solution.class.getMethod(funcName).invoke(instance);
    System.out.println(output);
  }

  static String splitn(String inp, String sub, int count) {
    for (int i = 0; i < inp.length() - sub.length()) {
      // if ()
    }
  }
}