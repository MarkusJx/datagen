package io.github.markusjx.datagen.schema;

import java.util.Map;

@SuppressWarnings("unused")
public class ArraySchema extends AnySchema<ArraySchema> {
    private Map<String, Integer> length;
    private SerializableSchema items;

    public ArraySchema() {
        super(DatagenType.ARRAY);
    }

    public ArraySchema fixedLength(int length) {
        this.length = Map.of("value", length);
        return this;
    }

    public ArraySchema randomLength(int min, int max) {
        this.length = Map.of("min", min, "max", max);
        return this;
    }

    public ArraySchema items(AnySchema<?> items) {
        this.items = items;
        return this;
    }
}
