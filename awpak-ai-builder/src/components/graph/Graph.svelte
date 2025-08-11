
<script lang="ts">
    import type { NodeConfig } from "../../model/node";
    import { add_store, graph } from "../../store";
    import Button from "../form/Button.svelte";
    import Card from "../util/Card.svelte";
    import GraphConfig from "./GraphConfig.svelte";
    import Node from "./Node.svelte";
    import Store from "./Store.svelte";
</script>

<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
    <div>
        <Card title="Graph config">
            <GraphConfig />
        </Card>
    </div>
    <div>
        <Card title="Stores">
            {#each $graph.stores as _, i}
                <Store
                    store={$graph.stores[i]}
                    base_path={"$.stores["+i+"]"}
                />
            {/each}
            <div class="text-center">
                <Button text="Add store" click={add_store} />
            </div>
        </Card>
    </div>
    <div>
        <Node node={$graph.first as NodeConfig} base_path="$.first" />
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