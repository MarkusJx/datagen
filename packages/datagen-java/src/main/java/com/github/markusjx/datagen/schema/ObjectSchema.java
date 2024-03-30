package com.github.markusjx.datagen.schema;

import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;
import java.util.HashMap;
import java.util.Map;

public class ObjectSchema extends AnySchema<ObjectSchema> {
    private Map<String, SerializableSchema> properties;

    public ObjectSchema() {
        super(DatagenType.OBJECT);
    }

    public @NotNull ObjectSchema properties(@NotNull Map<String, AnySchema<?>> properties) {
        this.properties = new HashMap<>(properties);
        return this;
    }

    public @NotNull ObjectSchema property(@NotNull String name, @Nullable AnySchema<?> schema) {
        if (this.properties == null) {
            this.properties = new HashMap<>();
        }

        this.properties.put(name, schema);
        return this;
    }
}
