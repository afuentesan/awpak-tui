import { AIAgent, AIAgentProviderAnthropic, AIAgentProviderDeepSeek, AIAgentProviderGemini, AIAgentProviderOllama, AIAgentProviderOpenAI, NodeMCPServer, type AIAgentProvider } from "../model/agent";
import { DataToAgentHistoryReplace, DataToAgentHistoryReplaceFirst, DataToAgentHistoryReplaceItem, DataToAgentHistoryReplaceLast, DataToAgentHistoryStringToFirst, DataToAgentHistoryStringToItem, DataToAgentHistoryStringToLast, type AgentHistoryMut, type DataToAgentHistory } from "../model/agent_history_mut";
import { Command, CommandOutputCode, CommandOutputErr, CommandOutputObject, CommandOutputOut, CommandOutputSuccess, type CommandOutput } from "../model/command";
import type { ContextMut } from "../model/context_mut";
import { DataMerge, DataOperationAdd, DataOperationLen, DataOperationSubstract, DataToContext, DataToString, DataType, FromAgentHistory, FromAgentHistoryContentFirst, FromAgentHistoryContentFirstMessage, FromAgentHistoryContentFull, FromAgentHistoryContentFullMessages, FromAgentHistoryContentItem, FromAgentHistoryContentItemMessage, FromAgentHistoryContentLast, FromAgentHistoryContentLastMessage, FromAgentHistoryContentRange, FromAgentHistoryContentRangeMessages, FromConcat, FromContext, FromInput, FromNull, FromOperation, FromParsedInput, FromStatic, type DataFrom, type FromAgentHistoryContent } from "../model/data";
import { DataComparatorAnd, DataComparatorEq, DataComparatorFalse, DataComparatorGt, DataComparatorLt, DataComparatorNot, DataComparatorNotEq, DataComparatorOr, DataComparatorRegex, DataComparatorTrue, type DataComparator } from "../model/data_comparator";
import { Graph } from "../model/graph";
import { GraphNode, GraphNodeOutputErr, GraphNodeOutputObject, GraphNodeOutputOut, GraphNodeOutputSuccess, Node, NodeDestination, NodeNextExitErr, NodeNextExitOk, NodeNextNode, type GraphNodeOutput, type NodeNext, type NodeType } from "../model/node";
import { NodeExecutorAgent, NodeExecutorAgentHistoryMut, NodeExecutorCommand, NodeExecutorContextMut, NodeExecutorParallel, NodeExecutorWebClient, type NodeExecutor } from "../model/node_executor";
import { Parallel, ParallelExecutorCommand, ParallelExecutorWebClient, type ParallelExecutor } from "../model/parallel";
import { AwpakMethod, WebClient, WebClientBodyForm, WebClientBodyJson, WebClientNameValue, WebClientOutputBody, WebClientOutputHeader, WebClientOutputObject, WebClientOutputStatus, WebClientOutputVersion, type WebClientBody, type WebClientOutput } from "../model/web_client";
import { is_empty, not_empty_or_string_eq } from "./data_functions";
import { is_type_in_enum } from "./form_utils";

export function load_graph_from_json( json : any ) : Graph
{
    return {
        context : load_context( json ),
        preserve_context : json.preserve_context || false,
        input_type : load_data_type( json.input_type ),
        first : load_node( json.first ),
        nodes : load_nodes( json.nodes )
    };
}

function load_nodes( nodes : Array<any> ) : Array<NodeType>
{
    if( ! nodes?.length ) return [];

    return nodes.map( ( n ) => load_node( n ) )
}

function load_node( node : any ) : NodeType
{
    if( node?.[ "Node" ] )
    {
        return load_node_node( node[ "Node" ] );
    }
    else if( node?.[ "Graph" ] )
    {
        return load_node_graph_node( node[ "Graph" ] );
    }
    
    throw new Error( "Node type not found. " + JSON.stringify( node ) );
}

function load_node_node( node : any ) : NodeType
{
    let ret = new Node( node.id );

    if ( node.executor ) ret.executor = load_node_executor( node.executor );

    if( node.output ) ret.output = load_data_to_context( node.output );
    

    if ( node.destination ) ret.destination = load_destinations( node.destination );

    return ret;
}

