
<script lang="ts">
    import { DataOperationVariant } from "../../model/data";
    import DataFrom from "./DataFrom.svelte";

    let { operation } = $props();
</script>

{#if operation._variant == DataOperationVariant.Len}
<DataFrom from={operation.value} />
{/if}

{#if operation._variant == DataOperationVariant.Add || operation._variant == DataOperationVariant.Substract}
<DataFrom from={operation.num_1} />
<DataFrom from={operation.num_1} />
{/if}

