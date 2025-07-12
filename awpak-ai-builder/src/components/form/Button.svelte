
<!-- svelte-ignore non_reactive_update -->
<script lang="ts">
    import { btn_classes } from "../../functions/form_utils";


    interface InputProps
    {
        text : string,
        click : ( event? : any ) => void,
        color? : string
    }

    let { text, click, color } : InputProps = $props();

    // if( ! color ) color = "green";

    // let btn_cls = "focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 mt-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800";

    // if( color == "red" )
    // {
    //     btn_cls = "focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900";
    // }
    // else if( color == "blue" )
    // {
    //     btn_cls = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800";
    // }

    // let btn_cls = btn_classes( color );

</script>

<button 
    type="button" 
    class={"mt-2 "+btn_classes( color )}
    onclick={click}
>{text}</button>