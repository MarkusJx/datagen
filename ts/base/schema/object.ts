import Base from "./base";
import Any from "./any";

export default interface ObjectSchema extends Base {
    type: 'object';
    properties: {
        [key: string]: Any;
    }
}