function load_node_graph_node( node : any ) : NodeType
{
    let ret = new GraphNode( node.id );

    ret.path = node.path;

    ret.input = load_vec_data_to_string( node.input );
    ret.output = load_graph_node_outputs( node.output );

    ret.node_output = load_data_to_context( node.node_output );
    ret.node_destination = load_destinations( node.node_destination );

    return ret;
}

function load_graph_node_outputs( outputs : Array<any> ) : Array<GraphNodeOutput>
{
    return outputs.map( ( o ) => load_graph_node_output( o ) );
}

function load_graph_node_output( output : any ) : GraphNodeOutput
{
    if( ! is_empty( output?.[ "Out" ] ) )
    {
        return load_graph_node_output_prefix_suffix( output[ "Out" ], new GraphNodeOutputOut() );
    }
    else if( ! is_empty( output?.[ "Err" ] ) )
    {
        return load_graph_node_output_prefix_suffix( output[ "Err" ], new GraphNodeOutputErr() );
    }
    else if( ! is_empty( output?.[ "Success" ] ) )
    {
        return load_graph_node_output_prefix_suffix( output[ "Success" ], new GraphNodeOutputSuccess() );
    }
    else if( ! is_empty( output?.[ "Object" ] ) )
    {
        return load_graph_node_output_prefix_suffix( output[ "Object" ], new GraphNodeOutputObject() );
    }

    throw new Error( "GraphNodeOutput not found. " + JSON.stringify( output ) );
}

function load_graph_node_output_prefix_suffix( 
    output : any, 
    src : GraphNodeOutputOut | GraphNodeOutputErr | GraphNodeOutputSuccess | GraphNodeOutputObject
) : GraphNodeOutputOut | GraphNodeOutputErr | GraphNodeOutputSuccess | GraphNodeOutputObject
{
    src.prefix = output.prefix;
    src.suffix = output.suffix;

    return src;
}

function load_node_executor( executor : any ) : NodeExecutor
{
    if( executor?.[ "Command" ] )
    {
        return load_node_executor_command( executor[ "Command" ] );
    }
    else if( executor?.[ "ContextMut" ] )
    {
        return load_node_executor_context_mut( executor[ "ContextMut" ] );
    }
    else if( executor?.[ "AgentHistoryMut" ] )
    {
        return load_node_executor_agent_history_mut( executor[ "AgentHistoryMut" ] );
    }
    else if( executor?.[ "Agent" ] )
    {
        return load_node_executor_agent( executor[ "Agent" ] );
    }
    else if( executor?.[ "WebClient" ] )
    {
        return load_node_executor_web_client( executor[ "WebClient" ] );
    }
    else if( executor?.[ "Parallel" ] )
    {
        return load_node_executor_parallel( executor[ "Parallel" ] );
    }

    throw new Error( "NodeExecutor not found. " + JSON.stringify( executor ) );
}

function load_node_executor_parallel( parallel : any ) : NodeExecutorParallel
{
    let ret = new NodeExecutorParallel();

    let value = new Parallel();

    value.executors = load_parallel_executors( parallel.executors );

    ret.value = value;

    return ret;
}

function load_parallel_executors( executors : Array<any> ) : Array<ParallelExecutor>
{
    if( ! executors?.length ) return [];

    return executors.map( ( e ) => load_parallel_executor( e ) );
}

function load_parallel_executor( executor : any ) : ParallelExecutor
{
    if( executor?.[ "Command" ]?.executor )
    {
        let ret = new ParallelExecutorCommand();

        ret.executor = load_command( executor[ "Command" ].executor );
        ret.ty = executor[ "Command" ].ty || undefined;
        ret.condition = load_data_comparator( executor[ "Command" ].condition );

        return ret;
    }
    else if( executor?.[ "WebClient" ]?.executor )
    {
        let ret = new ParallelExecutorWebClient();

        ret.executor = load_web_client( executor[ "WebClient" ].executor );
        ret.ty = executor[ "WebClient" ].ty || undefined;
        ret.condition = load_data_comparator( executor[ "WebClient" ].condition );

        return ret;
    }

    throw new Error( "ParallelExecutor not found. " + JSON.stringify( executor ) );
}

