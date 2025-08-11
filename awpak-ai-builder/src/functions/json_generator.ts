import { AIAgentProviderConfigVariant, type AIAgentProvider, type NodeMCPServer } from "../model/agent";
import { DataToAgentHistoryVariant, type AgentHistoryMut, type DataToAgentHistory } from "../model/agent_history_mut";
import { Command, CommandOutputVariant, type CommandOutput } from "../model/command";
import type { ContextMut } from "../model/context_mut";
import { DataFromVariant, DataOperationVariant, FromAgentHistoryContentVariant, type DataFrom, type DataOperation, type DataToContext, type DataToString, type FromAgentHistoryContent } from "../model/data";
import { DataComparatorVariant, type DataComparator } from "../model/data_comparator";
import type { Graph } from "../model/graph";
import { GraphNodeOutputVariant, type GraphExecutor, type GraphNodeOutput } from "../model/graph_executor";
import { NodeConfig, NodeDestination, NodeNextVariant, type NodeNext } from "../model/node";
import { NodeExecutorAgent, NodeExecutorAgentHistoryMut, NodeExecutorCommand, NodeExecutorContextMut, NodeExecutorParallel, NodeExecutorVariant, NodeExecutorWebClient, type NodeExecutor } from "../model/node_executor";
import { ParallelExecutorVariant, type ParallelExecutor } from "../model/parallel";
import { StoreDocumentSizerVariant, StoreModelVariant, type StoreConfig, type StoreDocument, type StoreDocumentSizer, type StoreModel } from "../model/store";
import { WebClient, WebClientBodyVariant, WebClientOutputVariant, type WebClientBody, type WebClientNameValue, type WebClientOutput } from "../model/web_client";
import { is_empty } from "./data_functions";

export function generate_json( graph : Graph ) : string
{
    let json : any = {};

    append_stores( json, graph.stores );

    append_initial_context( json, graph );
    append_input_data_type( json, graph );

    json[ "preserve_context" ] = graph.preserve_context || false;

    json[ "first" ] = json_node( graph.first );

    append_nodes( json, graph );

    return JSON.stringify( json, null, 2 );
}

function append_stores( json : any, stores : Array<StoreConfig> )
{
    json[ "stores" ] = [];

    stores.forEach(
        ( s : StoreConfig ) =>
        {
            let new_store = json_store( s );

            if( new_store ) json[ "stores" ].push( new_store );
        }
    )
}

function json_store( store : StoreConfig ) : any
{
    return {
        id : store.id,
        documents : json_store_documents( store.documents ),
        provider : store.provider,
        model : json_store_model( store.model )
    }
}

function json_store_model( model : StoreModel ) : any
{
    return {
            [model._variant] : {
                model : model.model,
                api_key : ( model._variant == StoreModelVariant.Gemini || model._variant == StoreModelVariant.OpenAI ) ? model.api_key : undefined
            }
    };
}

function json_store_documents( documents : Array<StoreDocument> ) : Array<any>
{
    return documents.map( ( d ) => json_store_document( d ) );
}

function json_store_document( document : StoreDocument ) : any
{
    return {
            [document._variant] : {
                path : document.path,
                sizer : json_store_document_sizer( document.sizer )
            }
    };
}

function json_store_document_sizer( sizer : StoreDocumentSizer ) : any
{
    if( sizer._variant == StoreDocumentSizerVariant.None )
    {
        return "None";
    }
    else if( sizer._variant == StoreDocumentSizerVariant.Chars )
    {
        return {
            "Chars" : {
                desired : sizer.desired,
                max : sizer.max
            }
        }
    }
    else if( sizer._variant == StoreDocumentSizerVariant.Markdown )
    {
        return {
            "Markdown" : {
                desired : sizer.desired,
                max : sizer.max
            }
        }
    }

    throw new Error( "StoreDocumentSizerVariant not found. ", sizer );
}

function append_nodes( json : any, graph : Graph )
{
    json[ "nodes" ] = [];

    graph.nodes.forEach( 
        ( n : NodeConfig ) => 
        {
            let new_node = json_node( n );

            if( new_node ) { json[ "nodes" ].push( new_node ); }
        }
    );
}

function append_input_data_type( json : any, graph : Graph )
{
    if( graph.input_type )
    {
        json[ "input_type" ] = graph.input_type;
    }
}

function append_initial_context( json : any, graph : Graph )
{
    if( graph.context )
    {
        json[ "context" ] = graph.context;
    }
}

