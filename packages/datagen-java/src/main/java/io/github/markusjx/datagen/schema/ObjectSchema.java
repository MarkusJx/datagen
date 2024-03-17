package io.github.markusjx.datagen.schema;

import java.util.HashMap;
import java.util.Map;

public class ObjectSchema extends AnySchema<ObjectSchema> {
    private Map<String, SerializableSchema> properties;

    public ObjectSchema() {
        super(DatagenType.OBJECT);
    }

    public ObjectSchema properties(Map<String, AnySchema<?>> properties) {
        this.properties = new HashMap<>(properties);
        return this;
    }

    public ObjectSchema property(String name, AnySchema<?> schema) {
        if (this.properties == null) {
            this.properties = new HashMap<>();
        }

        this.properties.put(name, schema);
        return this;
    }
}