function load_node_executor_command( command : any ) : NodeExecutorCommand
{
    let ret = new NodeExecutorCommand();

    ret.value = load_command( command );

    return ret;
}

function load_command( command : any ) : Command
{
    let value = new Command();

    value.args = load_vec_data_from( command.args );
    value.command = command.command;

    value.output = load_vec_command_output( command.output );

    return value;
}

function load_vec_command_output( output : Array<any> ) : Array<CommandOutput>
{
    return output.map( ( o ) => load_command_output( o ) );
}

function load_command_output( output : any ) : CommandOutput
{
    if( output?.[ "Out" ] )
    {
        return load_output_prefix_suffix( output[ "Out" ], new CommandOutputOut() ) as CommandOutput;
    }
    else if( output?.[ "Err" ] )
    {
        return load_output_prefix_suffix( output[ "Err" ], new CommandOutputErr() ) as CommandOutput;
    }
    else if( output?.[ "Success" ] )
    {
        return load_output_prefix_suffix( output[ "Success" ], new CommandOutputSuccess() ) as CommandOutput;
    }
    else if( output?.[ "Code" ] )
    {
        return load_output_prefix_suffix( output[ "Code" ], new CommandOutputCode() ) as CommandOutput;
    }
    else if( output?.[ "Object" ] )
    {
        return load_output_prefix_suffix( output[ "Object" ], new CommandOutputObject() ) as CommandOutput;
    }

    throw new Error( "CommandOutput not found. " + JSON.stringify( output ) );
}

type PrefixSuffix = CommandOutputOut | CommandOutputErr | CommandOutputSuccess | CommandOutputCode | CommandOutputObject |
                    WebClientOutputVersion |
                    WebClientOutputStatus |
                    WebClientOutputHeader |
                    WebClientOutputBody |
                    WebClientOutputObject;

function load_output_prefix_suffix( 
    output : any, 
    src : PrefixSuffix
) : PrefixSuffix
{
    src.prefix = output.prefix;
    src.suffix = output.suffix;

    return src;
}

function load_node_executor_web_client( web_client : any ) : NodeExecutorWebClient
{
    let ret = new NodeExecutorWebClient();

    ret.value = load_web_client( web_client );

    return ret;
}

function load_web_client( web_client : any ) : WebClient
{
    let value = new WebClient();

    value.url = load_data_from( web_client.url );
    value.method = load_method( web_client.method ) || AwpakMethod.Get;
    value.headers = load_vec_name_value( web_client.headers );
    value.query_params = load_vec_name_value( web_client.query_params );
    value.body = load_body( web_client.body );
    value.output = load_vec_web_client_output( web_client.output );

    return value;
}

function load_vec_web_client_output( output : Array<any> ) : Array<WebClientOutput>
{
    return ( output || [] ).map( ( o ) => load_web_client_output( o ) );
}

function load_web_client_output( output : any ) : WebClientOutput
{
    if( output?.[ "Version" ] )
    {
        return load_output_prefix_suffix( output[ "Version" ], new WebClientOutputVersion() ) as WebClientOutput;
    }
    else if( output?.[ "Status" ] )
    {
        return load_output_prefix_suffix( output[ "Status" ], new WebClientOutputStatus() ) as WebClientOutput;
    }
    else if( output?.[ "Header" ] )
    {
        let w = new WebClientOutputHeader();

        w.name = output[ "Header" ].name || "";

        return load_output_prefix_suffix( output[ "Header" ], w ) as WebClientOutput;
    }
    else if( output?.[ "Body" ] )
    {
        return load_output_prefix_suffix( output[ "Body" ], new WebClientOutputBody() ) as WebClientOutput;
    }
    else if( output?.[ "Object" ] )
    {
        return load_output_prefix_suffix( output[ "Object" ], new WebClientOutputObject() ) as WebClientOutput;
    }

    throw new Error( "WebClientOutput not found. " + JSON.stringify( output ) );
}

function load_body( body : any ) : WebClientBody | undefined
{
    if( ! body ) return undefined;

    if( ! is_empty( body?.[ "Json" ] ) )
    {
        let ret = new WebClientBodyJson();

        ret.value = load_data_from( body[ "Json" ] );

        return ret;
    }
    else if( ! is_empty( body?.[ "Form" ] ) )
    {
        let ret = new WebClientBodyForm();

        ret.value = load_vec_name_value( body[ "Form" ] );

        return ret;
    }
}

