import Base from "./base";

export default interface Reference extends Base {
    type: 'reference';
    ref: string;
    except?: (string | number)[];
}
