import { atom } from 'nanostores';
import { Graph } from './model/graph';
import { GraphNodeOutputVariant, Node, NodeDestination, NodeNextExitErr, NodeNextExitOk, NodeNextNode, NodeNextVariant, NodeTypeVariant, type NodeType } from './model/node';
import { DataFromVariant, DataMerge, DataToContext, DataToString, DataType, DataOperationVariant, FromAgentHistoryContentVariant } from './model/data';
import { change_node_next_variant, change_node_variant, clean_graph_node_ids, new_agent_provider_variant, new_command_node_output_variant, new_graph_node_output_variant, new_node_executor_variant, new_parallel_executor_variant, new_web_client_output_variant, next_node_id, node_by_id, update_graph_node_ids } from './functions/node_functions';
import { JSONPath } from 'jsonpath-plus';
import { is_empty, json_stringify, new_body_variant, new_data_comparator_variant, new_data_from_agent_history_content, new_data_from_variant, new_data_operation_variant, new_data_to_agent_history } from './functions/data_functions';
import { is_type_in_enum } from './functions/form_utils';
import type { DataComparatorVariant } from './model/data_comparator';
import type { NodeExecutorVariant } from './model/node_executor';
import type { CommandOutputVariant } from './model/command';
import type { AIAgentProviderConfigVariant } from './model/agent';
import { load_graph_from_json } from './functions/load_json';
import { ID_EXIT_ERR, ID_EXIT_OK } from './functions/graph_to_cytoscape';
import { AwpakMethod, WebClientBodyVariant, WebClientOutputVariant } from './model/web_client';
import type { DataToAgentHistoryVariant } from './model/agent_history_mut';
import type { ParallelExecutorVariant } from './model/parallel';

const KEY_LOCAL_STORAGE : string = "AWPAK_GRAPH";

export const graph = atom( from_local_or_new() );

function from_local_or_new() : Graph
{
    try
    {
        let json_graph = localStorage.getItem( KEY_LOCAL_STORAGE );

        if( ! json_graph?.trim() ) return default_new_graph();

        let graph : Graph = JSON.parse( json_graph );

        return graph;
    }
    catch( _e )
    {
        return default_new_graph();
    }
}

function default_new_graph() : Graph
{
    let g = new Graph();

    g.context = new Map();
    g.preserve_context = false;

    g.first = new Node( "Entry node" );

    return g;
}

function change_graph_and_local_save( new_graph : Graph )
{
    try
    {
        graph_local_save( new_graph );
    }
    catch( e )
    {
        console.warn( e );
    }

    graph.set( new_graph );
}

function graph_local_save( new_graph : Graph )
{
    localStorage.setItem( KEY_LOCAL_STORAGE, json_stringify( new_graph ) );
}

export function clear_graph()
{
    if( ! check_time() ) return;

    if( ! confirm( "This operation cannot be undone" ) )
    {
        return;
    }

    LAST_TIME = ( new Date() ).getTime();

    let new_graph = default_new_graph();

    graph.set( new_graph );
}

export function load_new_graph( json_str : string )
{
    if( ! json_str?.trim() ) { return; }

    try
    {
        let json = JSON.parse( json_str );

        let new_graph = load_graph_from_json( json );

        if( ! new_graph ) return;

        change_graph_and_local_save( new_graph );
    }
    catch( e )
    {
        console.log( "Error load new graph. ", e );
    }
}

export function element_from_path( graph : Graph, path : string ) : any
{
    const result = JSONPath( { path : path, json : graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].value ) { return undefined; }

    return result[ 0 ].value;
}

export function clone_and_append_to_array( array_path : string, object : any )
{
    if( is_empty( object ) ) return;

    let cloned;

    try
    {
        cloned = JSON.parse( json_stringify( object ) );
    }
    catch( e )
    {
        console.error( e );

        return;
    }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : array_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    ( result[ 0 ].parent[ result[ 0 ].parentProperty ] as Array<any> ).push( cloned );

    change_graph_and_local_save( new_graph );
}

