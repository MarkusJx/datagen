import String from "./string";
import Number from "./number";
import Boolean from "./boolean";
import Array from "./array";
import Date from "./date";
import ObjectSchema from "./object";
import {PluginSchema} from "./plugin";
import OneOf from "./oneOf";
import Reference from "./reference";

type Primitive = string | number | boolean | null;
type Any = String | Number | Boolean | ObjectSchema | Array | Date | PluginSchema | Primitive | OneOf | Reference;
export default Any;