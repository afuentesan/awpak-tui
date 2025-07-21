import { AIAgent, AIAgentProviderAnthropic, AIAgentProviderConfigVariant, AIAgentProviderDeepSeek, AIAgentProviderGemini, AIAgentProviderOllama, AIAgentProviderOpenAI, type AIAgentProvider } from "../model/agent";
import type { AgentHistoryMut } from "../model/agent_history_mut";
import { Command, CommandOutputCode, CommandOutputErr, CommandOutputObject, CommandOutputOut, CommandOutputSuccess, CommandOutputVariant, type CommandOutput } from "../model/command";
import type { ContextMut } from "../model/context_mut";
import { DataFromVariant, DataOperationVariant, type DataFrom } from "../model/data";
import { DataComparatorVariant, type DataComparator } from "../model/data_comparator";
import type { Graph } from "../model/graph";
import { GraphNode, GraphNodeOutputErr, GraphNodeOutputObject, GraphNodeOutputOut, GraphNodeOutputSuccess, GraphNodeOutputVariant, Node, NodeDestination, NodeNextExitErr, NodeNextExitOk, NodeNextNode, NodeNextVariant, NodeTypeVariant, type GraphNodeOutput, type NodeNext, type NodeType } from "../model/node";
import { NodeExecutorAgent, NodeExecutorAgentHistoryMut, NodeExecutorCommand, NodeExecutorContextMut, NodeExecutorParallel, NodeExecutorVariant, NodeExecutorWebClient, type NodeExecutor } from "../model/node_executor";
import { ParallelExecutorCommand, ParallelExecutorVariant, ParallelExecutorWebClient, type Parallel, type ParallelExecutor } from "../model/parallel";
import { WebClient, WebClientBodyVariant, WebClientOutputBody, WebClientOutputHeader, WebClientOutputObject, WebClientOutputStatus, WebClientOutputVariant, WebClientOutputVersion, type WebClientOutput } from "../model/web_client";
import { is_type_in_enum } from "./form_utils";

export function clean_graph_node_ids( graph : Graph, id : string )
{
    update_graph_node_ids( graph, id, undefined );
}

export function update_graph_node_ids( graph : Graph, id : string, new_id : string | undefined )
{
    update_ids_in_node( graph.first, id, new_id );

    graph.nodes.forEach( ( n ) => update_ids_in_node( n, id, new_id ) );
}

function update_ids_in_node( node : NodeType | undefined, id : string, new_id : string | undefined )
{
    if( ! node ) return;

    if( node._variant == NodeTypeVariant.Node )
    {
        if( node.executor?._variant == NodeExecutorVariant.AgentHistoryMut )
        {
            update_agent_history_ids_in_agent_historys_mut( node.executor.value, id, new_id );
        }
        else if( node.executor?._variant == NodeExecutorVariant.ContextMut )
        {
            update_ids_in_contexts_mut( node.executor.value, id, new_id );
        }
        else if( node.executor?._variant == NodeExecutorVariant.Command )
        {
            update_ids_in_command( node.executor.value, id, new_id );
        }
        else if( node.executor?._variant == NodeExecutorVariant.Agent )
        {
            update_ids_in_ai_agent( node.executor.value, id, new_id );
        }
        else if( node.executor?._variant == NodeExecutorVariant.WebClient )
        {
            update_ids_in_web_client( node.executor.value, id, new_id );
        }
        else if( node.executor?._variant == NodeExecutorVariant.Parallel )
        {
            update_ids_in_parallel( node.executor.value, id, new_id );
        }
    }
    else if( node._variant == NodeTypeVariant.GraphNode )
    {
        node.input.forEach( ( i ) => update_ids_in_data_from( i.from, id, new_id ) );
    }

    update_node_destinations_id( node, id, new_id );
}

function update_ids_in_parallel( parallel : Parallel, id : string, new_id : string | undefined )
{
    parallel.executors.forEach(
        ( p ) => {
            if( p._variant == ParallelExecutorVariant.Command )
            {
                update_ids_in_command( p.executor, id, new_id );

                update_ids_in_data_comparator( p.condition, id, new_id );
            }
            else if( p._variant == ParallelExecutorVariant.WebClient )
            {
                update_ids_in_web_client( p.executor, id, new_id );

                update_ids_in_data_comparator( p.condition, id, new_id );
            }
        }
    )
}

function update_ids_in_web_client( client : WebClient, id : string, new_id : string | undefined )
{
    update_ids_in_data_from( client.url, id, new_id );

    client.headers.forEach( ( h ) => {
        update_ids_in_data_from( h.name, id, new_id );
        update_ids_in_data_from( h.value, id, new_id );
    } );

    client.query_params.forEach( ( q ) => {
        update_ids_in_data_from( q.name, id, new_id );
        update_ids_in_data_from( q.value, id, new_id );
    } );

    if( client.body?._variant == WebClientBodyVariant.Json )
    {
        update_ids_in_data_from( client.body.value, id, new_id );
    }
    else if( client.body?._variant == WebClientBodyVariant.Form )
    {
        client.body.value.forEach( ( d ) => {
            update_ids_in_data_from( d.name, id, new_id );
            update_ids_in_data_from( d.value, id, new_id );
        } );
    }
}

