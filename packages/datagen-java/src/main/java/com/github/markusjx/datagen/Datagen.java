package com.github.markusjx.datagen;

import com.github.markusjx.datagen.generated.DatagenImpl;
import com.github.markusjx.datagen.generated.GenerateCallback;
import com.github.markusjx.datagen.schema.AnySchema;
import com.github.markusjx.jnibindgen.NativeExecutionException;
import com.google.gson.Gson;
import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;

public class Datagen {
    private final Gson gson;

    public Datagen() {
        this(null, null);
    }

    public Datagen(@Nullable String libraryPath) {
        this(libraryPath, null);
    }

    public Datagen(@Nullable String libraryPath, @Nullable Gson gson) {
        LibraryLoader.loadLibrary(libraryPath);
        this.gson = gson != null ? gson : new Gson();
    }

    public @NotNull String generateAsString(
            @NotNull String schema, @Nullable GenerateCallback callback
    )
            throws NativeExecutionException {
        return DatagenImpl.generateRandomData(schema, callback);
    }

    public @NotNull String generateAsString(@NotNull String schema)
            throws NativeExecutionException {
        return generateAsString(schema, null);
    }

    public @NotNull String generateAsString(
            @Nullable AnySchema<?> schema, @Nullable GenerateCallback callback
    )
            throws NativeExecutionException {
        return DatagenImpl.generateRandomData(gson.toJson(schema), callback);
    }

    public @NotNull String generateAsString(@Nullable AnySchema<?> schema)
            throws NativeExecutionException {
        return generateAsString(schema, null);
    }

    public <T> @NotNull T generate(
            @NotNull String schema, @NotNull Class<T> clazz,
            @Nullable GenerateCallback callback
    )
            throws NativeExecutionException {
        return gson.fromJson(generateAsString(schema, callback), clazz);
    }

    public <T> @NotNull T generate(@NotNull String schema, @NotNull Class<T> clazz)
            throws NativeExecutionException {
        return generate(schema, clazz, null);
    }

    public <T> @NotNull T generate(
            @Nullable AnySchema<?> schema,
            @NotNull Class<T> clazz,
            @Nullable GenerateCallback callback
    )
            throws NativeExecutionException {
        return gson.fromJson(generateAsString(schema, callback), clazz);
    }

    public <T> @NotNull T generate(@Nullable AnySchema<?> schema, @NotNull Class<T> clazz)
            throws NativeExecutionException {
        return generate(schema, clazz, null);
    }

    public @NotNull String getSchema() throws NativeExecutionException {
        return DatagenImpl.getSchema();
    }
}
