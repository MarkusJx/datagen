package io.github.markusjx.datagen.schema;

import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;
import java.util.Map;
import org.intellij.lang.annotations.Language;

public class Transform {
    private final TransformType type;
    private String field;
    private String operator;
    private String other;
    private String pattern;
    private Boolean serializeNonStrings;
    private String by;
    private Boolean reverse;
    private String subType;
    private String name;
    private String format;
    private Map<String, Object> args;

    public Transform(@NotNull TransformType type) {
        this.type = type;
    }

    public @NotNull Transform field(@NotNull String field) {
        this.field = field;
        return this;
    }

    public @NotNull Transform operator(@NotNull String operator) {
        this.operator = operator;
        return this;
    }

    public @NotNull Transform other(@NotNull String other) {
        this.other = other;
        return this;
    }

    public @NotNull Transform pattern(@NotNull String pattern) {
        this.pattern = pattern;
        return this;
    }

    public @NotNull Transform serializeNonStrings(@NotNull Boolean serializeNonStrings) {
        this.serializeNonStrings = serializeNonStrings;
        return this;
    }

    public @NotNull Transform by(@NotNull String by) {
        this.by = by;
        return this;
    }

    public @NotNull Transform reverse(@NotNull Boolean reverse) {
        this.reverse = reverse;
        return this;
    }

    public @NotNull Transform subType(@NotNull String subType) {
        this.subType = subType;
        return this;
    }

    public @NotNull Transform name(@NotNull String name) {
        this.name = name;
        return this;
    }

    public @NotNull Transform format(@NotNull String format) {
        this.format = format;
        return this;
    }

    public @NotNull Transform args(@NotNull Map<String, Object> args) {
        this.args = args;
        return this;
    }

    public static @NotNull Transform filter(
            @NotNull String field, @NotNull String operator, @NotNull String other) {
        return new Transform(TransformType.FILTER).field(field).operator(operator).other(other);
    }

    public static @NotNull Transform filterNonNull(String field) {
        return new Transform(TransformType.FILTER_NON_NULL).field(field);
    }

    public static @NotNull Transform regexFilter(
            @NotNull String field, @NotNull @Language("RegExp") String pattern) {
        return new Transform(TransformType.REGEX_FILTER).field(field).pattern(pattern);
    }

    public static @NotNull Transform sort(@NotNull String by, @Nullable Boolean reverse) {
        return new Transform(TransformType.SORT).by(by).reverse(reverse);
    }

    public static @NotNull Transform toUpperCase(@Nullable Boolean serializeNonStrings) {
        return new Transform(TransformType.TO_UPPER_CASE).serializeNonStrings(serializeNonStrings);
    }

    public static @NotNull Transform toLowerCase(@Nullable Boolean serializeNonStrings) {
        return new Transform(TransformType.TO_LOWER_CASE).serializeNonStrings(serializeNonStrings);
    }

    public static @NotNull Transform toStringDefault() {
        return new Transform(TransformType.TO_STRING).subType("default");
    }

    public static @NotNull Transform toStringFormat(
            @NotNull @Language("handlebars") String format) {
        toStringFormat("{{name}}");
        return new Transform(TransformType.TO_STRING).subType("format").format(format);
    }

    public static @NotNull Transform plugin(
            @NotNull String name, @Nullable Map<String, Object> args) {
        return new Transform(TransformType.PLUGIN).name(name).args(args);
    }
}
