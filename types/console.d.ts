
declare module "node:console" {
    export function trace(message:string):void;
    export function debug(message:string):void;
    export function log(message:string):void;
    export function warn(message:string):void;
    export function error(message:string):void;
}
