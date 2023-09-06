export default class OperationNotSupportedError extends Error {
    public constructor(operation: string, plugin: string) {
        super(`Operation '${operation}' not supported for plugin '${plugin}'`);
    }
}
