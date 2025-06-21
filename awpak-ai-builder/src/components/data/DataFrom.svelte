
<script lang="ts">
    import { DataFromVariant } from "../../model/data";
    import Checkbox from "../form/Checkbox.svelte";
    import Input from "../form/Input.svelte";
    import DataOperation from "./DataOperation.svelte";

let { from } = $props();
</script>

{#if 
    from._variant == DataFromVariant.Context || 
    from._variant == DataFromVariant.ParsedInput
}
<Input label="Path" value={from.path} change_value={undefined} />
{/if}

{#if from._variant == DataFromVariant.Static}
<Input label="Static" value={from.value} change_value={undefined} />
{/if}

{#if from._variant == DataFromVariant.Operation}
<DataOperation operation={from.value} />
{/if}

{#if from._variant == DataFromVariant.Concat}
<p>TODO: Concat</p>
{/if}

{#if 
    from._variant == DataFromVariant.Context || 
    from._variant == DataFromVariant.ParsedInput ||
    from._variant == DataFromVariant.Input
}
<Checkbox label="Required" value="true" checked={from.required} change_value={undefined} />
{/if}