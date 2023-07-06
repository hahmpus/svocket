<script lang="ts">

    export let path: string;

    $: paths = path.split("/").filter((p) => p !== "");

</script>

<ol class="breadcrumb">
    <li class="crumb"><a class="anchor" href="/">a</a></li>
    {#each paths as path}
        <li class="crumb-separator" aria-hidden>/</li>
        <li class="crumb"><a class="anchor" href="/{path}">{path}</a></li>
    {/each}
</ol>