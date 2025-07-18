import { AIAgentProviderAnthropic, AIAgentProviderConfigVariant, AIAgentProviderDeepSeek, AIAgentProviderGemini, AIAgentProviderOllama, AIAgentProviderOpenAI, type AIAgentProvider } from "../model/agent";
import { CommandOutputCode, CommandOutputErr, CommandOutputObject, CommandOutputOut, CommandOutputSuccess, CommandOutputVariant, type CommandOutput } from "../model/command";
import type { Graph } from "../model/graph";
import { GraphNode, GraphNodeOutputErr, GraphNodeOutputObject, GraphNodeOutputOut, GraphNodeOutputSuccess, GraphNodeOutputVariant, Node, NodeDestination, NodeNextExitErr, NodeNextExitOk, NodeNextNode, NodeNextVariant, NodeTypeVariant, type GraphNodeOutput, type NodeNext, type NodeType } from "../model/node";
import { NodeExecutorAgent, NodeExecutorCommand, NodeExecutorContextMut, NodeExecutorVariant, NodeExecutorWebClient, type NodeExecutor } from "../model/node_executor";
import { WebClientOutputBody, WebClientOutputHeader, WebClientOutputObject, WebClientOutputStatus, WebClientOutputVariant, WebClientOutputVersion, type WebClientOutput } from "../model/web_client";
import { is_type_in_enum } from "./form_utils";

export function clean_graph_destinations_id( graph : Graph, id : string )
{
    clean_node_destinations_id( graph.first, id );

    graph.nodes.forEach( ( n ) => clean_node_destinations_id( n, id ) );
}

function clean_node_destinations_id( node : NodeType | undefined, id : string )
{
    if( ! node ) return;

    let destinations = ( node._variant == NodeTypeVariant.Node ) ? node.destination : node.node_destination;

    destinations.forEach( 
        ( d : NodeDestination ) => 
        {
            if( d.next?._variant == NodeNextVariant.Node )
            {
                if( d.next.value == id )
                {
                    d.next.value = undefined;
                }
            }
        } 
    );
}

export function update_graph_destinations_id( graph : Graph, id : string, new_id : string )
{
    update_node_destinations_id( graph.first, id, new_id );

    graph.nodes.forEach( ( n ) => update_node_destinations_id( n, id, new_id ) );
}

function update_node_destinations_id( node : NodeType | undefined, id : string, new_id : string )
{
    if( ! node ) return;

    let destinations = ( node._variant == NodeTypeVariant.Node ) ? node.destination : node.node_destination;

    destinations.forEach( 
        ( d : NodeDestination ) => 
        {
            if( d.next?._variant == NodeNextVariant.Node )
            {
                if( d.next.value == id )
                {
                    d.next.value = new_id;
                }
            }
        } 
    );
}

export function node_variants() : Array<string>
{
    return [
        "Node",
        "GraphNode"
    ]
}

export function node_next_variants() : Array<string>
{
    return [
        "Node",
        "ExitOk",
        "ExitErr"
    ]
}

export function node_ids( graph : Graph ) : Array<string>
{
    let ids : Array<string> = [ graph.first?.id as string ];

    graph.nodes.forEach( ( v : NodeType ) => ids.push( v.id as string ) );

    return ids;
}

export function agent_ids( graph : Graph ) : Array<string>
{
    let ids : Array<string> = [];

    if( node_is_agent( graph.first ) && graph.first?.id ) ids.push( graph.first.id );

    graph.nodes.forEach(
        ( n ) => {
            if( node_is_agent( n ) && n.id ) ids.push( n.id );
        }
    );

    return ids;
}

function node_is_agent( node : NodeType | undefined ) : boolean
{
    if( ! node || node._variant != NodeTypeVariant.Node ) return false;

    if( node.executor?._variant == NodeExecutorVariant.Agent ) return true;

    return false;
}

export function next_node_id( graph : Graph ) : string
{
    let ids = node_ids( graph );

    let i = 1 + graph.nodes.length;

    let id = "node " + i;

    while( ids.includes( id ) )
    {
        i++;

        id = "node " + i;
    }

    return id;
}

