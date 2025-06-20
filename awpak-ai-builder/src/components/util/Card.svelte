
<script lang="ts">

    let { title, children } = $props();
</script>

<div class="w-full p-6 bg-white border border-gray-200 rounded-lg shadow-sm dark:bg-gray-800 dark:border-gray-700">

    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{title}</h5>

    {@render children?.()}
</div>