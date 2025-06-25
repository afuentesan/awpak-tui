
<script lang="ts">
    import type { DataToString, DataFrom as DTSDataFrom } from "../../model/data";
    import { change_option_string } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import DataFrom from "./DataFrom.svelte";

    interface InputProps
    {
        data : DataToString,
        base_path : string,
        label : string,
        remove_from_loop? : () => void | undefined
    }

    let { data, base_path, label, remove_from_loop } : InputProps = $props();
</script>

<Box title={label}>

    <Input label="Prefix" value={data.prefix} change_value={change_option_string} base_path={base_path+".prefix"} />
    <DataFrom from={data.from as DTSDataFrom} base_path={base_path+".from"} label={label+" from"} />
    <Input label="Suffix" value={data.suffix} change_value={change_option_string} base_path={base_path+".suffix"} />

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove DataToString" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>