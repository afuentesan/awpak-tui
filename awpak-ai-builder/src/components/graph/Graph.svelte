
<script lang="ts">
    import { generate_json } from "../../functions/json_generator";
    import type { NodeType } from "../../model/node";
    import { add_node, graph, remove_from_array } from "../../store";
    import Button from "../form/Button.svelte";
    import TextArea from "../form/TextArea.svelte";
    import Card from "../util/Card.svelte";
    import GraphConfig from "./GraphConfig.svelte";
    import Node from "./Node.svelte";

    let last_json = $state( "" );
</script>

<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 p-2">
    <div>
        <Card title="Graph config">
            <GraphConfig />
        </Card>
    </div>
    <div>
        <Node node={$graph.first as NodeType} base_path="$.first" label="First node" />
    </div>
    {#each $graph.nodes as _, i}
        <div>
            <Node 
                node={$graph.nodes[i]} 
                base_path={"$.nodes["+i+"]"} 
                label={$graph.nodes[i]._variant+" "+$graph.nodes[i].id}
                remove_from_loop={
                    () => remove_from_array( "$.nodes", i )
                }
            />
        </div>
    {/each}
</div>
<div class="w-full p-4 text-center border-t-2 border-b-2 shadow-sm border-gray-200 dark:border-gray-700">
    <Button text="Generate JSON" click={() => { last_json = generate_json( $graph ) } } color="blue" />
    <Button text="+ Add node" click={() => { add_node(); } } />
    <div>
        <TextArea 
            base_path={undefined} 
            change_value={undefined}
            label={undefined}
            value={last_json}
        />
    </div>
</div>