import { atom } from 'nanostores';
import { Graph } from './model/graph';
import { Node, type NodeType } from './model/node';
import type { DataType } from './model/data';
import { change_node_variant, node_by_id } from './functions/node_functions';

let g = new Graph();

g.context = new Map();
g.preserve_context = false;

g.first = new Node( "first_node" );

export const graph = atom( g );

export function change_node_id( id : string, new_id : string )
{
    if( ! new_id?.trim() || ! id?.trim() || id.trim() == new_id.trim() ) { return; }

    let new_graph = Object.assign( {}, graph.get() );

    if( new_graph.first?.id == id )
    {
        new_graph.first.id = new_id;

        graph.set( new_graph );

        return;
    }

    let node = new_graph.nodes.get( id );

    if( ! node ) { return; }

    new_graph.nodes.delete( id );

    node.id = new_id;

    new_graph.nodes.set( new_id, node );

    graph.set( new_graph );
}

export function change_node_type( id : string, new_type : string )
{
    if( ! new_type?.trim() || ! id?.trim() ) { return; }

    let new_graph = Object.assign( {}, graph.get() );

    let node = node_by_id( new_graph, id );

    if( ! node || node._variant == new_type ) { return; }

    let new_node = change_node_variant( node, new_type );

    if( ! new_node ) { return; }

    if( new_graph.first?.id == id )
    {
        new_graph.first = new_node;

        graph.set( new_graph );

        return;
    }

    new_graph.nodes.set( id, new_node );

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


