import Base from "./base";
import Any from "./any";

export interface ConstantArrayLength {
    /**
     * The constant length of the array.
     *
     * @minimum 0
     * @TJS-type integer
     */
    value: number;
}

export interface RandomArrayLength {
    /**
     * The minimum length of the array.
     *
     * @minimum 0
     * @TJS-type integer
     */
    min?: number;
    /**
     * The maximum length of the array.
     *
     * @minimum 0
     * @TJS-type integer
     */
    max?: number;
    /**
     * The step between the minimum and maximum length of the array.
     *
     * @minimum 1
     * @TJS-type integer
     */
    step?: number;
}

/**
 * An array schema.
 */
export default interface Array extends Base {
    type: 'array';
    length: ConstantArrayLength | RandomArrayLength | number;
    items: Any;
}
