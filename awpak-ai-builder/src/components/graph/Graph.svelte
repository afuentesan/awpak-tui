
<script lang="ts">
    import type { NodeType } from "../../model/node";
    import { graph } from "../../store";
    import Card from "../util/Card.svelte";
    import GraphConfig from "./GraphConfig.svelte";
    import Node from "./Node.svelte";
</script>

<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
    <div>
        <Card title="Graph config">
            <GraphConfig />
        </Card>
    </div>
    <div>
        <Node node={$graph.first as NodeType} base_path="$.first" />
    </div>
    {#each $graph.nodes as _, i}
        <div>
            <Node 
                node={$graph.nodes[i]} 
                base_path={"$.nodes["+i+"]"}
            />
        </div>
    {/each}
</div>