export type * from './types';
import SchemaJson from './schema.json';
export { SchemaJson };
import { SchemaOptions } from './types';

type AnyFunction = (...args: any) => any;
type PromisifyFunction<T> = T extends AnyFunction
  ? (...args: Parameters<T>) => Promise<Awaited<ReturnType<T>>>
  : T;
type PromisifyObject<T> = {
  [K in keyof T]: PromisifyFunction<T[K]>;
};

export type ImportedPlugin = PromisifyObject<Required<DatagenPlugin>>;

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

  /**
   * Resolve a reference inside the schema asynchronously
   *
   * @param path the path of the reference
   */
  resolveRefAsync(path: string): Promise<any[]>;

  /**
   * Finalize a value.
   * This is required so this value can be found by references.
   */
  finalize<T>(value: T): T;

  /**
   * Finalize a value asynchronously
   */
  finalizeAsync<T>(value: T): Promise<T>;

  /**
   * Get the current path
   */
  path(): SchemaPath;

  /**
   * Get a plugin by name
   */
  getPlugin(name: string): ImportedPlugin;

  /**
   * Get the schema options
   */
  get options(): SchemaOptions;
}

export interface SchemaPath {
  /**
   * Append a path to the current path
   */
  append(path: string): SchemaPath;

  /**
   * Get the length of the path
   */
  len(): bigint;

  /**
   * Check if the path is empty
   */
  isEmpty(): boolean;

  /**
   * Get the normalized path
   */
  toNormalizedPath(): string;

  /**
   * Get the string representation of the path
   */
  toString(): string;
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