function json_node( node : NodeConfig | undefined ) : any
{
    if( ! node ) return undefined;

    let json : any = { 
        id : node.id, 
        executor : json_executor_from_node_executor( node.executor ),
        output : json_data_to_context( node.output ),
        destination : json_node_destinations( node.destination )
    };

    return json;
}

function json_executor_from_node_executor( executor : NodeExecutor | undefined ) : any
{
    if( ! executor ) return undefined;

    if( executor._variant == NodeExecutorVariant.Command )
    {
        return json_executor_command( executor );
    }
    else if( executor._variant == NodeExecutorVariant.ContextMut )
    {
        return json_executor_context_mut( executor );
    }
    else if( executor._variant == NodeExecutorVariant.AgentHistoryMut )
    {
        return json_executor_agent_history_mut( executor );
    }
    else if( executor._variant == NodeExecutorVariant.Agent )
    {
        return json_agent( executor );
    }
    else if( executor._variant == NodeExecutorVariant.WebClient )
    {
        return json_executor_web_client( executor );
    }
    else if( executor._variant == NodeExecutorVariant.Parallel )
    {
        return json_parallel( executor );
    }
    else if( executor._variant == NodeExecutorVariant.Graph )
    {
        return json_executor_graph( executor.value );
    }
}

function json_agent( executor : NodeExecutorAgent ) : any
{
    return {
        "Agent" : {
            provider : json_agent_provider( executor.value.provider ),
            system_prompt : json_vec_data_to_string( executor.value.system_prompt ),
            save_history : executor.value.save_history,
            prompt : json_vec_data_to_string( executor.value.prompt ),
            servers : json_vec_mcp_server( executor.value.servers ),
            is_stream : executor?.value?.is_stream ? true : false
        }
    }
}

function json_vec_mcp_server( servers : Array<NodeMCPServer> ) : any
{
    return servers.map( ( s : NodeMCPServer ) => json_mcp_server( s ) )
}

function json_mcp_server( server : NodeMCPServer ) : any
{
    return {
        command : server.command,
        arguments : json_vec_data_from( server.args )
    }
}

function json_agent_provider( provider : AIAgentProvider ) : any
{
    if( 
        provider._variant == AIAgentProviderConfigVariant.Anthropic ||
        provider._variant == AIAgentProviderConfigVariant.DeepSeek
    )
    {
        return {
            [provider._variant] : {
                api_key : provider.api_key,
                model : json_data_from( provider.model ),
                max_tokens : provider.max_tokens
            }
        }
    }
    else if( 
        provider._variant == AIAgentProviderConfigVariant.OpenAI ||
        provider._variant == AIAgentProviderConfigVariant.Gemini
    )
    {
        return {
            [provider._variant] : {
                api_key : provider.api_key,
                model : json_data_from( provider.model )
            }
        }
    }
    else if( 
        provider._variant == AIAgentProviderConfigVariant.Ollama
    )
    {
        return {
            [provider._variant] : {
                model : json_data_from( provider.model )
            }
        }
    }
}

function json_parallel( executor : NodeExecutorParallel ) : any
{
    return {
        "Parallel" : {
            executors : json_vec_executors_parallel( executor.value.executors )
        }
    }
}

function json_vec_executors_parallel( executors : Array<ParallelExecutor> ) : Array<any>
{
    return executors.map( ( e ) => json_executor_parallel( e ) );
}

function json_executor_parallel( executor : ParallelExecutor ) : any
{
    if( executor._variant == ParallelExecutorVariant.Command )
    {
        return {
            "Command" : {
                ty : executor.ty || undefined,
                executor : json_command( executor.executor ),
                condition : json_data_comparator( executor.condition )
            }
        };
    }
    else if( executor._variant == ParallelExecutorVariant.WebClient )
    {
        return {
            "WebClient" : {
                ty : executor.ty || undefined,
                executor : json_web_client( executor.executor ),
                condition : json_data_comparator( executor.condition )
            }
        };
    }
}

function json_executor_graph( node : GraphExecutor ) : any
{
    let json : any = { 
        "Graph" : {
            id : node.id,
            path : node.path,
            input : json_vec_data_to_string( node.input ),
            output : json_vec_graph_node_output( node.output )
        } 
    };

    return json;
}

function json_executor_command( executor : NodeExecutorCommand ) : any
{
    return {
        "Command" : json_command( executor.value )
    };
}

function json_command( command : Command ) : any
{
    return {
            command : json_data_from( command.command ),
            args : json_vec_data_from( command.args ),
            output : json_vec_command_output( command.output ),
            timeout : command.timeout
    }
}

