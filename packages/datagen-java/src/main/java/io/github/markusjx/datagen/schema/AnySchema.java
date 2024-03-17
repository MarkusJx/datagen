package io.github.markusjx.datagen.schema;

import jakarta.validation.constraints.NotNull;

public abstract class AnySchema<T extends AnySchema<T>> implements SerializableSchema {
    protected DatagenType type;

    protected AnySchema(@NotNull DatagenType type) {
        this.type = type;
    }

    @SuppressWarnings("unchecked")
    public T transform() {

        return (T) this;
    }
}
