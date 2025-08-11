import type cytoscape from "cytoscape";
import type { Graph } from "../model/graph";
import { NodeConfig, NodeNextVariant, type NodeDestination } from "../model/node";
import { DataComparatorVariant, type DataComparator } from "../model/data_comparator";
import { DataFromVariant, type DataFrom } from "../model/data";
import { NodeExecutorVariant, type NodeExecutor } from "../model/node_executor";

export const ID_EXIT_OK = "__ExitOk";
export const ID_EXIT_ERR = "__ExitErr";

const CLS_INFO_BODY_HEADER = ` class="text-center font-bold p-1" `

export function graph_to_cytoscape( graph : Graph, positions? : Map<string, cytoscape.Position> ) : [Array<any>, boolean]
{
    let [ nodes, all_has_position ] = graph_nodes( graph, positions );

    return [[ ...nodes, ...graph_edges( graph ) ], all_has_position]
}

function graph_nodes( graph : Graph, positions? : Map<string, cytoscape.Position> ) : [Array<any>, boolean]
{
    if( ! graph.first?.id ) { return [[], false] };

    let ret : Array<any> = [];

    let first_node = graph_node( graph.first, positions?.get( graph.first.id ), true );

    let all_has_position = first_node.position ? true : false;

    ret.push( first_node );

    graph.nodes.forEach( 
        ( n ) => 
        {
            let node = graph_node( n, positions?.get( n.id as string ) );

            if( ! node.position )
            {
                all_has_position = false;
            }

            ret.push( node ) 
        }
    );

    ret.push( ...exit_nodes( positions ) );

    return [ ret, all_has_position ];
}

function exit_nodes( positions? : Map<string, cytoscape.Position> ) : Array<any>
{
    return [ 
        { 
            data : { 
                id : ID_EXIT_OK, 
                label : "Exit Ok",
                ty : ID_EXIT_OK
            },
            position : clone_position( positions?.get( ID_EXIT_OK ) )
        },
        { 
            data : { 
                id : ID_EXIT_ERR, 
                label : "Exit Err",
                ty : ID_EXIT_ERR
            },
            position : clone_position( positions?.get( ID_EXIT_ERR ) )
        }
    ]
}

function clone_position( position : cytoscape.Position | undefined )
{
    if( ! position?.x || ! position?.y ) return undefined;

    return {
        x : position.x,
        y : position.y
    }
}

function graph_node( node : NodeConfig, position? : cytoscape.Position | undefined, is_first? : boolean ) : any
{
    return { 
        data : { 
            id : node.id, 
            label : label_node( node ),
            ty : graph_node_ty( node ),
            first : is_first ? "first" : "",
            info : graph_node_info( node )
        },
        position : clone_position( position )
    }
}

function graph_node_info( node : NodeConfig ) : string | undefined
{
    if( ! node.executor ) return undefined;

    return `<h4 class="font-bold text-center mb-2 bg-gray-200 dark:bg-indigo-700 pb-1 pt-1 pl-2 pr-2">${node.executor?._variant}</h4>
    ${node_executor_info( node.executor )}
`;
}

function node_executor_info( executor : NodeExecutor ) : string
{
    if( executor._variant == NodeExecutorVariant.Command )
    {
        return `${info_data_from( executor.value.command)}
        <p ${CLS_INFO_BODY_HEADER}>Args</p>
        ${
            executor.value.args.reduce(
                ( prev, curr ) =>
                {
                    return prev + info_data_from( curr )
                },
                ""
            )
        }
        `;
    }

    return "";
}

function graph_node_ty( node : NodeConfig ) : string
{
    if( node.executor ) return node.executor._variant;

    return "Node";
}

function label_node( node : NodeConfig ) : string
{
    return node.id || "Node";
}

function graph_edges( graph : Graph ) : Array<any>
{
    if( ! graph.first ) { return [] };

    let ret : Array<any> = [];

    ret.push( ...node_edges( graph.first ) );

    graph.nodes.forEach( ( n ) => ret.push( ...node_edges( n ) ) );

    return ret;
}

function node_edges( node : NodeConfig ) : Array<any>
{
    let ret : Array<any> = [];

    node.destination.forEach(
        ( d, idx ) => ret.push( node_edge( node, d, idx ) )
    );

    return ret;
}