export function add_node()
{
    if( ! check_time() ) return;

    let new_graph = Object.assign( {}, graph.get() );

    let id = next_node_id( new_graph );

    new_graph.nodes.push( new Node( id ) );

    new_graph.nodes = [ ...new_graph.nodes ];

    change_graph_and_local_save( new_graph );
}

export function change_node_id( id : string, new_id : string )
{
    if( ! new_id?.trim() || ! id?.trim() || id.trim() == new_id.trim() ) { return; }

    let new_graph = Object.assign( {}, graph.get() );

    if( new_graph.first?.id == id )
    {
        new_graph.first.id = new_id;

        new_graph.first = Object.assign( {}, new_graph.first );

        update_graph_node_ids( new_graph, id, new_id );

        change_graph_and_local_save( new_graph );

        return;
    }

    new_graph.nodes = new_graph.nodes.map( ( n : NodeType ) => 
        {
            if( n.id == id )
            {
                n.id = new_id;

                return Object.assign( {}, n );
            }

            return n;
        }
    );

    update_graph_node_ids( new_graph, id, new_id );

    change_graph_and_local_save( new_graph );
}

export function make_node_first( base_path : string )
{
    if( ! base_path?.trim() || base_path == "$.first" ) return;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    let first = new_graph.first;
    new_graph.first = result[ 0 ].parent[ result[ 0 ].parentProperty ];

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = first;

    change_graph_and_local_save( new_graph );
}

export function swap_array_items( base_path : string, from : number, to : number )
{
    if( from < 0 || to < 0 || from == to ) return;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    if( ! result[ 0 ]?.value?.length || from >= result[ 0 ]?.value?.length || to >= result[ 0 ]?.value?.length ) { return; }

    let item_from = result[ 0 ].parent[ result[ 0 ].parentProperty ][ to ];
    let item_to = result[ 0 ].parent[ result[ 0 ].parentProperty ][ from ];

    result[ 0 ].parent[ result[ 0 ].parentProperty ][ to ] = item_to;
    result[ 0 ].parent[ result[ 0 ].parentProperty ][ from ] = item_from;

    // result[ 0 ].parent[ result[ 0 ].parentProperty ] = [ ...result[ 0 ].parent[ result[ 0 ].parentProperty ] ];
    
    change_graph_and_local_save( new_graph );
}

let LAST_TIME : number | undefined = undefined;

function check_time() : boolean
{
    if( is_empty( LAST_TIME ) )
    {
        LAST_TIME = ( new Date() ).getTime();

        return true;
    }

    let now = ( new Date() ).getTime();

    if( ( now - ( LAST_TIME as number ) ) < 1000 ) { console.log( "Last time false" ); return false; };

    LAST_TIME = now;

    return true;
}

export function remove_from_array( base_path : string, idx : number )
{
    if( idx < 0 || ! check_time() ) { return; }

    if( ! confirm( "This operation cannot be undone" ) )
    {
        return;
    }
    
    LAST_TIME = ( new Date() ).getTime();

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    if( ! result[ 0 ]?.value?.length || idx >= result[ 0 ]?.value?.length ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ].splice( idx, 1 );

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = [ ...result[ 0 ].parent[ result[ 0 ].parentProperty ] ];

    change_graph_and_local_save( new_graph );
}

export function append_to_array( base_path : string, new_elem : any )
{
    if( ! check_time() ) return;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ].push( new_elem );

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = [ ...result[ 0 ].parent[ result[ 0 ].parentProperty ] ];

    change_graph_and_local_save( new_graph );
}

export function change_map_value( base_path : string, next : string )
{
    if( ! next?.trim() || ! next.trim().startsWith( "{" ) || ! next.trim().endsWith( "}" ) ) { return; }

    let new_next : Map<string, any> = new Map();

    try
    {
        new_next = JSON.parse( next );
    }
    catch( e )
    {
        return;
    }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_next;

    change_graph_and_local_save( new_graph );
}

export function change_request_method( base_path : string, next : string | undefined )
{
    if( ! next?.trim() || ! is_type_in_enum( AwpakMethod, next ) ) { next = undefined; };

    let new_method = ! next ? undefined : next as AwpakMethod;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_method;

    change_graph_and_local_save( new_graph );
}

