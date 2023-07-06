<script lang="ts">

    export let url: string;
    export let filters: any;

    import request from "$lib/utils/request";
    const [data, fetching, error, get] = request("GET", url, filters);

    let test: String[] = ["test", "test2", "test3", "test4"];

</script>

<div class="datalist-wrapper">
    {#if fetching}
        fetching...
    {:else if error}
        error: {error}
    {:else}
        {#each test as item}
            <slot name="item" {item} />
        {/each}
    {/if}
</div>