function load_method( method : any ) : AwpakMethod | undefined
{
    if( ! method?.trim() || ! is_type_in_enum( AwpakMethod, method ) ) return undefined;

    return method as AwpakMethod
}

function load_vec_name_value( data : Array<any> ) : Array<WebClientNameValue>
{
    return ( data || [] ).map( ( d ) => load_name_value( d ) );
}

function load_name_value( data : any ) : WebClientNameValue
{
    return {
        name : load_data_from( data.name ),
        value : load_data_from( data.value )
    }
}

function load_node_executor_agent( agent : any ) : NodeExecutorAgent
{
    let ret = new NodeExecutorAgent();

    let value = new AIAgent();

    value.provider = load_ai_agent_provider( agent.provider );
    
    value.system_prompt = agent.system_prompt;
    value.save_history = agent.save_history ? true : false;

    if( agent.servers ) value.servers = load_mcp_servers( agent.servers );

    if( agent.prompt ) value.prompt = load_vec_data_to_string( agent.prompt );

    ret.value = value;

    return ret;
}

function load_ai_agent_provider( provider : any ) : AIAgentProvider
{
    if( provider?.[ "Ollama" ] )
    {
        return load_ai_agent_provider_model( provider[ "Ollama" ], new AIAgentProviderOllama() );
    }
    else if( provider?.[ "OpenAI" ] )
    {
        return load_ai_agent_provider_model_api_key( provider[ "OpenAI" ], new AIAgentProviderOpenAI() );
    }
    else if( provider?.[ "Gemini" ] )
    {
        return load_ai_agent_provider_model_api_key( provider[ "Gemini" ], new AIAgentProviderGemini() );
    }
    else if( provider?.[ "Anthropic" ] )
    {
        return load_ai_agent_provider_model_api_key_max_tokens( provider[ "Anthropic" ], new AIAgentProviderAnthropic() );
    }
    else if( provider?.[ "DeepSeek" ] )
    {
        return load_ai_agent_provider_model_api_key_max_tokens( provider[ "DeepSeek" ], new AIAgentProviderDeepSeek() );
    }

    throw new Error( "AIAgentProvider not found. " + JSON.stringify( provider ) );
}

function load_ai_agent_provider_model( 
    provider : any,
    src : AIAgentProviderOllama
) : AIAgentProviderOllama
{
    src.model = provider.model;

    return src;
}

function load_ai_agent_provider_model_api_key( 
    provider : any,
    src : AIAgentProviderOpenAI | AIAgentProviderGemini
) : AIAgentProviderOpenAI | AIAgentProviderGemini
{
    src.model = provider.model;
    src.api_key = provider.api_key;

    return src;
}

function load_ai_agent_provider_model_api_key_max_tokens( 
    provider : any,
    src : AIAgentProviderAnthropic | AIAgentProviderDeepSeek
) : AIAgentProviderAnthropic | AIAgentProviderDeepSeek
{
    src.model = provider.model;
    src.api_key = provider.api_key;
    src.max_tokens = provider.max_tokens;

    return src;
}

function load_mcp_servers( servers : Array<any> ) : Array<NodeMCPServer>
{
    return servers.map( ( s ) => load_mcp_server( s ) );
}

function load_mcp_server( server : any ) : NodeMCPServer
{
    return {
        command : server.command,
        args : load_vec_data_from( server.arguments )
    }
}

function load_node_executor_context_mut( context_mut : Array<any> ) : NodeExecutorContextMut
{
    let ret = new NodeExecutorContextMut();

    ret.value = context_mut.map( ( c ) => load_item_node_executor_context_mut( c ) );

    return ret;
}

function load_item_node_executor_context_mut( item : any ) : ContextMut
{
    return {
        from : load_data_from( item.from ),
        to : item.to ? load_data_to_context( item.to ) : undefined,
        condition : load_data_comparator( item.condition )
    }
}

