package io.github.markusjx.datagen.schema;

import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;
import java.util.Map;

public class StringGenerator {
    private String type;
    private String format;
    private Map<String, String> args;

    public StringGenerator(@NotNull String type) {
        this.type = type;
    }

    public @NotNull StringGenerator format(@Nullable String format) {
        this.format = format;
        return this;
    }

    public @NotNull StringGenerator args(@Nullable Map<String, String> args) {
        this.args = args;
        return this;
    }

    public static @NotNull StringGenerator format(
            @NotNull String format, @NotNull Map<String, String> args) {
        return new StringGenerator("format").format(format).args(args);
    }
}
