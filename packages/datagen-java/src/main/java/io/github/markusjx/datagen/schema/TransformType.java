package io.github.markusjx.datagen.schema;

import com.google.gson.annotations.SerializedName;

public enum TransformType {
    @SerializedName("filter")
    FILTER,
    @SerializedName("filterNonNull")
    FILTER_NON_NULL,
    @SerializedName("regexFilter")
    REGEX_FILTER,
    @SerializedName("sort")
    SORT,
    @SerializedName("toUpperCase")
    TO_UPPER_CASE,
    @SerializedName("toLowerCase")
    TO_LOWER_CASE,
    @SerializedName("toString")
    TO_STRING,
    @SerializedName("plugin")
    PLUGIN
}
