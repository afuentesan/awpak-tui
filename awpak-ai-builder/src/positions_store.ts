import { atom } from 'nanostores';
import cytoscape from 'cytoscape';

export const graph_positions = atom( new Map<string, cytoscape.Position>() );

export function change_graph_positions( cy : cytoscape.Core )
{
    if( ! cy ) return;

    let new_positions : Map<string, cytoscape.Position> = new Map();

    cy.nodes().forEach( n => {
            new_positions.set( n.id(), n.position() );
        }
    );

    graph_positions.set( new_positions );
}