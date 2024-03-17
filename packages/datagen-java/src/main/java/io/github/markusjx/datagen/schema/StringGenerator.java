package io.github.markusjx.datagen.schema;

import java.util.Map;

public class StringGenerator {
    private String type;
    private String format;
    private Map<String, Object> args;

    public StringGenerator(String type) {
        this.type = type;
    }

    public StringGenerator format(String format) {
        this.format = format;
        return this;
    }

    public StringGenerator args(Map<String, Object> args) {
        this.args = args;
        return this;
    }
}
