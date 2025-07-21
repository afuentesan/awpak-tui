
import type { Graph } from "../model/graph";
import { generate_json } from "./json_generator";

export function save_graph_to_file( graph : Graph )
{
    let json = generate_json( graph );

    const a = document.createElement("a");

    a.href = URL.createObjectURL(new Blob([json], {
        type: "application/json"
    }));

    a.setAttribute( "download", "awpak_graph.json" );
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
}