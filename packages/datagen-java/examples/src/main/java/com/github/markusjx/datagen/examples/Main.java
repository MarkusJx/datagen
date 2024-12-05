package com.github.markusjx.datagen.examples;

import com.github.markusjx.datagen.Datagen;

public class Main {
    public static void main(String[] args) throws Exception {
        Datagen datagen = new Datagen();
        String generated = datagen.generateAsString("""
                {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "generator": {
                                "type": "fullName"
                            }
                        },
                        "age": {
                            "type": "number",
                            "min": 0,
                            "max": 100
                        }
                    }
                }
                """);

        System.out.println(generated);
    }
}