import type { DataFrom, DataToContext } from "./data";

export type NodeExecutor = NodeExecutorCommand | NodeExecutorGraph | NodeExecutorContextMut;

export class NodeExecutorCommand
{
    _variant = "Command";
}

export class NodeExecutorGraph
{
    _variant = "Graph";
}

export class NodeExecutorContextMut
{
    _variant = "ContextMut";

    from : DataFrom | undefined;
    to : DataToContext | undefined;
}