function json_vec_command_output( output : Array<CommandOutput> ) : any
{
    return output.map( ( o : CommandOutput ) => json_command_output( o ) )
}

function json_command_output( output : CommandOutput ) : any
{
    let pre_suf = {
        prefix : output.prefix,
        suffix : output.suffix
    };

    if( 
        output._variant == CommandOutputVariant.Out ||
        output._variant == CommandOutputVariant.Err ||
        output._variant == CommandOutputVariant.Success ||
        output._variant == CommandOutputVariant.Code ||
        output._variant == CommandOutputVariant.Object
    )
    {
        return {
            [output._variant] : pre_suf
        }
    }
}

function json_executor_web_client( executor : NodeExecutorWebClient ) : any
{
    return {
        "WebClient" : json_web_client( executor.value )
    }
}

function json_web_client( web_client : WebClient ) : any
{
    return {
        url : json_data_from( web_client.url ),
        method : web_client.method,
        headers : json_vec_name_value( web_client.headers ),
        query_params : json_vec_name_value( web_client.query_params ),
        body : json_request_body( web_client.body ),
        output : json_vec_web_client_output( web_client.output ),
        timeout : web_client.timeout
    };
}

function json_vec_web_client_output( output : Array<WebClientOutput> ) : any
{
    return ( output || [] ).map( ( o : WebClientOutput ) => json_web_client_output( o ) )
}

function json_web_client_output( output : WebClientOutput ) : any
{
    let pre_suf : any = {
        prefix : output.prefix,
        suffix : output.suffix
    };

    if( 
        output._variant == WebClientOutputVariant.Version ||
        output._variant == WebClientOutputVariant.Status ||
        output._variant == WebClientOutputVariant.Body ||
        output._variant == WebClientOutputVariant.Object
    )
    {
        return {
            [output._variant] : pre_suf
        }
    }
    else if( output._variant == WebClientOutputVariant.Header )
    {
        pre_suf.name = output.name;

        return {
            [output._variant] : pre_suf
        }
    }
}

function json_request_body( body : WebClientBody | undefined ) : any
{
    if( ! body ) return undefined;

    if( body._variant == WebClientBodyVariant.Json )
    {
        return {
            "Json" : json_data_from( body.value )
        }
    }
    else if( body._variant == WebClientBodyVariant.Form )
    {
        return {
            "Form" : json_vec_name_value( body.value )
        }
    }
}

function json_vec_name_value( vec : Array<WebClientNameValue> ) : any
{
    return ( vec || [] ).map( ( o ) => json_name_value( o ) );
}

function json_name_value( obj : WebClientNameValue ) : any
{
    return {
        name : json_data_from( obj.name ),
        value : json_data_from( obj.value )
    }
}

function json_executor_context_mut( executor : NodeExecutorContextMut ) : any
{
    return {
        "ContextMut" : json_vec_context_mut( executor.value )
    };
}

function json_vec_context_mut( 
    context_muts : Array<ContextMut>
)
{
    return context_muts.map( ( c : ContextMut ) => json_context_mut( c ) )
}

function json_context_mut(
    context_mut : ContextMut
)
{
    return {
        from : context_mut.from ? json_data_from( context_mut.from ) : undefined,
        to : json_data_to_context( context_mut.to ),
        condition : json_data_comparator( context_mut.condition )
    }
}

function json_executor_agent_history_mut( executor : NodeExecutorAgentHistoryMut ) : any
{
    return {
        "AgentHistoryMut" : json_vec_agent_history_mut( executor.value )
    };
}

function json_vec_agent_history_mut( 
    history_muts : Array<AgentHistoryMut>
)
{
    return history_muts.map( ( c : AgentHistoryMut ) => json_agent_history_mut( c ) )
}

function json_agent_history_mut(
    history_mut : AgentHistoryMut
)
{
    return {
        id : history_mut.id,
        from : history_mut.from ? json_data_from( history_mut.from ) : undefined,
        to : json_data_to_agent_history( history_mut.to ),
        condition : json_data_comparator( history_mut.condition )
    }
}

function json_data_to_agent_history(
    data : DataToAgentHistory
)
{
    if( data._variant == DataToAgentHistoryVariant.Replace )
    {
        return "Replace";
    }
    else if( data._variant == DataToAgentHistoryVariant.ReplaceFirst )
    {
        return "ReplaceFirst";
    }
    else if( data._variant == DataToAgentHistoryVariant.ReplaceLast )
    {
        return "ReplaceLast";
    }
    else if( data._variant == DataToAgentHistoryVariant.ReplaceItem )
    {
        return {
            "ReplaceItem" : data.value
        };
    }
    else if( data._variant == DataToAgentHistoryVariant.StringToLast )
    {
        return "StringToLast";
    }
    else if( data._variant == DataToAgentHistoryVariant.StringToFirst )
    {
        return "StringToFirst";
    }
    else if( data._variant == DataToAgentHistoryVariant.StringToItem )
    {
        return {
            "StringToItem" : data.value
        };
    }
}

