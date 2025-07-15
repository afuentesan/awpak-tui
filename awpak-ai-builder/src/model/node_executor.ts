import { AIAgent } from "./agent";
import { Command } from "./command";
import type { ContextMut } from "./context_mut";
import { DataToContext, type DataFrom } from "./data";
import { WebClient } from "./web_client";

export type NodeExecutor = NodeExecutorCommand | NodeExecutorContextMut | NodeExecutorAgent | NodeExecutorWebClient;

export enum NodeExecutorVariant
{
    Command = "Command",
    ContextMut = "ContextMut",
    Agent = "Agent",
    WebClient = "WebClient"
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

    value : Array<ContextMut> = []
}

export class NodeExecutorWebClient
{
    readonly _variant = NodeExecutorVariant.WebClient;

    value : WebClient;

    constructor()
    {
        this.value = new WebClient();
    }
}