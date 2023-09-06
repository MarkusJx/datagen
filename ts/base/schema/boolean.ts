import Base from "./base";

export interface RandomBoolean extends Base {
    type: 'boolean';
    chance: number;
}

export interface ConstantBoolean extends Base {
    type: 'boolean';
    value: boolean;
}

type Boolean = RandomBoolean | ConstantBoolean;
export default Boolean;
