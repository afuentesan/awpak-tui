
<script lang="ts">
    import { btn_classes } from "../../functions/form_utils";


    interface InputProps
    {
        id : string,
        text : string,
        change : ( event? : any ) => void,
        color? : string
    }

    let { id, change, color, text } : InputProps = $props();

    let btn_cls = btn_classes( color );

</script>

<label 
    for={id}
    class={"mt-2 "+btn_cls}
>
    {text}
</label>

<input 
    type="file" 
    style="display:none;" 
    id={id}
    onchange={change}
/>