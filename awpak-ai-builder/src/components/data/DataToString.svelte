
<script lang="ts">
    import Input from "../form/Input.svelte";
    import DataFrom from "./DataFrom.svelte";

    let { data } = $props();
</script>

<Input label="Prefix" value={data.prefix} change_value={undefined} />
<DataFrom from={data.from} />
<Input label="Suffix" value={data.suffix} change_value={undefined} />