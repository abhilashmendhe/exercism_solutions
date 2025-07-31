public class LogLevels {
    
    public static String message(String logLine) {
		int ind = logLine.indexOf(':');
		return logLine.substring(ind+1).strip();
    }

    public static String logLevel(String logLine) {
    	int find = logLine.indexOf('[');
    	int sind = logLine.indexOf(']');
    	return logLine.substring(find+1, sind).toLowerCase();
    }

    public static String reformat(String logLine) {
    	return message(logLine) + " (" + logLevel(logLine) + ")";
    }
}
