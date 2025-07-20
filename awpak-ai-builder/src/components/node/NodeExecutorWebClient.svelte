
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { AwpakMethod, WebClientNameValue, WebClientOutputStatus, type WebClient } from "../../model/web_client";
    import { append_to_array, change_request_method, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import NameValue from "../data/NameValue.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Select from "../form/Select.svelte";
    import WebClientBody from "./WebClientBody.svelte";
    import WebClientOutput from "./WebClientOutput.svelte";


    interface InputProps
    {
        web_client : WebClient,
        base_path : string
    }

    let { web_client, base_path } : InputProps = $props();

</script>

<DataFrom
    from={web_client.url}
    label="URL"
    base_path={base_path+".url"}
/>
<Select
    label="Method"
    base_path={base_path+".method"}
    change_value={change_request_method}
    options={
        select_options_from_enum(
            AwpakMethod,
            web_client.method,
            false
        )
    }
    value={web_client.method}
/>

<Box title="Headers" base_path={base_path+".headers"}>
    {#each web_client.headers as _, i}
        <NameValue
            label={"Header "+i}
            name_value={web_client.headers[i]}
            base_path={base_path+".headers["+i+"]"}
            remove_from_loop={
                () => remove_from_array( base_path+".headers", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".headers", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button
            text="Add header"
            click={
                () => append_to_array( base_path+".headers", new WebClientNameValue() )
            }
        />
    </div>
</Box>

<Box title="Query params" base_path={base_path+".query_params"}>
    {#each web_client.query_params as _, i}
        <NameValue
            label={"Param "+i}
            name_value={web_client.query_params[i]}
            base_path={base_path+".query_params["+i+"]"}
            remove_from_loop={
                () => remove_from_array( base_path+".query_params", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".query_params", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button
            text="Add query param"
            click={
                () => append_to_array( base_path+".query_params", new WebClientNameValue() )
            }
        />
    </div>
</Box>

<WebClientBody
    label="Body"
    base_path={base_path+".body"}
    body={web_client.body}
/>

<Box title="WebClient output" base_path={base_path+".output"}>
    {#each web_client.output as _, i}
        <WebClientOutput
            label={"WebClientOutput "+i}
            output={web_client.output[i]}
            base_path={base_path+".output["+i+"]"}
            remove_from_loop={
                () => remove_from_array( base_path+".output", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".output", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button
            text="Add output"
            click={
                () => append_to_array( base_path+".output", new WebClientOutputStatus() )
            }
        />
    </div>
</Box>