function json_vec_data_from( data_from : Array<DataFrom> ) : any
{
    return data_from.map( ( f : DataFrom ) => json_data_from( f ) )
}

function json_data_from( data_from : DataFrom ) : any
{
    if( 
        data_from._variant == DataFromVariant.Context ||
        data_from._variant == DataFromVariant.ParsedInput
    )
    {
        return {
            [data_from._variant] : {
                path : data_from.path,
                required : data_from.required
            }
        }
    }
    else if( data_from._variant == DataFromVariant.Input )
    {
        return {
            [data_from._variant] : {
                required : data_from.required
            }
        }
    }
    else if( data_from._variant == DataFromVariant.Static )
    {
        return {
            [data_from._variant] : json_from_static( data_from.value )
        }
    }
    else if( data_from._variant == DataFromVariant.Concat )
    {
        return {
            [data_from._variant] : json_vec_data_from( data_from.value )
        }
    }
    else if( data_from._variant == DataFromVariant.Operation )
    {
        return {
            [data_from._variant] : data_from.value ? json_data_operation( data_from.value ) : undefined
        }
    }
    else if( data_from._variant == DataFromVariant.Null )
    {
        return "Null"
    }
    else if( data_from._variant == DataFromVariant.AgentHistory )
    {
        return {
            [data_from._variant] : {
                id : data_from.id,
                content : json_from_agent_history_content( data_from.content )
            }
        }
    }
    else if( data_from._variant == DataFromVariant.Store )
    {
        return {
            [data_from._variant] : {
                id : data_from.id,
                query : json_data_from( data_from.query ),
                samples : data_from.samples
            }
        }
    }
}

function json_from_agent_history_content( content : FromAgentHistoryContent ) : any
{
    if( 
        content._variant == FromAgentHistoryContentVariant.Full ||
        content._variant == FromAgentHistoryContentVariant.FullMessages ||
        content._variant == FromAgentHistoryContentVariant.First ||
        content._variant == FromAgentHistoryContentVariant.FirstMessage ||
        content._variant == FromAgentHistoryContentVariant.Last ||
        content._variant == FromAgentHistoryContentVariant.LastMessage
    )
    {
        return content._variant;
    }
    else if( 
        content._variant == FromAgentHistoryContentVariant.Range ||
        content._variant == FromAgentHistoryContentVariant.RangeMessages
    )
    {
        return {
            [content._variant] : {
                from : content.from,
                to : content.to
            }
        }
    }
    else if( 
        content._variant == FromAgentHistoryContentVariant.Item ||
        content._variant == FromAgentHistoryContentVariant.ItemMessage
    )
    {
        return {
            [content._variant] : content.value
        }
    }
}

function json_from_static( input : any ) : any
{
    if( is_empty( input ) ) return "";

    if( typeof( input ) === "string" && input.trim() === "" ) return input;

    if( typeof( input ) === "number" ) return input;

    if( typeof( input ) === "boolean" ) return input;

    try
    {
        let json = JSON.parse( input );

        return json;
    }
    catch( e )
    {
        return json_number_string_or_boolean( input );
    }
}

function json_number_string_or_boolean( input : any ) : number | string | boolean
{
    if( typeof( input ) === "string" && input?.trim() == "" ) return input;

    if( typeof( input ) === "string" && ( input === "true" || input === "false" ) ) return input === "true";

    try
    {
        let n = Number( input );

        if( isNaN( n ) )
        {
            return input + "";
        }

        return n;
    }
    catch( e )
    {
        return input + "";
    }
}

function json_data_to_context( data : DataToContext | undefined ) : any
{
    if( ! data?.path?.trim() ) { return undefined; }

    return {
        path : data.path,
        ty : data.ty,
        merge : data.merge,
        optional : data.optional || false
    }
}

function json_node_destinations( destinations : Array<NodeDestination> ) : any
{
    return destinations.map( ( d : NodeDestination ) => json_node_destination( d ) )
}

function json_node_destination( destination : NodeDestination ) : any
{
    return {
        next : json_node_next( destination.next ),
        condition : json_data_comparator( destination.condition )
    };
}

