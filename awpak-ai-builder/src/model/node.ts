import type { DataToContext, DataToString } from "./data";
import type { DataComparator } from "./data_comparator";
import type { NodeExecutor } from "./node_executor";

export type NodeType = Node | GraphNode;

export class Node
{
    readonly _variant = "Node";

    id : string | undefined;

    executor : NodeExecutor | undefined = undefined;

    output : DataToContext | undefined = undefined;
    destination : Array<NodeDestination> = [];

    constructor( id : string )
    {
        this.id = id;
    }
}

export class GraphNode
{
    readonly _variant = "GraphNode";

    id : string | undefined;
    path : string | undefined;

    input : Array<DataToString> = [];

    output : Array<GraphNodeOutput> = [];

    node_output : DataToContext | undefined;

    node_destination : Array<NodeDestination> = [];

    constructor( id : string )
    {
        this.id = id;
    }
}

export type GraphNodeOutput = GraphNodeOutputOut | GraphNodeOutputErr;

export class GraphNodeOutputOut
{
    _variant = "Out";

    prefix : string | undefined;
    suffix : string | undefined;
}

export class GraphNodeOutputErr
{
    _variant = "Err";

    prefix : string | undefined;
    suffix : string | undefined;
}

export class NodeDestination
{
    next : NodeNext | undefined;

    condition : DataComparator | undefined;
}

export type NodeNext = NodeNextNode | NodeNextExitOk | NodeNextExitErr;

export class NodeNextNode
{
    _variant = "Node";

    value : string | undefined;
}

export class NodeNextExitOk
{
    _variant = "ExitOk";

    value : Array<DataToString> = [];
}

export class NodeNextExitErr
{
    _variant = "ExitErr";

    value : Array<DataToString> = [];
}