
<script lang="ts">
    import DataList from "$lib/components/DataList.svelte";
    import RecipieForm from "$lib/forms/RecipieForm.svelte";
    import reqwest from "../../utils/request";

    import Dialog, { Title, Content, Actions } from '@smui/dialog';
    import Button, { Label } from '@smui/button';

    let filters:any = {};
    let addFormOpen = false;

    

</script>

<Dialog 
    bind:open={addFormOpen} 
    fullscreen 
    aria-labelledby="simple-title" 
    aria-describedby="simple-content"
>
    {#if addFormOpen}
        <RecipieForm />
    {/if}
</Dialog>

<div>

    <Button on:click={() => addFormOpen = true}>
        <Label>Add</Label>
    </Button>

    <hr />
    


    <DataList url={"/api/recipie"} {filters}>
        <div class="recipies" slot="item" let:item={recipie}>
            <h2>{recipie.id}</h2>
            <button on:click={() => reqwest('DELETE', `/api/recipie/${recipie.id}`, {})}>DELETE</button>
        </div>
    </DataList>
</div>

<style>

</style>