export function node_by_id( graph : Graph, id : string ) : NodeType | undefined
{
    if( ! id?.trim() ) { return undefined };

    if( graph.first?.id == id )
    {
        return graph.first;
    }

    return graph.nodes.find( ( n : NodeType ) => n.id == id );
}

export function node_and_base_path_from_id( 
    graph : Graph, 
    id : string 
) : { node : NodeType, base_path : string, idx : number | undefined, is_first : boolean } | undefined
{
    if( ! id?.trim() ) return undefined;

    if( graph.first?.id == id )
    {
        return {
            node : graph.first,
            base_path : "$.first",
            idx : undefined,
            is_first : true
        };
    }

    for( let i = 0 ; i < graph.nodes.length ; i++ )
    {
        if( graph.nodes[ i ].id == id )
        {
            return {
                node : graph.nodes[ i ],
                base_path : "$.nodes["+i+"]",
                idx : i,
                is_first : false
            }
        }
    }
}

export function new_agent_provider_variant( old : AIAgentProvider, new_variant : string ) : AIAgentProvider | undefined
{
    if( ! is_type_in_enum( AIAgentProviderConfigVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as AIAgentProviderConfigVariant;

    if( old._variant == new_variant ) { return old; }

    let model = old.model;

    let api_key = ( old._variant != AIAgentProviderConfigVariant.Ollama ) ? old.api_key : undefined;

    let max_tokens = (
        old._variant == AIAgentProviderConfigVariant.Anthropic ||
        old._variant == AIAgentProviderConfigVariant.DeepSeek
    )
    ?
    old.max_tokens
    :
    undefined;

    if( new_variant == AIAgentProviderConfigVariant.Anthropic || new_variant == AIAgentProviderConfigVariant.DeepSeek )
    {
        let provider = ( new_variant == AIAgentProviderConfigVariant.Anthropic ) ? new AIAgentProviderAnthropic() : new AIAgentProviderDeepSeek();

        provider.model = model;
        provider.api_key = api_key;
        provider.max_tokens = max_tokens;

        return provider;
    }
    else if( new_variant == AIAgentProviderConfigVariant.OpenAI || new_variant == AIAgentProviderConfigVariant.Gemini )
    {
        let provider = ( new_variant == AIAgentProviderConfigVariant.OpenAI ) ? new AIAgentProviderOpenAI() : new AIAgentProviderGemini();

        provider.model = model;
        provider.api_key = api_key;

        return provider;
    }
    else if( new_variant == AIAgentProviderConfigVariant.Ollama )
    {
        let provider = new AIAgentProviderOllama();

        provider.model = model;

        return provider;
    }
}

export function new_node_executor_variant( old : NodeExecutor, new_variant : string ) : NodeExecutor | undefined
{
    if( ! is_type_in_enum( NodeExecutorVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as NodeExecutorVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == NodeExecutorVariant.Command )
    {
        return new NodeExecutorCommand();
    }
    else if( new_variant == NodeExecutorVariant.ContextMut )
    {
        return new NodeExecutorContextMut();
    }
    else if( new_variant == NodeExecutorVariant.Agent )
    {
        return new NodeExecutorAgent();
    }
    else if( new_variant == NodeExecutorVariant.WebClient )
    {
        return new NodeExecutorWebClient();
    }
}

export function new_graph_node_output_variant( old : GraphNodeOutput, new_variant : string ) : GraphNodeOutput | undefined
{
    if( ! is_type_in_enum( GraphNodeOutputVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as GraphNodeOutputVariant;

    if( old._variant == new_variant ) { return old; }

    let new_output;

    if( new_variant == GraphNodeOutputVariant.Err  )
    {
        new_output = new GraphNodeOutputErr();
    }
    else if( new_variant == GraphNodeOutputVariant.Out  )
    {
        new_output = new GraphNodeOutputOut();
    }
    else if( new_variant == GraphNodeOutputVariant.Success  )
    {
        new_output = new GraphNodeOutputSuccess();
    }
    else if( new_variant == GraphNodeOutputVariant.Object  )
    {
        new_output = new GraphNodeOutputObject();
    }

    if( ! new_output ) { return undefined; }

    new_output.prefix = old.prefix;
    new_output.suffix = old.suffix;
    
    return new_output;
}

export function new_web_client_output_variant( old : WebClientOutput, new_variant : string ) : WebClientOutput | undefined
{
    if( ! is_type_in_enum( WebClientOutputVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as WebClientOutputVariant;

    if( old._variant == new_variant ) { return old; }

    let new_output;

    if( new_variant == WebClientOutputVariant.Version )
    {
        new_output = new WebClientOutputVersion();
    }
    else if( new_variant == WebClientOutputVariant.Status )
    {
        new_output = new WebClientOutputStatus();
    }
    else if( new_variant == WebClientOutputVariant.Header )
    {
        new_output = new WebClientOutputHeader();
    }
    else if( new_variant == WebClientOutputVariant.Body )
    {
        new_output = new WebClientOutputBody();
    }
    else if( new_variant == WebClientOutputVariant.Object )
    {
        new_output = new WebClientOutputObject();
    }

    if( ! new_output ) { return undefined; }

    new_output.prefix = old.prefix;
    new_output.suffix = old.suffix;
    
    return new_output;
}

export function new_command_node_output_variant( old : CommandOutput, new_variant : string ) : CommandOutput | undefined
{
    if( ! is_type_in_enum( CommandOutputVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as CommandOutputVariant;

    if( old._variant == new_variant ) { return old; }

    let new_output;

    if( new_variant == CommandOutputVariant.Out )
    {
        new_output = new CommandOutputOut();
    }
    else if( new_variant == CommandOutputVariant.Err )
    {
        new_output = new CommandOutputErr();
    }
    else if( new_variant == CommandOutputVariant.Success )
    {
        new_output = new CommandOutputSuccess();
    }
    else if( new_variant == CommandOutputVariant.Code )
    {
        new_output = new CommandOutputCode();
    }
    else if( new_variant == CommandOutputVariant.Object )
    {
        new_output = new CommandOutputObject();
    }

    if( ! new_output ) { return undefined; }

    new_output.prefix = old.prefix;
    new_output.suffix = old.suffix;
    
    return new_output;
}

export function is_node_next_variant( node_next : any ) : boolean
{
    if( ! node_next._variant?.trim() ) { return false; }

    if( ! node_next_variants().includes( node_next._variant ) ) { return false; }

    return true;
}

export function change_node_next_variant( node_next : NodeNext, new_variant : string ) : NodeNext | undefined
{
    if( ! is_node_next_variant( node_next ) ) { return undefined; }

    if( ! new_variant?.trim() || new_variant == node_next._variant ) { return undefined; }

    if( node_next._variant == NodeNextVariant.Node )
    {
        return ( new_variant == NodeNextVariant.ExitOk ) ? new NodeNextExitOk() : new NodeNextExitErr();
    }

    if( new_variant == NodeNextVariant.Node )
    {
        return new NodeNextNode();
    }
    else
    {
        let value = node_next.value;

        let new_node_next = ( new_variant == NodeNextVariant.ExitOk ) ? new NodeNextExitOk() : new NodeNextExitErr();

        new_node_next.value = ( ! value ? [] : value );

        return new_node_next;
    }
}

export function is_node_variant( node : any ) : boolean
{
    if( ! node._variant?.trim() ) { return false; }

    if( ! node_variants().includes( node._variant ) ) { return false; }

    return true;
}

export function change_node_variant( node : NodeType, new_variant : string ) : NodeType | undefined
{
    if( ! is_node_variant( node ) ) { return undefined; }

    if( ! new_variant?.trim() || new_variant == node._variant ) { return undefined; }

    if( new_variant == "Node" )
    {
        return node_variant( node, new_variant );
    }
    else if( new_variant == "GraphNode" )
    {
        return graph_node_variant( node, new_variant );
    }
}

function node_variant( from : NodeType, new_variant : string ) : NodeType
{
    let node = new Node( from.id as string );

    if( new_variant == "GraphNode" )
    {
        node.output = ( from as GraphNode ).node_output;
        node.destination = ( from as GraphNode ).node_destination;
    }

    return node;
}

function graph_node_variant( from : NodeType, new_variant : string ) : NodeType
{
    let node = new GraphNode( from.id as string );

    if( new_variant == "Node" )
    {
        node.node_output = ( from as Node ).output;
        node.node_destination = ( from as Node ).destination;
    }

    return node;
}