<script lang="ts">

	import { fly } from 'svelte/transition';
    import type { CssClasses } from '@skeletonlabs/skeleton';

    type ProgressData = {
        value: number;
        color: CssClasses;
        label?: string;
    }

    //props
    export let pieces: ProgressData[];
    export let labels: boolean = false;
    export let max: number = 0;

    //if max is not set, calculate it
    if(max === 0) {
        for(let item of pieces) {
            max += item.value;
        }
    }
    
    //classes
    const classesWrapper = 'h-5 w-full overflow-hidden bg-surface-200-700-token rounded-token';
    const classesPieces = 'flex items-center justify-center h-full';
    
</script>


<div class="progress-bar {classesWrapper}" aria-valuemin={0} aria-valuemax={max - 0}>
    {#each pieces as piece, index}
        <div
            in:fly={{x: 10, duration: 75, delay: 100 * index}}
            class="progress-bar-meter {classesPieces} {piece.color}" 
            style:width="{piece.value}%" 
        >
            {#if labels && piece.label}
                <span class="text-xs text-surface-900-50-token">{piece.label}</span>
            {/if}
        </div>
    {/each}
</div>

<style>
    .progress-bar {
        display: flex;
    }
</style>
