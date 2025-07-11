import { AIAgent } from "./agent";
import { Command } from "./command";
import { DataToContext, type DataFrom } from "./data";

export type NodeExecutor = NodeExecutorCommand | NodeExecutorContextMut | NodeExecutorAgent;

export enum NodeExecutorVariant
{
    Command = "Command",
    ContextMut = "ContextMut",
    Agent = "Agent"
}

export class NodeExecutorAgent
{
    readonly _variant = NodeExecutorVariant.Agent;

    value : AIAgent;

    constructor()
    {
        this.value = new AIAgent();
    }
}

export class NodeExecutorCommand
{
    readonly _variant = NodeExecutorVariant.Command;

    value : Command;

    constructor()
    {
        this.value = new Command();
    }
}

export class NodeExecutorContextMut
{
    readonly _variant = NodeExecutorVariant.ContextMut;

    value : Array<{
        from : DataFrom | undefined;
        to : DataToContext | undefined
    }> = []
}