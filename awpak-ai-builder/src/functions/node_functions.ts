import { CommandOutputCode, CommandOutputErr, CommandOutputOut, CommandOutputSuccess, CommandOutputVariant, type CommandOutput } from "../model/command";
import type { Graph } from "../model/graph";
import { GraphNode, GraphNodeOutputErr, GraphNodeOutputOut, GraphNodeOutputVariant, Node, NodeDestination, NodeNextExitErr, NodeNextExitOk, NodeNextNode, NodeNextVariant, NodeTypeVariant, type GraphNodeOutput, type NodeNext, type NodeType } from "../model/node";
import { NodeExecutorCommand, NodeExecutorContextMut, NodeExecutorVariant, type NodeExecutor } from "../model/node_executor";
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
}

export function new_graph_node_output_variant( old : GraphNodeOutput, new_variant : string ) : GraphNodeOutput | undefined
{
    if( ! is_type_in_enum( GraphNodeOutputVariant, new_variant ) ) { return undefined; }
    
    new_variant = new_variant as GraphNodeOutputVariant;

    if( old._variant == new_variant ) { return old; }

    let new_output = ( 
        new_variant == GraphNodeOutputVariant.Err 
    ) 
    ? 
    new GraphNodeOutputErr() 
    :
    (
        ( new_variant == GraphNodeOutputVariant.Out )
        ?
        new GraphNodeOutputOut()
        :
        undefined
    );

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