<script lang="ts">

    import { modalStore, ProgressBar } from "@skeletonlabs/skeleton";
    import type { ModalSettings, ModalComponent } from "@skeletonlabs/skeleton";

    import RecipieModal from "$lib/modals/RecipieModal.svelte";
    import DataList from "$lib/components/DataList.svelte";

    const customModal: ModalComponent = {
        ref: RecipieModal,
        props: {},
    };
    const recipieModal: ModalSettings = {type: "component", component: customModal};

</script>

<button class="btn" on:click={() => modalStore.trigger(recipieModal)}>
    Open
</button>

