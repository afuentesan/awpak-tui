import { atom } from 'nanostores';
import { Graph } from './model/graph';
import { GraphNodeOutputVariant, Node, NodeDestination, NodeNextExitErr, NodeNextExitOk, NodeNextNode, NodeNextVariant, NodeTypeVariant, type NodeNext, type NodeType } from './model/node';
import { DataFromVariant, DataMerge, DataToContext, DataToString, FromConcat, FromContext, FromInput, FromOperation, FromParsedInput, FromStatic, DataType, DataOperationVariant } from './model/data';
import { change_node_next_variant, change_node_variant, clean_node_destination_id, new_command_node_output_variant, new_graph_node_output_variant, new_node_executor_variant, node_by_id, update_node_destination_id } from './functions/node_functions';
import { JSONPath } from 'jsonpath-plus';
import { new_data_comparator_variant, new_data_from_variant, new_data_operation_variant } from './functions/data_functions';
import { is_type_in_enum, random_id } from './functions/form_utils';
import type { DataComparatorVariant } from './model/data_comparator';
import type { NodeExecutorVariant } from './model/node_executor';
import type { CommandOutputVariant } from './model/command';

let g = new Graph();

g.context = new Map();
g.preserve_context = false;

g.first = new Node( "first_node" );

export const graph = atom( g );

export function add_node()
{
    let new_graph = Object.assign( {}, graph.get() );

    let id = random_id();

    new_graph.nodes.push( new Node( id ) );

    graph.set( new_graph );
}

export function change_node_id( id : string, new_id : string )
{
    if( ! new_id?.trim() || ! id?.trim() || id.trim() == new_id.trim() ) { return; }

    let new_graph = Object.assign( {}, graph.get() );

    if( new_graph.first?.id == id )
    {
        new_graph.first.id = new_id;

        update_node_destination_id( new_graph, id, new_id );

        graph.set( new_graph );

        return;
    }

    new_graph.nodes = new_graph.nodes.map( ( n : NodeType ) => 
        {
            if( n.id == id )
            {
                n.id = new_id;
            }

            return n
        }
    );

    update_node_destination_id( new_graph, id, new_id );

    graph.set( new_graph );
}

export function remove_node( idx : number )
{
    if( idx < 0 ) { return; }

    if( ! confirm( "This operation cannot be undone" ) )
    {
        return;
    }

    let new_graph = Object.assign( {}, graph.get() );

    if( ! new_graph?.nodes?.length || idx >= new_graph?.nodes?.length ) return;

    let id = new_graph.nodes[ idx ].id;

    new_graph.nodes.splice( idx, 1 );

    clean_node_destination_id( new_graph, id as string );

    graph.set( new_graph );
}

export function remove_from_array( base_path : string, idx : number )
{
    if( idx < 0 ) { return; }

    if( ! confirm( "This operation cannot be undone" ) )
    {
        return;
    }
    
    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    if( ! result[ 0 ]?.value?.length || idx >= result[ 0 ]?.value?.length ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ].splice( idx, 1 );

    console.log( "New graph: ", new_graph );

    graph.set( new_graph );
}

export function append_to_array( base_path : string, new_elem : any )
{
    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ].push( new_elem );

    console.log( "New graph: ", new_graph );

    graph.set( new_graph );
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

    graph.set( new_graph );
}

export function change_data_merge( base_path : string, next : string | undefined )
{
    if( ! next?.trim() || ! is_type_in_enum( DataMerge, next ) ) { next = undefined; };

    let new_merge = ! next ? undefined : next as DataMerge;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_merge;

    graph.set( new_graph );
}

export function change_data_type( base_path : string, next : string | undefined )
{
    if( ! next?.trim() || ! is_type_in_enum( DataType, next ) ) { next = undefined; };

    let new_type = ! next ? undefined : next as DataType;

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = new_type;

    graph.set( new_graph );
}

export function change_boolean( base_path : string, next : boolean )
{
    if( ! next ) { next = false; }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = next;

    graph.set( new_graph );
}

export function change_option_string( base_path : string, next : string )
{
    if( ! next.trim() ) { next = ""; }

    let new_graph = Object.assign( {}, graph.get() );

    const result = JSONPath( { path : base_path, json : new_graph, resultType : "all" } );

    if( ! result?.length || ! result[ 0 ].parent ) { return; }

    result[ 0 ].parent[ result[ 0 ].parentProperty ] = next;

    graph.set( new_graph );
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

    graph.set( new_graph );
}

export function change_node_executor_variant( base_path : string, next_variant : NodeExecutorVariant )
{
    change_variant( base_path, next_variant, new_node_executor_variant );
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

        graph.set( new_graph );

        return;
    }

    if( ! output.path?.trim() )
    {
        output.path = path;

        replace_node_in_graph( new_graph, node );

        graph.set( new_graph );

        return;
    }


    if( output.path == path ) { return; }

    output.path = path;

    replace_node_in_graph( new_graph, node );

    graph.set( new_graph );
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

    graph.set( new_graph );
}

export function change_preserve_context( preserve : boolean )
{
    let new_graph = Object.assign( {}, graph.get() );

    new_graph.preserve_context = preserve;
    
    graph.set( new_graph );
}

export function change_input_data_type( input_data_type : DataType | undefined )
{
    let new_graph = Object.assign( {}, graph.get() );

    new_graph.input_type = input_data_type;

    graph.set( new_graph );
}

export function add_node_destination( id : string )
{
    let new_graph = Object.assign( {}, graph.get() );

    let node = node_by_id( new_graph, id );

    if( ! node ) { return; }

    if( node._variant == NodeTypeVariant.Node )
    {
        if( ! node.destination ) { node.destination = []; }        

        node.destination.push( new NodeDestination() );
    }
    else if( node._variant == NodeTypeVariant.GraphNode )
    {
        if( ! node.node_destination ) { node.node_destination = []; }        

        node.node_destination.push( new NodeDestination() );
    }
    else
    {
        return;
    }

    replace_node_in_graph( new_graph, node );

    graph.set( new_graph );
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


