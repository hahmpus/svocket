<script lang="ts">
    import type { PageData } from "./$types";
    import DataList from "$lib/components/DataList.svelte";
    let filters: any = {};
</script>

<input bind:value={filters.name} />

<DataList url={"/api/recipie"} {filters}>
    <div class="persons" slot="item" let:item={person}>
        {console.log('person', person)}
    </div>
</DataList>

<style>
    .persons {
        border: 1px solid red;
        padding: 10px;
        margin: 10px;
    }
</style>
