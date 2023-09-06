export type InitFn<Args = any> = (args: Args) => void | Promise<void>;
export type TransformFn<Args = any, T = any, R = T> =(value: T, args: Args) => R | Promise<R>;
export type GenerateFn<Args = any, T = any> = (args: Args) => T | Promise<T>;
export type PluginExports = {
    init?: InitFn;
    transform?: TransformFn;
    generate?: GenerateFn;
};