function load_node_executor_agent_history_mut( context_mut : Array<any> ) : NodeExecutorAgentHistoryMut
{
    let ret = new NodeExecutorAgentHistoryMut();

    ret.value = context_mut.map( ( c ) => load_item_node_executor_agent_history_mut( c ) );

    return ret;
}

function load_item_node_executor_agent_history_mut( item : any ) : AgentHistoryMut
{
    return {
        id : item.id,
        from : load_data_from( item.from ),
        to : item.to ? load_data_to_agent_history( item.to ) : new DataToAgentHistoryReplace(),
        condition : load_data_comparator( item.condition )
    }
}

function load_data_to_agent_history( data : any ) : DataToAgentHistory
{
    if( not_empty_or_string_eq( data, "Replace" ) )
    {
        return new DataToAgentHistoryReplace();
    }
    else if( not_empty_or_string_eq( data, "ReplaceFirst" ) )
    {
        return new DataToAgentHistoryReplaceFirst();
    }
    else if( not_empty_or_string_eq( data, "ReplaceLast" ) )
    {
        return new DataToAgentHistoryReplaceLast();
    }
    else if( ! is_empty( data?.[ "ReplaceItem" ] ) )
    {
        let ret = new DataToAgentHistoryReplaceItem();

        ret.value = data[ "ReplaceItem" ];

        return ret;
    }
    else if( not_empty_or_string_eq( data, "StringToLast" ) )
    {
        return new DataToAgentHistoryStringToLast();
    }
    else if( not_empty_or_string_eq( data, "StringToFirst" ) )
    {
        return new DataToAgentHistoryStringToFirst();
    }
    else if( ! is_empty( data?.[ "StringToItem" ] ) )
    {
        let ret = new DataToAgentHistoryStringToItem();

        ret.value = data[ "StringToItem" ];

        return ret;
    }

    return new DataToAgentHistoryReplace();
}

function load_data_to_context( data : any ) : DataToContext
{
    return {
        path : data.path,
        ty : load_data_type( data.ty ),
        merge : load_merge( data.merge ),
        optional : data.optional || false
    }
}

function load_merge( merge : any ) : DataMerge | undefined
{
    if( ! merge?.trim() || ! is_type_in_enum( DataMerge, merge ) ) return undefined;

    return merge as DataMerge
}

function load_destinations( destinations : Array<any> ) : Array<NodeDestination>
{
    return destinations.map( ( d ) => load_destination( d ) );
}

function load_destination( destination : any ) : NodeDestination
{
    return {
        next : load_node_next( destination.next ),
        condition : load_data_comparator( destination.condition )
    }
}

function load_node_next( next : any ) : NodeNext
{
    if( next?.[ "Node" ] )
    {
        return load_node_next_node( next[ "Node" ] );
    }
    else if( next?.[ "ExitOk" ] )
    {
        return load_node_next_exit_ok( next[ "ExitOk" ] );
    }
    else if( next?.[ "ExitErr" ] )
    {
        return load_node_next_exit_err( next[ "ExitErr" ] );
    }

    throw new Error( "NodeNext not found. " + JSON.stringify( next ) );
}

function load_node_next_node( next : any ) : NodeNextNode
{
    let ret = new NodeNextNode();

    ret.value = next;

    return ret;
}

function load_node_next_exit_ok( next : any ) : NodeNextExitOk
{
    let ret = new NodeNextExitOk();

    ret.value = load_vec_data_to_string( next );

    return ret;
}

function load_node_next_exit_err( next : any ) : NodeNextExitErr
{
    let ret = new NodeNextExitErr();

    ret.value = load_vec_data_to_string( next );

    return ret;
}

