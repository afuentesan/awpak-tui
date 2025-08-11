import { AIAgent } from "./agent";
import type { AgentHistoryMut } from "./agent_history_mut";
import { Command } from "./command";
import type { ContextMut } from "./context_mut";
import { GraphExecutor } from "./graph_executor";
import { Parallel } from "./parallel";
import { WebClient } from "./web_client";

export type NodeExecutor = NodeExecutorCommand | 
                           NodeExecutorContextMut | 
                           NodeExecutorAgent | 
                           NodeExecutorWebClient |
                           NodeExecutorAgentHistoryMut |
                           NodeExecutorParallel |
                           NodeExecutorGraph;

export enum NodeExecutorVariant
{
    Command = "Command",
    ContextMut = "ContextMut",
    Agent = "Agent",
    WebClient = "WebClient",
    AgentHistoryMut = "AgentHistoryMut",
    Parallel = "Parallel",
    Graph = "Graph"
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

export class NodeExecutorAgentHistoryMut
{
    readonly _variant = NodeExecutorVariant.AgentHistoryMut;

    value : Array<AgentHistoryMut> = []
}

export class NodeExecutorParallel
{
    readonly _variant = NodeExecutorVariant.Parallel;

    value : Parallel = new Parallel();
}

export class NodeExecutorGraph
{
    readonly _variant = NodeExecutorVariant.Graph;

    value : GraphExecutor = new GraphExecutor();
}