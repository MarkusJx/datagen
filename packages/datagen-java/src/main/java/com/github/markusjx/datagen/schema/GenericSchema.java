package com.github.markusjx.datagen.schema;

import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@SuppressWarnings("unused")
public class GenericSchema extends AnySchema<GenericSchema> {
    private Object value;
    private Number min;
    private Number max;
    private Integer start;
    private Integer stop;
    private Boolean pathSpecific;
    private Double probability;
    private List<SerializableSchema> values;
    private String pluginName;
    private Map<String, Object> args;
    private StringGenerator generator;
    private String path;
    private String mode;

    public GenericSchema(@NotNull DatagenType type) {
        super(type);
    }

    public @NotNull GenericSchema value(@Nullable Object value) {
        this.value = value;
        return this;
    }

    public @NotNull GenericSchema min(@Nullable Number min) {
        this.min = min;
        return this;
    }

    public @NotNull GenericSchema max(@Nullable Number max) {
        this.max = max;
        return this;
    }

    public @NotNull GenericSchema start(@Nullable Integer start) {
        this.start = start;
        return this;
    }

    public @NotNull GenericSchema stop(@Nullable Integer stop) {
        this.stop = stop;
        return this;
    }

    public @NotNull GenericSchema pathSpecific(@Nullable Boolean pathSpecific) {
        this.pathSpecific = pathSpecific;
        return this;
    }

    public @NotNull GenericSchema probability(@Nullable Double probability) {
        this.probability = probability;
        return this;
    }

    public @NotNull GenericSchema values(List<AnySchema<?>> values) {
        this.values = new ArrayList<>(values);
        return this;
    }

    public @NotNull GenericSchema pluginName(@Nullable String name) {
        this.pluginName = name;
        return this;
    }

    public @NotNull GenericSchema args(@Nullable Map<String, Object> args) {
        this.args = args;
        return this;
    }

    public @NotNull GenericSchema generator(@Nullable StringGenerator generator) {
        this.generator = generator;
        return this;
    }

    public @NotNull GenericSchema path(@Nullable String path) {
        this.path = path;
        return this;
    }

    public @NotNull GenericSchema mode(@Nullable String mode) {
        this.mode = mode;
        return this;
    }

    public static @NotNull GenericSchema integer(int value) {
        return new GenericSchema(DatagenType.INTEGER).value(value);
    }

    public static @NotNull GenericSchema integer(int min, int max) {
        return new GenericSchema(DatagenType.INTEGER).min(min).max(max);
    }

    public static @NotNull GenericSchema number(double value) {
        return new GenericSchema(DatagenType.NUMBER).value(value);
    }

    public static @NotNull GenericSchema number(double min, double max) {
        return new GenericSchema(DatagenType.NUMBER).min(min).max(max);
    }

    public static @NotNull GenericSchema string(@NotNull String value) {
        return new GenericSchema(DatagenType.STRING).value(value);
    }

    public static @NotNull GenericSchema string(@NotNull StringGenerator generator) {
        return new GenericSchema(DatagenType.STRING).generator(generator);
    }

    public static @NotNull GenericSchema bool(boolean value) {
        return new GenericSchema(DatagenType.BOOL).value(value);
    }

    public static @NotNull GenericSchema bool(double probability) {
        return new GenericSchema(DatagenType.BOOL).probability(probability);
    }

    public static @NotNull GenericSchema counter(
            @Nullable Integer start, @Nullable Integer stop, @Nullable Boolean pathSpecific) {
        return new GenericSchema(DatagenType.COUNTER)
                .start(start)
                .stop(stop)
                .pathSpecific(pathSpecific);
    }

    public static @NotNull GenericSchema anyOf(@NotNull List<AnySchema<?>> values) {
        return new GenericSchema(DatagenType.ANY_OF).values(values);
    }

    public static @NotNull GenericSchema anyOf(@NotNull AnySchema<?>... values) {
        return anyOf(List.of(values));
    }

    public static @NotNull GenericSchema flatten(@NotNull List<AnySchema<?>> values) {
        return new GenericSchema(DatagenType.FLATTEN).values(values);
    }

    public static @NotNull GenericSchema flatten(@NotNull AnySchema<?>... values) {
        return flatten(List.of(values));
    }

    public static @NotNull GenericSchema plugin(
            @NotNull String pluginName, @Nullable Map<String, Object> args) {
        return new GenericSchema(DatagenType.PLUGIN).pluginName(pluginName).args(args);
    }

    public static @NotNull GenericSchema file(@NotNull String path, @Nullable String mode) {
        return new GenericSchema(DatagenType.FILE).path(path).mode(mode);
    }
}