function load_data_comparator( data : any ) : DataComparator
{
    if( data?.[ "Eq" ] )
    {
        return load_data_comparator_from_1_from_2( data[ "Eq" ], new DataComparatorEq() );
    }
    else if( data?.[ "NotEq" ] )
    {
        return load_data_comparator_from_1_from_2( data[ "NotEq" ], new DataComparatorNotEq() );
    }
    else if( data?.[ "Gt" ] )
    {
        return load_data_comparator_from_1_from_2( data[ "Gt" ], new DataComparatorGt() );
    }
    else if( data?.[ "Lt" ] )
    {
        return load_data_comparator_from_1_from_2( data[ "Lt" ], new DataComparatorLt() );
    }
    else if( data?.[ "Regex" ] )
    {
        return load_data_comparator_regex( data[ "Regex" ] );
    }
    else if( data?.[ "And" ] )
    {
        return load_data_comparator_comp_1_comp_2( data[ "And" ], new DataComparatorAnd() );
    }
    else if( data?.[ "Or" ] )
    {
        return load_data_comparator_comp_1_comp_2( data[ "Or" ], new DataComparatorOr() );
    }
    else if( data?.[ "Not" ] )
    {
        return load_data_comparator_value( data[ "Not" ], new DataComparatorNot() );
    }
    else if( data?.[ "True" ] || ( typeof( data ) === "string" && data == "True" ) )
    {
        return new DataComparatorTrue();
    }
    else if( data?.[ "False" ] || ( typeof( data ) === "string" && data == "False" ) )
    {
        return new DataComparatorFalse();
    }
    else
    {
        return new DataComparatorTrue();
    }

    // throw new Error( "DataComparator not found. " + JSON.stringify( data ) );
}

function load_data_comparator_value( data : any, src : DataComparatorNot ) : DataComparatorNot
{
    src.value = load_data_comparator( data );

    return src;
}

function load_data_comparator_regex( data : any ) : DataComparatorRegex
{
    let ret = new DataComparatorRegex();

    ret.regex = data.regex;
    ret.from = load_data_from( data.from );

    return ret;
}

function load_data_comparator_comp_1_comp_2(
    data : any,
    src : DataComparatorAnd | DataComparatorOr
) : DataComparatorAnd | DataComparatorOr
{
    src.comp_1 = load_data_comparator( data.comp_1 );
    src.comp_2 = load_data_comparator( data.comp_2 );

    return src;
}

function load_data_comparator_from_1_from_2( 
    data : any, 
    src : DataComparatorEq | DataComparatorNotEq | DataComparatorGt | DataComparatorLt
) : DataComparatorEq | DataComparatorNotEq | DataComparatorGt | DataComparatorLt
{
    src.from_1 = load_data_from( data.from_1 );
    src.from_2 = load_data_from( data.from_2 );
    
    return src
}

function load_vec_data_to_string( data : Array<any> ) : Array<DataToString>
{
    return data.map( ( d ) => load_data_to_string( d ) );
}

function load_data_to_string( data : any ) : DataToString
{
    return {
        from : load_data_from( data.from ),
        prefix : data.prefix,
        suffix : data.suffix
    }
}

function load_vec_data_from( data : Array<any> ) : Array<DataFrom>
{
    return data.map( ( d ) => load_data_from( d ) );
}

function load_data_from( data : any ) : DataFrom
{
    if( ! is_empty( data?.[ "Context" ] ) )
    {
        return load_from_context( data[ "Context" ] );
    }
    else if( ! is_empty( data?.[ "ParsedInput" ] ) )
    {
        return load_from_parsed_input( data[ "ParsedInput" ] );
    }
    else if( ! is_empty( data?.[ "Static" ] ) )
    {
        return load_from_static( data[ "Static" ] );
    }
    else if( ! is_empty( data?.[ "Input" ] ) )
    {
        return load_from_input( data[ "Input" ] );
    }
    else if( ! is_empty( data?.[ "Operation" ] ) )
    {
        return load_from_operation( data[ "Operation" ] );
    }
    else if( ! is_empty( data?.[ "Concat" ] ) )
    {
        return load_from_concat( data[ "Concat" ] );
    }
    else if( ! is_empty( data?.[ "AgentHistory" ] ) )
    {
        return load_from_agent_history( data[ "AgentHistory" ] );
    }
    else if( typeof( data ) === "string" && data === "Null" )
    {
        return new FromNull();
    }
    
    throw new Error( "DataFrom not found. " + JSON.stringify( data ) );
}

function load_from_agent_history( data : any ) : FromAgentHistory
{
    let ret = new FromAgentHistory();

    ret.id = data.id;
    ret.content = load_from_agent_history_content( data.content );

    return ret;
}

