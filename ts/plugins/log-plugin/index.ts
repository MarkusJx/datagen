import {GenerateFn, InitFn, TransformFn} from "../../base/plugin";

let i = 0;

export const init: InitFn<{}> = async (_args) => {
    //console.log("init", args);
    console.time('1000');
}

export const transform: TransformFn<{}> = async (value, _args) => {
    i++;
    if (i % 1000 === 0) {
        console.timeEnd(i.toString());
        console.time((i + 1000).toString());
    }

    return value;
}

export const generate: GenerateFn<{}, string> = async (_args) => {
    //console.log("generate", args);
    return 'test';
}
