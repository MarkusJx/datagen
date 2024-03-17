package io.github.markusjx.datagen.schema;

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
    private Double probability;
    private List<SerializableSchema> values;
    private Map<String, Object> args;
    private StringGenerator generator;
    private String path;
    private String mode;

    public GenericSchema(@NotNull DatagenType type) {
        super(type);
    }

    public GenericSchema value(Object value) {
        this.value = value;
        return this;
    }

    public GenericSchema min(Number min) {
        this.min = min;
        return this;
    }

    public GenericSchema max(Number max) {
        this.max = max;
        return this;
    }

    public GenericSchema start(Integer start) {
        this.start = start;
        return this;
    }

    public GenericSchema probability(Double probability) {
        this.probability = probability;
        return this;
    }

    public GenericSchema values(List<AnySchema<?>> values) {
        this.values = new ArrayList<>(values);
        return this;
    }

    public GenericSchema args(Map<String, Object> args) {
        this.args = args;
        return this;
    }

    public GenericSchema generator(StringGenerator generator) {
        this.generator = generator;
        return this;
    }

    public GenericSchema path(String path) {
        this.path = path;
        return this;
    }

    public GenericSchema mode(String mode) {
        this.mode = mode;
        return this;
    }
}
