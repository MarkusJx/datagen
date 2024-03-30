package com.github.markusjx.datagen.schema;

import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;
import java.util.Map;

@SuppressWarnings("unused")
public class ArraySchema extends AnySchema<ArraySchema> {
    private Map<String, Integer> length;
    private SerializableSchema items;

    public ArraySchema() {
        super(DatagenType.ARRAY);
    }

    public @NotNull ArraySchema fixedLength(int length) {
        this.length = Map.of("value", length);
        return this;
    }

    public @NotNull ArraySchema randomLength(int min, int max) {
        this.length = Map.of("min", min, "max", max);
        return this;
    }

    public @NotNull ArraySchema items(@Nullable AnySchema<?> items) {
        this.items = items;
        return this;
    }
}
