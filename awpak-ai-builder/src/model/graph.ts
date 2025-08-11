import type { DataType } from "./data";
import { NodeConfig } from "./node";
import type { StoreConfig } from "./store";

export class Graph
{
    stores : Array<StoreConfig> = [];

    context : Map<string, any> | undefined;
    preserve_context : boolean | undefined;

    input_type : DataType | undefined;

    first : NodeConfig | undefined;

    nodes : Array<NodeConfig> = [];
}

