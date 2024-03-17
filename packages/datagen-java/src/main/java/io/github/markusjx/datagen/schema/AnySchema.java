package io.github.markusjx.datagen.schema;

import jakarta.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.List;

public abstract class AnySchema<T extends AnySchema<T>> implements SerializableSchema {
    protected DatagenType type;
    private List<Transform> transform;

    protected AnySchema(@NotNull DatagenType type) {
        this.type = type;
    }

    @SuppressWarnings("unchecked")
    public @NotNull T transform(@NotNull Transform transform) {
        if (this.transform == null) {
            this.transform = new ArrayList<>();
        }

        this.transform.add(transform);
        return (T) this;
    }
}