function node_edge( node : NodeConfig, destination : NodeDestination, idx : number ) : any
{
    let target = target_destination( destination );

    return { 
        data : { 
            id : node.id + "__" + target + "__" + idx, 
            source : node.id, 
            target : target,
            idx : ( idx + 1 ),
            info : info_edge( idx, destination )
        } 
    }
}

function info_edge( idx : number, destination : NodeDestination ) : string | undefined
{
    if( ! destination.condition ) return undefined;

    let header = `<h4 class="font-bold text-center mb-2 bg-gray-200 dark:bg-indigo-700 pb-1 pt-1 pl-2 pr-2">Out ${idx+1}</h4>`;
    let body = body_info( destination.condition );

    if( ! body?.trim() ) return undefined;

    return header + body;
}

function body_info( condition : DataComparator | undefined ) : string | undefined
{
    if( ! condition ) return undefined;

    const cls_cond = ` class="text-center font-bold p-1" `;

    if( 
        condition._variant == DataComparatorVariant.True ||
        condition._variant == DataComparatorVariant.False
    )
    {
        return `<p>${condition._variant}</p>`;
    }
    else if( 
        condition._variant == DataComparatorVariant.Eq ||
        condition._variant == DataComparatorVariant.NotEq ||
        condition._variant == DataComparatorVariant.Gt ||
        condition._variant == DataComparatorVariant.Lt
    )
    {
        return `${info_data_from(condition.from_1)}<p ${cls_cond}>${condition._variant}</p>${info_data_from(condition.from_2)}`;
    }
    else if(
        condition._variant == DataComparatorVariant.Regex
    )
    {
        return `<p>${condition._variant}: ${condition.regex}</p><p>From:</p>${info_data_from(condition.from)}`;
    }
    else if(
        condition._variant == DataComparatorVariant.And ||
        condition._variant == DataComparatorVariant.Or ||
        condition._variant == DataComparatorVariant.Xor ||
        condition._variant == DataComparatorVariant.Nand
    )
    {
        return `${body_info(condition.comp_1)}<p ${cls_cond}>${condition._variant}</p>${body_info(condition.comp_2)}`;
    }
    else if( condition._variant == DataComparatorVariant.Not )
    {
        return `<p>${condition._variant}</p>${body_info(condition.value)}`;
    }
    else if(
        condition._variant == DataComparatorVariant.Empty ||
        condition._variant == DataComparatorVariant.NotEmpty
    )
    {
        return `<p>${condition._variant}. From:</p>${info_data_from(condition.value)}`;
    }
    
    return undefined;
}

function info_data_from( data : DataFrom | undefined) : string
{
    if( ! data ) return "";

    if( 
        data._variant == DataFromVariant.Input ||
        data._variant == DataFromVariant.Null
    )
    {
        return "<p>" + data._variant + "</p>";
    }
    else if(
        data._variant == DataFromVariant.Context ||
        data._variant == DataFromVariant.ParsedInput
    )
    {
        return "<p>From " + data._variant + ". Path: " + data.path + "</p>";
    }
    else if( data._variant == DataFromVariant.Static )
    {
        return "<p>" + data._variant + ": " + data.value + "</p>";
    }
    else if( data._variant == DataFromVariant.Concat )
    {
        return `<p>${data._variant}</p>` + 
                data.value
                .reduce( 
                    ( prev, curr ) => 
                    { 
                        return prev + info_data_from( curr )
                    },
                    ""    
                );
    }
    else if( data._variant == DataFromVariant.AgentHistory )
    {
        return `<p>${data._variant}: Agent id: ${data.id}. From: ${data.content._variant}</p>`;
    }
    else if( data._variant == DataFromVariant.Operation )
    {
        return `<p>${data._variant}: ${data.value?._variant}</p>`;
    }

    return "";
}

function target_destination( destination : NodeDestination ) : string
{
    if( destination.next?._variant == NodeNextVariant.ExitOk )
    {
        return ID_EXIT_OK;
    }
    else if( destination.next?._variant == NodeNextVariant.ExitErr )
    {
        return ID_EXIT_ERR;
    }
    else if( destination.next?._variant == NodeNextVariant.Node )
    {
        return destination.next.value as string;
    }
    
    throw new Error( "Next node variant not found" );
}