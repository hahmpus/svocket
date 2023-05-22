
<script>
    let counter = 0;
    let clicks  = 0;

    const increase = () => () => {
        counter++;
        clicks++;
    };
    
    const decrease = () => () => {
        counter--;
        clicks++;
    };

    const reset = () => () => {
        counter = 0;
        clicks = 0;
    };


</script>

<h1>Value: {counter}</h1>


<button on:click={increase()}>Increase</button>
<button on:click={decrease()}>Decrease</button>
<button on:click={reset()}>Reset</button>

{#each Array(clicks) as click, i}
    <div><a href="/counter/{i+1}">go to {i+1}</a></div>
{:else}
    <div>no clicks</div>
{/each}