export function change_data_merge( base_path : string, next : string | undefined )
{
    if( ! next?.trim() || ! is_type_in_enum( DataMerge, next ) ) { next = undefined; };

    let new_merge = ! next ? undefined : next as DataMerge;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_merge;

    change_graph_and_local_save( new_graph );
}

export function change_data_type( base_path : string, next : string | undefined )
{
    if( ! next?.trim() || ! is_type_in_enum( DataType, next ) ) { next = undefined; };

    let new_type = ! next ? undefined : next as DataType;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_type;

    change_graph_and_local_save( new_graph );
}

export function change_boolean( base_path : string, next : boolean )
{
    if( ! next ) { next = false; }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = next;

    change_graph_and_local_save( new_graph );
}

export function change_option_number( base_path : string, next : number | undefined )
{
    if( 
        typeof( next ) === "undefined" || 
        next === null ||
        ( typeof( next ) === "string" && ( next + "" ).trim() == "" )
    ) { next = undefined; }

    if( next !== undefined )
    {
        next = Number( next );

        if( typeof( next ) === "undefined" || next === null || isNaN( next ) ) return;
    }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = next;

    change_graph_and_local_save( new_graph );
}

export function change_option_string( base_path : string, next : string )
{
    if( is_empty( next ) ) { next = ""; }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { console.log( "No lo encuentra." ); return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = next;

    change_graph_and_local_save( new_graph );
}

export function change_variant( 
    base_path : string, 
    next_variant : string, 
    new_variant : ( old : any, n : string ) => any
)
{
    if( ! next_variant?.trim() ) { return; }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    let new_obj = new_variant( result[ 0 ].value, next_variant );

    if( ! new_obj ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_obj;

    change_graph_and_local_save( new_graph );
}

export function change_provider_variant( base_path : string, next_variant : AIAgentProviderConfigVariant )
{
    change_variant( base_path, next_variant, new_agent_provider_variant );
}

export function change_parallel_executor_variant( base_path : string, next_variant : ParallelExecutorVariant )
{
    change_variant( base_path, next_variant, new_parallel_executor_variant );
}

export function change_node_executor_variant( base_path : string, next_variant : NodeExecutorVariant )
{
    change_variant( base_path, next_variant, new_node_executor_variant );
}

export function change_web_client_node_output( base_path : string, next_variant : WebClientOutputVariant )
{
    change_variant( base_path, next_variant, new_web_client_output_variant );
}

export function change_command_node_output( base_path : string, next_variant : CommandOutputVariant )
{
    change_variant( base_path, next_variant, new_command_node_output_variant );
}

export function change_graph_node_output( base_path : string, next_variant : GraphNodeOutputVariant )
{
    change_variant( base_path, next_variant, new_graph_node_output_variant );
}

export function change_data_operation( base_path : string, next_variant : DataOperationVariant )
{
    change_variant( base_path, next_variant, new_data_operation_variant );
}

export function chage_data_comparator( base_path : string, next_variant : DataComparatorVariant )
{
    change_variant( base_path, next_variant, new_data_comparator_variant );
}

export function chage_data_body_variant( base_path : string, next_variant : WebClientBodyVariant | string )
{
    if( ! next_variant?.trim() ) 
    {
        let new_graph = Object.assign( {}, graph.get() );

        const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

        if( ! result?.length || ! result[ 0 ].parent ) { return; }

        result[ 0 ].parent[ result[ 0 ].parentProperty ] = undefined;

        change_graph_and_local_save( new_graph );

        return; 
    }

    change_variant( base_path, next_variant, new_body_variant );
}

export function change_data_from_agent_history_content( base_path : string, next_variant : FromAgentHistoryContentVariant )
{
    change_variant( base_path, next_variant, new_data_from_agent_history_content );
}

export function change_data_to_agent_history( base_path : string, next_variant : DataToAgentHistoryVariant )
{
    change_variant( base_path, next_variant, new_data_to_agent_history );
}

export function chage_data_from_variant( base_path : string, next_variant : DataFromVariant )
{
    change_variant( base_path, next_variant, new_data_from_variant );
}

export function change_next_step( base_path : string, next_step : NodeNextVariant )
{
    change_variant( base_path, next_step, change_node_next_variant );
}

export function change_node_type( node_path : string, new_type : string )
{
    change_variant( node_path, new_type, change_node_variant );
}

export function change_node_output_path( id : string, path : string )
{
    if( ! path?.trim() ) { return; }

    let new_graph = Object.assign( {}, graph.get() );

    let node = node_by_id( new_graph, id );

    if( ! node ) { return; }

    let output = node._variant == NodeTypeVariant.Node ? node.output : node.node_output;

    if( ! output )
    {
        output = new DataToContext();

        output.path = path;

        node.output = output;

        replace_node_in_graph( new_graph, node );

        change_graph_and_local_save( new_graph );

        return;
    }

    if( ! output.path?.trim() )
    {
        output.path = path;

        replace_node_in_graph( new_graph, node );

        change_graph_and_local_save( new_graph );

        return;
    }


    if( output.path == path ) { return; }

    output.path = path;

    replace_node_in_graph( new_graph, node );

    change_graph_and_local_save( new_graph );
}

export function add_node_exit_text( idx : number, id : string )
{
    let new_graph = Object.assign( {}, graph.get() );

    let node = node_by_id( new_graph, id );

    if( ! node ) { return; }

    let destinations = ( node._variant == NodeTypeVariant.Node ) ? node.destination : node.node_destination;

    if( ! destinations?.length ) { return; }

    if( idx >= destinations.length ) { return; }

    if( ! destinations[ idx ].next || destinations[ idx ].next._variant == NodeNextVariant.Node ) { return; }

    destinations[ idx ].next.value.push( new DataToString() );

    replace_node_in_graph( new_graph, node );

    change_graph_and_local_save( new_graph );
}

export function change_preserve_context( preserve : boolean )
{
    let new_graph = Object.assign( {}, graph.get() );

    new_graph.preserve_context = preserve;
    
    change_graph_and_local_save( new_graph );
}

export function change_input_data_type( input_data_type : DataType | undefined )
{
    let new_graph = Object.assign( {}, graph.get() );

    new_graph.input_type = input_data_type;

    change_graph_and_local_save( new_graph );
}

export function add_node_destination( id : string, dest_node? : string )
{
    let new_graph = Object.assign( {}, graph.get() );

    if( 
        dest_node?.trim() && 
        dest_node != ID_EXIT_OK &&
        dest_node != ID_EXIT_ERR &&
        ! node_by_id( new_graph, dest_node ) 
    ) return;

    let node = node_by_id( new_graph, id );

    if( ! node ) { return; }

    let new_destination = new NodeDestination();

    if( dest_node?.trim() ) 
    {
        if( dest_node == ID_EXIT_OK )
        {
            new_destination.next = new NodeNextExitOk();
        }
        else if( dest_node == ID_EXIT_ERR )
        {
            new_destination.next = new NodeNextExitErr();
        }
        else
        {
            new_destination.next = new NodeNextNode();

            new_destination.next.value = dest_node;
        }
    }

    if( node._variant == NodeTypeVariant.Node )
    {
        if( ! node.destination ) { node.destination = []; }        

        node.destination.push( new_destination );
    }
    else if( node._variant == NodeTypeVariant.GraphNode )
    {
        if( ! node.node_destination ) { node.node_destination = []; }        

        node.node_destination.push( new_destination );
    }
    else
    {
        return;
    }

    replace_node_in_graph( new_graph, node );

    change_graph_and_local_save( new_graph );
}

function replace_node_in_graph( graph : Graph, node : NodeType )
{
    if( graph.first?.id == node.id )
    {
        graph.first = node;
    }
    else
    {
        for( let i = 0 ; i < graph.nodes.length ; i++ )
        {
            if( graph.nodes[ i ].id == node.id )
            {
                graph.nodes[ i ] = node;
            }
        }
    }
}


