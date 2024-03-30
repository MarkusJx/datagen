package com.github.markusjx.datagen;

import com.github.markusjx.datagen.generated.DatagenImpl;
import com.github.markusjx.datagen.generated.GenerateCallback;
import com.github.markusjx.datagen.schema.AnySchema;
import com.github.markusjx.jnibindgen.NativeExecutionException;
import com.google.gson.Gson;
import jakarta.annotation.Nullable;
import jakarta.validation.constraints.NotNull;
import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public class Datagen {
    private static final AtomicBoolean LIBRARY_LOADED = new AtomicBoolean(false);
    private final Gson gson;

    public Datagen() {
        this(null, null);
    }

    public Datagen(@Nullable String libraryPath) {
        this(libraryPath, null);
    }

    public Datagen(@Nullable String libraryPath, @Nullable Gson gson) {
        loadLibrary(libraryPath);
        this.gson = gson != null ? gson : new Gson();
    }

    public @NotNull String generateAsString(
            @NotNull String schema, @Nullable GenerateCallback callback)
            throws NativeExecutionException {
        return DatagenImpl.generateRandomData(schema, callback);
    }

    public @NotNull String generateAsString(@NotNull String schema)
            throws NativeExecutionException {
        return generateAsString(schema, null);
    }

    public @NotNull String generateAsString(
            @Nullable AnySchema<?> schema, @Nullable GenerateCallback callback)
            throws NativeExecutionException {
        return DatagenImpl.generateRandomData(gson.toJson(schema), callback);
    }

    public @NotNull String generateAsString(@Nullable AnySchema<?> schema)
            throws NativeExecutionException {
        return generateAsString(schema, null);
    }

    public <T> @NotNull T generate(
            @NotNull String schema, @NotNull Class<T> clazz, @Nullable GenerateCallback callback)
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
            @Nullable GenerateCallback callback)
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

    private static void loadLibrary(@Nullable String path) {
        synchronized (LIBRARY_LOADED) {
            if (!LIBRARY_LOADED.get()) {
                if (path != null && path.contains(File.separator)) {
                    System.load(path);
                } else {
                    System.loadLibrary(path == null ? "datagen_java" : path);
                }
                LIBRARY_LOADED.set(true);
            }
        }
    }
}
