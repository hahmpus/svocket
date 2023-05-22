<script lang="ts">
    import { stagger } from "../../utils/utils";
    import { postRequests }   from "../../utils/store";

    //props
    export let url:string;
    export let filters:any;
    
    //get initial values and request data, then wait for it
    import reqwest from "../../utils/request";
    const [data, fetching, error, get] = reqwest('GET', url, filters);
  
    let urlRequests = postRequests.to(url);

    //subscribe to filters and posts to this url
    $: {    
        $urlRequests;
        stagger(() => get(filters), 'DataList_'+url, 300);
    }

</script>

<div class="data-list-wrapper">
    {#if $fetching}
	    fetching...
    {:else if $error}
        Error: {$error}
    {:else}
        {#each $data as item}
            <slot name="item" {item}></slot>
        {/each} 
    {/if}
</div>

<style>
</style>