function load_from_agent_history_content( data : any ) : FromAgentHistoryContent
{
    if( not_empty_or_string_eq( data, "Full" ) )
    {
        return new FromAgentHistoryContentFull();
    }
    else if( not_empty_or_string_eq( data, "FullMessages" ) )
    {
        return new FromAgentHistoryContentFullMessages();
    }
    else if( not_empty_or_string_eq( data, "First" ) )
    {
        return new FromAgentHistoryContentFirst();
    }
    else if( not_empty_or_string_eq( data, "FirstMessage" ) )
    {
        return new FromAgentHistoryContentFirstMessage();
    }
    else if( not_empty_or_string_eq( data, "Last" ) )
    {
        return new FromAgentHistoryContentLast();
    }
    else if( not_empty_or_string_eq( data, "LastMessage" ) )
    {
        return new FromAgentHistoryContentLastMessage();
    }
    else if( ! is_empty( data?.[ "Range" ] ) )
    {
        let ret = new FromAgentHistoryContentRange();

        ret.from = data?.[ "Range" ].from;
        ret.to = data?.[ "Range" ].to;

        return ret;
    }
    else if( ! is_empty( data?.[ "RangeMessages" ] ) )
    {
        let ret = new FromAgentHistoryContentRangeMessages();

        ret.from = data?.[ "RangeMessages" ].from;
        ret.to = data?.[ "RangeMessages" ].to;

        return ret;
    }
    else if( ! is_empty( data?.[ "Item" ] ) )
    {
        let ret = new FromAgentHistoryContentItem();

        ret.value = data[ "Item" ];

        return ret;
    }
    else if( ! is_empty( data?.[ "ItemMessage" ] ) )
    {
        let ret = new FromAgentHistoryContentItemMessage();

        ret.value = data[ "ItemMessage" ];

        return ret;
    }

    return new FromAgentHistoryContentFull();
}

function load_from_context( data : any ) : FromContext
{
    let ret = new FromContext();

    ret.path = data.path;
    ret.required = data.required ? true : false;

    return ret;
}

function load_from_parsed_input( data : any ) : FromParsedInput
{
    let ret = new FromParsedInput();

    ret.path = data.path;
    ret.required = data.required ? true : false;

    return ret;
}

function load_from_static( data : any ) : FromStatic
{
    let ret = new FromStatic;

    if( typeof( data ) == "object" )
    {
        data = JSON.stringify( data );
    }

    ret.value = data;

    return ret;
}

function load_from_input( data : any ) : FromInput
{
    let ret = new FromInput();

    ret.required = data.required ? true : false;

    return ret;
}

function load_from_operation( data : any ) : FromOperation
{
    let ret = new FromOperation();

    if( data?.[ "Len" ] )
    {
        ret.value = load_from_operation_len( data[ "Len" ] );

        return ret;
    }
    else if( data?.[ "Add" ] )
    {
        ret.value = load_from_operation_add( data[ "Add" ] );

        return ret;
    }
    else if( data?.[ "Substract" ] )
    {
        ret.value = load_from_operation_substract( data[ "Substract" ] );

        return ret;
    }

    throw new Error( "DataOperation not found. " + JSON.stringify( data ) );
}

function load_from_operation_len( data : any ) : DataOperationLen
{
    let ret = new DataOperationLen();

    ret.value = load_data_from( data );

    return ret;
}

function load_from_operation_add( data : any ) : DataOperationAdd
{
    let ret = new DataOperationAdd();

    ret.num_1 = load_data_from( data.num_1 );
    ret.num_2 = load_data_from( data.num_2 );

    return ret;
}

function load_from_operation_substract( data : any ) : DataOperationSubstract
{
    let ret = new DataOperationSubstract();

    ret.num_1 = load_data_from( data.num_1 );
    ret.num_2 = load_data_from( data.num_2 );

    return ret;
}

function load_from_concat( data : any ) : FromConcat
{
    let ret = new FromConcat;

    ret.value = load_vec_data_from( data );

    return ret;
}

function load_context( json : any ) : Map<string, any> | undefined
{
    let ctx = Object.assign( new Map(), json.context );

    return ctx;
}

function load_data_type( data_type : any ) : DataType | undefined
{
    if( ! data_type?.trim() || ! is_type_in_enum( DataType, data_type ) ) return undefined;

    return data_type as DataType
}