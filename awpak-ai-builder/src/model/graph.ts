import type { DataType } from "./data";
import { type NodeType } from "./node";

export class Graph
{
    context : Map<string, any> | undefined;
    preserve_context : boolean | undefined;

    input_type : DataType | undefined;

    first : NodeType | undefined;

    nodes : Array<NodeType> = [];
}

