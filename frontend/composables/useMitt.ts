import mitt from "mitt";

const e = mitt();

export const useMitt = () => {
    return e;
}