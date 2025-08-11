import type { DataToString } from "./data";

export class GraphExecutor
{
    id : string = "";
    path : string = "";

    input : Array<DataToString> = [];

    output : Array<GraphNodeOutput> = [];
}

export type GraphNodeOutput = GraphNodeOutputOut | GraphNodeOutputErr | GraphNodeOutputSuccess | GraphNodeOutputObject;

export enum GraphNodeOutputVariant
{
    Success = "Success",
    Out = "Out",
    Err = "Err",
    Object = "Object"
}

export class GraphNodeOutputOut
{
    readonly _variant = GraphNodeOutputVariant.Out;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class GraphNodeOutputErr
{
    readonly _variant = GraphNodeOutputVariant.Err;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class GraphNodeOutputSuccess
{
    readonly _variant = GraphNodeOutputVariant.Success;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class GraphNodeOutputObject
{
    readonly _variant = GraphNodeOutputVariant.Object;

    prefix : string | undefined;
    suffix : string | undefined;
}