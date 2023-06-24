package jp.cafebabe.btmeister.cli;

import picocli.CommandLine.IVersionProvider;

import java.io.InputStream;
import java.util.Properties;

public class Btmeister implements IVersionProvider {
    public static final String NAME = "btmeister";

    @Override
    public String[] getVersion() throws Exception {
        var props = new Properties();
        try(InputStream in = Btmeister.class.getResourceAsStream("/resources/info.properties")) {
            props.load(in);
        }
        return new String[] {
                props.getProperty("btmeister.version")
        };
    }
}
