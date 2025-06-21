
<script lang="ts">
    import NodeNext from "./NodeNext.svelte";

    let { destination } = $props();
</script>

<NodeNext node_next={destination.next} change_node_destination={undefined} />