function update_ids_in_ai_agent( agent : AIAgent, id : string, new_id : string | undefined )
{
    agent.system_prompt.forEach( ( p ) => update_ids_in_data_from( p.from, id, new_id ) );

    agent.prompt.forEach( ( p ) => update_ids_in_data_from( p.from, id, new_id ) );

    agent.servers.forEach( ( s ) => s.args.forEach( ( a ) => update_ids_in_data_from( a, id, new_id ) ) );

    update_ids_in_data_from( agent.provider.model, id, new_id );
}

function update_ids_in_command( command : Command, id : string, new_id : string | undefined )
{
    update_ids_in_data_from( command.command, id, new_id );

    command.args.forEach( ( a ) => update_ids_in_data_from( a, id, new_id ) );
}

function update_ids_in_contexts_mut( contexts : Array<ContextMut>, id : string, new_id : string | undefined )
{
    contexts.forEach( ( c ) => update_ids_in_context_mut( c, id, new_id ) );
}

function update_ids_in_context_mut( context : ContextMut, id : string, new_id : string | undefined )
{
    update_ids_in_data_from( context.from, id, new_id );
    update_ids_in_data_comparator( context.condition, id, new_id );
}

function update_agent_history_ids_in_agent_historys_mut( agent_historys : Array<AgentHistoryMut>, id : string, new_id : string | undefined )
{
    agent_historys.forEach( ( h ) => update_agent_history_ids_in_agent_history_mut( h, id, new_id ) );
}

function update_agent_history_ids_in_agent_history_mut( agent_history : AgentHistoryMut, id : string, new_id : string | undefined )
{
    update_ids_in_data_from( agent_history.from, id, new_id );

    update_ids_in_data_comparator( agent_history.condition, id, new_id );

    if( agent_history.id == id )
    {
        agent_history.id = new_id ? new_id : "";
    }
}

function update_node_destinations_id( node : NodeType | undefined, id : string, new_id : string | undefined )
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
            else
            {
                d.next?.value.forEach(
                    ( d ) => update_ids_in_data_from( d.from, id, new_id )
                )
            }

            update_ids_in_data_comparator( d.condition, id, new_id );
        } 
    );
}

function update_ids_in_data_comparator( data_comparator : DataComparator | undefined, id : string, new_id : string | undefined )
{
    if( ! data_comparator ) return;

    if( 
        data_comparator._variant == DataComparatorVariant.Eq ||
        data_comparator._variant == DataComparatorVariant.NotEq ||
        data_comparator._variant == DataComparatorVariant.Gt ||
        data_comparator._variant == DataComparatorVariant.Lt
    )
    {
        update_ids_in_data_from( data_comparator.from_1, id, new_id );
        update_ids_in_data_from( data_comparator.from_2, id, new_id );
    }
    else if(
        data_comparator._variant == DataComparatorVariant.Regex
    )
    {
        update_ids_in_data_from( data_comparator.from, id, new_id );
    }
    else if(
        data_comparator._variant == DataComparatorVariant.Not
    )
    {
        update_ids_in_data_comparator( data_comparator.value, id, new_id );
    }
    else if(
        data_comparator._variant == DataComparatorVariant.And ||
        data_comparator._variant == DataComparatorVariant.Or
    )
    {
        update_ids_in_data_comparator( data_comparator.comp_1, id, new_id );
        update_ids_in_data_comparator( data_comparator.comp_2, id, new_id );
    }
}

function update_ids_in_data_from( data_from : DataFrom | undefined, id : string, new_id : string | undefined )
{
    if( ! data_from ) return;

    if( data_from._variant == DataFromVariant.AgentHistory )
    {
        if( data_from.id == id )
        {
            data_from.id = new_id ? new_id : "";
        }
    }
    else if( data_from._variant == DataFromVariant.Concat )
    {
        data_from.value.forEach( ( d ) => update_ids_in_data_from( d, id, new_id ) ); 
    }
    else if( data_from._variant == DataFromVariant.Operation )
    {
        if( data_from.value?._variant == DataOperationVariant.Len )
        {
            update_ids_in_data_from( data_from.value.value, id, new_id );
        }
        else if(
            data_from.value?._variant == DataOperationVariant.Add ||
            data_from.value?._variant == DataOperationVariant.Substract
        )
        {
            update_ids_in_data_from( data_from.value.num_1, id, new_id );
            update_ids_in_data_from( data_from.value.num_2, id, new_id );
        }
    }
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

export function new_parallel_executor_variant( old : ParallelExecutor, new_variant : string ) : ParallelExecutor | undefined
{
    if( ! is_type_in_enum( ParallelExecutorVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as ParallelExecutorVariant;

    if( old._variant == new_variant ) { return old; }

    if( new_variant == ParallelExecutorVariant.Command )
    {
        let ret = new ParallelExecutorCommand();

        ret.ty = old.ty;

        return ret;
    }
    else if( new_variant == ParallelExecutorVariant.WebClient )
    {
        let ret = new ParallelExecutorWebClient();

        ret.ty = old.ty;

        return ret;
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
    else if( new_variant == NodeExecutorVariant.AgentHistoryMut )
    {
        return new NodeExecutorAgentHistoryMut();
    }
    else if( new_variant == NodeExecutorVariant.WebClient )
    {
        return new NodeExecutorWebClient();
    }
    else if( new_variant == NodeExecutorVariant.Parallel )
    {
        return new NodeExecutorParallel();
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