function json_node_next( node_next : NodeNext | undefined ) : any
{
    if( ! node_next ) { return undefined; }

    if( node_next._variant == NodeNextVariant.Node )
    {
        return {
            "Node" : node_next.value
        }
    }
    else if( node_next._variant == NodeNextVariant.ExitOk || node_next._variant == NodeNextVariant.ExitErr )
    {
        return {
            [node_next._variant] : json_vec_data_to_string( node_next.value )
        }
    }
}

function json_data_comparator( data_comparator : DataComparator | undefined ) : any
{
    if( ! data_comparator ) { return undefined; }

    if( 
        data_comparator._variant == DataComparatorVariant.Eq ||
        data_comparator._variant == DataComparatorVariant.NotEq ||
        data_comparator._variant == DataComparatorVariant.Gt ||
        data_comparator._variant == DataComparatorVariant.Lt
    )
    {
        return {
            [data_comparator._variant] : {
                from_1 : data_comparator.from_1 ? json_data_from( data_comparator.from_1 ) : undefined,
                from_2 : data_comparator.from_2 ? json_data_from( data_comparator.from_2 ) : undefined,
            }
        }
    }
    else if( data_comparator._variant == DataComparatorVariant.Regex )
    {
        return {
            "Regex" : {
                regex : data_comparator.regex,
                from : data_comparator.from ? json_data_from( data_comparator.from ) : undefined,
            }
        }
    }
    else if( 
        data_comparator._variant == DataComparatorVariant.And ||
        data_comparator._variant == DataComparatorVariant.Or ||
        data_comparator._variant == DataComparatorVariant.Xor ||
        data_comparator._variant == DataComparatorVariant.Nand
    )
    {
        return {
            [data_comparator._variant] : {
                comp_1 : data_comparator.comp_1 ? json_data_comparator( data_comparator.comp_1 ) : undefined,
                comp_2 : data_comparator.comp_2 ? json_data_comparator( data_comparator.comp_2 ) : undefined,
            }
        }
    }
    else if( data_comparator._variant == DataComparatorVariant.Not )
    {
        return {
            [data_comparator._variant] : data_comparator.value ? json_data_comparator( data_comparator.value ) : undefined
        }
    }
    else if( 
        data_comparator._variant == DataComparatorVariant.Empty || 
        data_comparator._variant == DataComparatorVariant.NotEmpty
    )
    {
        return {
            [data_comparator._variant] : data_comparator.value ? json_data_from( data_comparator.value ) : undefined
        }
    }
    else if( 
        data_comparator._variant == DataComparatorVariant.True || 
        data_comparator._variant == DataComparatorVariant.False
    )
    {
        return data_comparator._variant;
    }
}

function json_vec_data_to_string( data : Array<DataToString> ) : any
{
    return data.map( ( d : DataToString ) => json_data_to_string( d ) )
}

function json_data_to_string( data : DataToString ) : any
{
    return {
        from : data.from ? json_data_from( data.from ) : undefined,
        prefix : data.prefix,
        suffix : data.suffix
    }
}

function json_vec_graph_node_output( output : Array<GraphNodeOutput> ) : any
{
    return output.map( ( d : GraphNodeOutput ) => json_graph_node_output( d ) )
}

function json_graph_node_output( output : GraphNodeOutput ) : any
{
    let pre_suf = {
        prefix : output.prefix,
        suffix : output.suffix
    };

    if( 
        output._variant == GraphNodeOutputVariant.Out || 
        output._variant == GraphNodeOutputVariant.Err ||
        output._variant == GraphNodeOutputVariant.Success ||
        output._variant == GraphNodeOutputVariant.Object
    )
    {
        return {
            [output._variant] : pre_suf
        }
    }
}

function json_data_operation( data : DataOperation ) : any
{
    if( data._variant == DataOperationVariant.Len )
    {
        return {
            [data._variant] : data.value ? json_data_from( data.value ) : undefined
        }
    }
    else if(
        data._variant == DataOperationVariant.Add ||
        data._variant == DataOperationVariant.Substract
    )
    {
        return {
            [data._variant] : {
                num_1 : data.num_1 ? json_data_from( data.num_1 ) : undefined,
                num_2 : data.num_2 ? json_data_from( data.num_2 ) : undefined
            }
        }
    }
    else if( data._variant == DataOperationVariant.StringSplit )
    {
        return {
            [data._variant] : {
                from : json_data_from( data.from ),
                sep : is_empty( data.sep ) ? "" : ( data.sep + "" )
            }
        }
    }
}