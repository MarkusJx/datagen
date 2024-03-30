package com.github.markusjx.datagen;

import com.github.markusjx.datagen.schema.AnySchema;
import com.github.markusjx.datagen.schema.GenericSchema;
import com.github.markusjx.datagen.schema.ObjectSchema;
import com.github.markusjx.datagen.schema.StringGenerator;
import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class DatagenTest {
    private static final String SCHEMA_STRING =
            """
            {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "generator": {
                            "type": "uuid"
                        }
                    },
                    "number": {
                        "type": "integer",
                        "min": 0,
                        "max": 100
                    }
                }
            }""";
    private static final AnySchema<?> SCHEMA =
            new ObjectSchema()
                    .property("id", GenericSchema.string(new StringGenerator("uuid")))
                    .property("number", GenericSchema.integer(0, 100));
    private static final Gson gson = new Gson();

    @Test
    public void testGenerateAsString() throws Exception {
        Datagen datagen = new Datagen();
        String generated = datagen.generateAsString(SCHEMA_STRING);

        checkResult(generated);
    }

    @Test
    public void testGenerate() throws Exception {
        Datagen datagen = new Datagen();
        BasicObject result = datagen.generate(SCHEMA_STRING, BasicObject.class);

        checkResult(result);
    }

    @Test
    public void testGenerateWithAnySchema() throws Exception {
        Datagen datagen = new Datagen();
        BasicObject result = datagen.generate(SCHEMA, BasicObject.class);

        checkResult(result);
    }

    @Test
    public void testGenerateAsStringWithAnySchema() throws Exception {
        Datagen datagen = new Datagen();
        String generated = datagen.generateAsString(SCHEMA);

        checkResult(generated);
    }

    @Test
    public void testGenerateAsStringWithCallback() throws Exception {
        Datagen datagen = new Datagen();
        List<Progress> progress = new ArrayList<>();

        String generated =
                datagen.generateAsString(
                        SCHEMA_STRING, (cur, total) -> progress.add(new Progress(cur, total)));

        checkResult(generated);
        checkProgress(progress);
    }

    @Test
    public void testGenerateWithCallback() throws Exception {
        Datagen datagen = new Datagen();
        List<Progress> progress = new ArrayList<>();

        BasicObject result =
                datagen.generate(
                        SCHEMA_STRING,
                        BasicObject.class,
                        (cur, total) -> progress.add(new Progress(cur, total)));

        checkResult(result);
        checkProgress(progress);
    }

    @Test
    public void testGenerateWithAnySchemaAndCallback() throws Exception {
        Datagen datagen = new Datagen();
        List<Progress> progress = new ArrayList<>();

        BasicObject result =
                datagen.generate(
                        SCHEMA,
                        BasicObject.class,
                        (cur, total) -> progress.add(new Progress(cur, total)));

        checkResult(result);
        checkProgress(progress);
    }

    @Test
    public void testGenerateAsStringWithAnySchemaAndCallback() throws Exception {
        Datagen datagen = new Datagen();
        List<Progress> progress = new ArrayList<>();

        String generated =
                datagen.generateAsString(
                        SCHEMA, (cur, total) -> progress.add(new Progress(cur, total)));

        checkResult(generated);
        checkProgress(progress);
    }

    @Test
    public void testGetSchema() throws Exception {
        Datagen datagen = new Datagen();
        String schema = datagen.getSchema();

        Assertions.assertNotNull(schema);
    }

    private static void checkProgress(List<Progress> progress) {
        Assertions.assertEquals(3, progress.size());
        Assertions.assertEquals(1, progress.get(0).current());
        Assertions.assertEquals(2, progress.get(1).current());
        Assertions.assertEquals(3, progress.get(2).current());
        Assertions.assertEquals(3, progress.get(0).total());
        Assertions.assertEquals(3, progress.get(1).total());
        Assertions.assertEquals(3, progress.get(2).total());
    }

    private static void checkResult(BasicObject result) {
        Assertions.assertNotNull(result);
        Assertions.assertNotNull(result.id);
        Assertions.assertTrue(result.number >= 0 && result.number <= 100);
    }

    private static void checkResult(String generated) {
        BasicObject result = gson.fromJson(generated, BasicObject.class);
        checkResult(result);
    }

    private record BasicObject(UUID id, int number) {}

    private record Progress(int current, int total) {}
}
