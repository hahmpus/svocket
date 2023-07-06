<script lang="ts">

    import { modalStore } from "@skeletonlabs/skeleton";

    export let test: string = "default value";

</script>

{#if $modalStore[0]}
    <div class="modal block bg-surface-100-800-token w-modal h-auto p-4 space-y-4 rounded-container-token shadow-xl">
        <h1>Recipie Modal</h1>
        <slot />
        <p>{test}</p>
        <label class="label">
            <span>Name</span>
            <input class="input" type="text" />
        </label>
    </div>
{/if}