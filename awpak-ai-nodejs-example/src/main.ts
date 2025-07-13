import { run_graph, run_graph_once } from "awpak-ai-nodejs";

(
    async () =>
    {
        let result_0 = await run_graph_once(
            "/home/angel/proyectos/rust-apps/awpak-tui/awpak-ai/test_data/graphs/ollama_chat.json", 
            "7+7"
        );

        let result_1 = await run_graph( 
            "g",
            "/home/angel/proyectos/rust-apps/awpak-tui/awpak-ai/test_data/graphs/ollama_chat.json", 
            "2+2" 
        );

        let result_2 = await run_graph( 
            "g",
            "/home/angel/proyectos/rust-apps/awpak-tui/awpak-ai/test_data/graphs/ollama_chat.json", 
            "a eso sumale 5" 
        );

        console.log( result_0, "\n\n", result_1, "\n\n", result_2 );
    }
)();



