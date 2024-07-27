package com.github.markusjx.datagen.schema;

import com.google.gson.annotations.SerializedName;

public enum DatagenType {
    @SerializedName("array")
    ARRAY,
    @SerializedName("object")
    OBJECT,
    @SerializedName("integer")
    INTEGER,
    @SerializedName("number")
    NUMBER,
    @SerializedName("string")
    STRING,
    @SerializedName("bool")
    BOOL,
    @SerializedName("counter")
    COUNTER,
    @SerializedName("anyOf")
    ANY_OF,
    @SerializedName("flatten")
    FLATTEN,
    @SerializedName("plugin")
    PLUGIN,
    @SerializedName("file")
    FILE,
    @SerializedName("null")
    NULL
}
