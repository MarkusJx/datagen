export type * from './types';
import SchemaJson from './schema.json';
export { SchemaJson };

/**
 * The current schema path.
 */
export interface CurrentSchema {
  /**
   * Create a child schema from this schema
   *
   * ## Example
   * ```ts
   * function generate(schema: CurrentSchema, args: string[]): Record<string, string> {
   *   const result: Record<string, string> = {};
   *   let child: CurrentSchema | null = null;
   *
   *   for (const arg of args) {
   *     child = schema.child(arg, child);
   *     other.generate(child, arg);
   *   }
   *
   *   return result;
   * }
   * ```
   *
   * @param path the path of the child schema
   * @param sibling a sibling schema to use as a base
   */
  child(path: string, sibling?: CurrentSchema | null): CurrentSchema;

  /**
   * Resolve a reference inside the schema
   *
   * @param path the path of the reference
   */
  resolveRef(path: string): any[];
}

export interface DatagenPlugin {
  /**
   * Generate a value
   *
   * @param schema the current schema
   * @param args the arguments passed to the plugin
   * @returns the generated value
   */
  generate?(schema: CurrentSchema, args: any): any | Promise<any>;

  /**
   * Transform a value
   *
   * @param schema the current schema
   * @param args the arguments passed to the plugin
   * @param value the value to transform
   * @returns the transformed value
   */
  transform?(schema: CurrentSchema, args: any, value: any): any | Promise<any>;

  /**
   * Serialize a value
   *
   * @param args the arguments passed to the plugin
   * @param value the value to serialize
   * @returns the serialized value
   */
  serialize?(args: any, value: any): string | Promise<string>;
}

export type PluginInitFunction = (
  args: any
) => DatagenPlugin | Promise<DatagenPlugin>;
