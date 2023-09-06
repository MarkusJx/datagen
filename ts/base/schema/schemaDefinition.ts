import Any from "./any";

export interface SchemaOptions {
    /**
     * Plugins to use for this schema.
     */
    plugins?: Record<string, any>;
    /**
     * Whether to ignore references that cannot be resolved.
     * Defaults to false. If true, the value of the reference
     * will be the value of the notFoundRefValue option.
     */
    ignoreNotFoundLocalRefs?: boolean;
    /**
     * Whether to ignore global references that cannot be resolved.
     * Defaults to false. If true, the value of the reference
     * will be the value of the notFoundRefValue option.
     */
    ignoreNotFoundGlobalRefs?: boolean;
    /**
     * Whether to let global references reference themselves.
     * Defaults to false. If true, the value of the reference
     * will be the value of the notFoundRefValue option.
     * Only works for primitive types.
     */
    doNotReferenceSelf?: boolean;
    /**
     * The value to use when a reference cannot be resolved.
     * Defaults to null.
     */
    notFoundRefValue?: any;
}

type SchemaDefinitionOptions = {
    /**
     * The options for this schema.
     */
    options?: SchemaOptions;
};

type SchemaDefinition = Any & SchemaDefinitionOptions;
export default SchemaDefinition;
