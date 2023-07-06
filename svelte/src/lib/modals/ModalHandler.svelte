<script lang="ts">
    import { Modal, type ModalComponent } from "@skeletonlabs/skeleton";

    //own modals
    import RecipieModal from "./RecipieModal.svelte";

    const myModals: Record<string, ModalComponent> = {
        RecipieModal: {
            ref: RecipieModal,
            props: {
                test: "test",
            },
            slot: '<p>content</p>'
        }
    };

</script>

<Modal 
    components={myModals} 
    flyY={10}
    duration={100}    
/>