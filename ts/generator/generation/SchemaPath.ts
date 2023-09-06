export default class SchemaPath {
    private constructor(readonly path: string[]) {}

    public static root(): SchemaPath {
        return new SchemaPath([]);
    }

    public append(name: string): SchemaPath {
        return new SchemaPath([...this.path, name]);
    }

    public toString(): string {
        return this.path.join('.') || '.';
    }

    public toParentPath(): string {
        return this.path.slice(0, this.path.length - 1).join('.') || '.';
    }

    public toCommonPath(): string {
        return this.toString().replaceAll(/(\[(\d+)]|\d+\.)/g, '');
    }
}
