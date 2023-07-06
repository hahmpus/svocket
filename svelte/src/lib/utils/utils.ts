//TIMERS
export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

var staggeredFunctions:any = {};
export function stagger(callback:()=>void, key:string, time:number=300) {
    if (staggeredFunctions[key]) {
        clearTimeout(staggeredFunctions[key]);
        staggeredFunctions[key] = null;
    }
    staggeredFunctions[key] = setTimeout(() => {
        staggeredFunctions[key] = null;
        callback();
    }, time);
}

var frozenFunctions:any = {};
export function freeze(callback:()=>void, key:string, time:number=1000) {
    if (frozenFunctions[key]) {
        return;
    }
    frozenFunctions[key] = setTimeout(() => {
        frozenFunctions[key] = null;
    }, time);
    callback();
}
