<script lang="ts">

//make a array with items
let items = [1,2,3,4,5,6,7,8,9,10];

</script>

{#each items as item}
    <div class="item">
        {item}
    </div>
{/each}