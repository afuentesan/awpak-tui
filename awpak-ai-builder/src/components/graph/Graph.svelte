
<script lang="ts">
    import type { NodeType } from "../../model/node";
    import { graph } from "../../store";
    import Card from "../util/Card.svelte";
    import GraphConfig from "./GraphConfig.svelte";
    import Node from "./Node.svelte";
</script>

<Card title="Graph config">
    <GraphConfig />
</Card>

<Card title="First node">
    <Node node={$graph.first as NodeType} />
</Card>