package com.github.markusjx.datagen;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.concurrent.atomic.AtomicBoolean;

public abstract class LibraryLoader {
    private static final AtomicBoolean LIBRARY_LOADED = new AtomicBoolean(false);
    private static final String LIBRARY_NAME = "datagen_java";

    private LibraryLoader() {
        throw new UnsupportedOperationException("LibraryLoader cannot be instantiated");
    }

    public static void loadLibrary(String path) {
        if (LIBRARY_LOADED.get()) {
            return;
        }

        synchronized (LIBRARY_LOADED) {
            if (LIBRARY_LOADED.get()) {
                return;
            }

            if (path != null && path.contains(File.separator)) {
                System.load(path);
            } else {
                try {
                    System.loadLibrary(path == null ? LIBRARY_NAME : path);
                } catch (UnsatisfiedLinkError ignored) {
                    loadLibraryInternal();
                }
            }
        }
    }

    private static void loadLibraryInternal() {
        try (LibraryInfo info = findLibrary()) {
            File outFile = File.createTempFile(info.name, null);
            outFile.deleteOnExit();

            try (FileOutputStream out = new FileOutputStream(outFile)) {
                info.stream.transferTo(out);
            }

            System.load(outFile.getAbsolutePath());
        } catch (IOException e) {
            throw new RuntimeException("Failed to load library from resources", e);
        }
    }

    private static LibraryInfo findLibrary() throws IOException {
        String fileName = System.mapLibraryName(LIBRARY_NAME);
        InputStream in = LibraryLoader.class.getResourceAsStream("/" + fileName);
        if (in == null) {
            throw new IOException("Library '%s' not found in resources".formatted(fileName));
        }

        return new LibraryInfo(in, fileName);
    }

    private record LibraryInfo(InputStream stream, String name) implements AutoCloseable {
        @Override
        public void close() throws IOException {
            stream.close();
        }
    }
}
