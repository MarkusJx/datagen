import Base from "./base";

export interface PluginSchema extends Base {
    type: 'generator';
    name: string;
    args?: any;
}