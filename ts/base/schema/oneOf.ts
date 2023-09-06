import Base from "./base";
import Any from "./any";

export default interface OneOf extends Base {
    type: 'oneOf';
    elements: Any[];
}
