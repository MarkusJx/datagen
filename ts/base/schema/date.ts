import Base from "./base";

export interface ConstantDate extends Base {
    type: 'date';
    value: string;
}

export interface RandomDate extends Base {
    type: 'date';
    min?: string;
    max?: string;
}

type GeneratedDate = ConstantDate | RandomDate;
export default GeneratedDate;
