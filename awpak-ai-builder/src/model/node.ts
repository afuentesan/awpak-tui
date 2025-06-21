import type { DataToContext, DataToString } from "./data";
import { DataComparatorEq, type DataComparator } from "./data_comparator";
import type { NodeExecutor } from "./node_executor";

export type NodeType = Node | GraphNode;

export enum NodeTypeVariant
{
    Node = "Node",
    GraphNode = "GraphNode"
}

export class Node
{
    readonly _variant = NodeTypeVariant.Node;

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
    readonly _variant = NodeTypeVariant.GraphNode;

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

export enum GraphNodeOutputVariant
{
    Out = "Out",
    Err = "Err"
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

export class NodeDestination
{
    next : NodeNext | undefined;

    condition : DataComparator | undefined;

    constructor()
    {
        this.next = new NodeNextNode();
        this.condition = new DataComparatorEq();
    }
}

export type NodeNext = NodeNextNode | NodeNextExitOk | NodeNextExitErr;

export enum NodeNextVariant
{
    Node = "Node",
    ExitOk = "ExitOk",
    ExitErr = "ExitErr"
}

export class NodeNextNode
{
    readonly _variant = NodeNextVariant.Node;

    value : string | undefined;
}

export class NodeNextExitOk
{
    readonly _variant = NodeNextVariant.ExitOk;

    value : Array<DataToString> = [];
}

export class NodeNextExitErr
{
    readonly _variant = NodeNextVariant.ExitErr;

    value : Array<DataToString> = [];
}