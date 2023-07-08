package jp.cafebabe.btmeister;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
import java.io.InputStream;
import java.net.URL;

class BuildToolJsonParser {
    private final InputStream stream;

    private BuildToolJsonParser(InputStream stream) {
        this.stream = stream;
    }

    public BuildToolManager parse() throws IOException {
        return new BuildToolManager(new ObjectMapper().readValue(stream, new TypeReference<>() {
        }));
    }

    public static BuildToolManager parse(InputStream stream) throws IOException {
        return new BuildToolJsonParser(stream)
                .parse();
    }
}
