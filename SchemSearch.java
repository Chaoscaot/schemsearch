public class SchemSearch {

    public static native String search(String schematicFile, String patternFile);

    public static void init(String filePath) {
        System.loadLibrary(filePath);
    }
}
