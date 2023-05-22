<script lang="ts">

    import Textfield from '@smui/textfield';
    import Button from '@smui/button';
    import { Title, Content, Header } from '@smui/dialog';
    import { onDestroy } from 'svelte';

    export let edit: boolean = false;

    let name: string = '';

    let textfieldOptions = {
        style: 'width: 100%;',
        helperLine$style: 'width: 100%;',
    }

</script>

<div class="form-wrapper">
    <Header>
        <Title id="simple-title">
            {#if edit}
                Edit Recipie
            {:else}
                Add Recipie
            {/if}
        </Title>
    </Header>
    
    <Content id="simple-content">
        <Textfield bind:value={name} label="Name" {...textfieldOptions} />
    </Content>
</div>

<style>

</style>