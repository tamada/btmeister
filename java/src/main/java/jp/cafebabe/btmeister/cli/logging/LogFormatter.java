package jp.cafebabe.btmeister.cli.logging;

import java.io.PrintWriter;
import java.io.StringWriter;
import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneId;
import java.time.format.DateTimeFormatter;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.logging.Formatter;
import java.util.logging.Level;
import java.util.logging.LogRecord;
import java.util.regex.Pattern;

/**
 * @see <a href="https://blog1.mammb.com/entry/2017/02/24/070608">Java の標準ロギングAPI JUL(java.util.logger) を少しマシにする</a>
 */
public class LogFormatter extends Formatter {
    private static final DateTimeFormatter formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS");
    private final AtomicInteger nameColumnWidth = new AtomicInteger(45);
    private static final Map<Level, String> levelMsgMap = Map.of(
            Level.SEVERE,  "SEVERE",
            Level.WARNING, "WARN  ",
            Level.INFO,    "INFO  ",
            Level.CONFIG,  "CONFIG",
            Level.FINE,    "FINE  ",
            Level.FINER,   "FINER ",
            Level.FINEST,  "FINEST"
    );

    @Override
    public String format(LogRecord record) {
        var buffer = new StringBuilder(200);
        buffer.append(dateString(record)).append(" ");
        buffer.append("[").append(categoryString(record)).append("] ");
        buffer.append(levelMsgMap.get(record.getLevel())).append(" ");
        buffer.append(formatMessage(record));
        buffer.append(System.lineSeparator());
        appendException(buffer, record);
        return new String(buffer);
    }

    private void appendException(StringBuilder buffer, LogRecord record) {
        if(record.getThrown() == null)
            return;
        try {
            StringWriter out = new StringWriter();
            PrintWriter pout = new PrintWriter(out);
            record.getThrown().printStackTrace(pout);
            pout.close();
            buffer.append(out);
        } catch(Exception e) {
            throw new InternalError(e);
        }
    }

    private String categoryString(LogRecord record) {
        String category;
        if (record.getSourceClassName() != null) {
            category = record.getSourceClassName();
            if (record.getSourceMethodName() != null) {
                category += " " + record.getSourceMethodName();
            }
        } else {
            category = record.getLoggerName();
        }
        int width = nameColumnWidth.intValue();
        return adjustLength(category, width);
    }

    static String adjustLength(String packageName, int aimLength) {
        int overflowWidth = packageName.length() - aimLength;

        String[] fragment = packageName.split(Pattern.quote("."));
        for (int i = 0; i < fragment.length - 1; i++) {
            if (fragment[i].length() > 1 && overflowWidth > 0) {
                int cutting = (fragment[i].length() - 1) - overflowWidth;
                cutting = (cutting < 0) ? (fragment[i].length() - 1) : overflowWidth;
                fragment[i] = fragment[i].substring(0, fragment[i].length() - cutting);
                overflowWidth -= cutting;
            }
        }
        StringBuilder result = new StringBuilder(String.join(".", fragment));
        while (result.length() < aimLength) {
            result.append(" ");
        }
        return new String(result);
    }

    private String dateString(LogRecord record) {
        Instant instant = Instant.ofEpochMilli(record.getMillis());
        LocalDateTime ldt = LocalDateTime.ofInstant(instant, ZoneId.systemDefault());
        return formatter.format(ldt);
    }
}