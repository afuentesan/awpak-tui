import { DataToContext, type DataToString } from "./data";
import { DataComparatorTrue, type DataComparator } from "./data_comparator";
import { NodeExecutorCommand, type NodeExecutor } from "./node_executor";

export class NodeConfig
{
    id : string | undefined;

    executor : NodeExecutor = new NodeExecutorCommand();

    output : DataToContext = new DataToContext();
    destination : Array<NodeDestination> = [];

    constructor( id : string )
    {
        this.id = id;
    }
}

export class NodeDestination
{
    next : NodeNext | undefined;

    condition : DataComparator | undefined;

    constructor()
    {
        this.next = new NodeNextNode();
        this.condition = new DataComparatorTrue();
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