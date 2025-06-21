import type { DataFrom, DataToContext } from "./data";

export type NodeExecutor = NodeExecutorCommand | NodeExecutorGraph | NodeExecutorContextMut;

export enum NodeExecutorVariant
{
    Command = "Command",
    Graph = "Graph",
    ContextMut = "ContextMut"
}

export class NodeExecutorCommand
{
    readonly _variant = NodeExecutorVariant.Command;
}

export class NodeExecutorGraph
{
    readonly _variant = NodeExecutorVariant.Graph;
}

export class NodeExecutorContextMut
{
    readonly _variant = NodeExecutorVariant.ContextMut;

    from : DataFrom | undefined;
    to : DataToContext | undefined;
}