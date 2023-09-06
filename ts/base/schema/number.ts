import Base from "./base";

export interface RandomNumber extends Base {
    type: 'number';
    min?: number;
    max?: number;
    step?: number;
    decimalPlaces?: number;
}

export interface ConstantNumber extends Base {
    type: 'number';
    value: number;
}

type Number = RandomNumber | ConstantNumber;
export default Number;