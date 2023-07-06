<script lang="ts">

    //import '@skeletonlabs/skeleton/themes/theme-seafoam.css';
    import "../yellow.postcss"
    import '@skeletonlabs/skeleton/styles/skeleton.css';
    import "../app.css";

    import AnimatedRoute from "$lib/components/AnimatedRoute.svelte";
    import { AppBar, AppShell, LightSwitch, Modal } from "@skeletonlabs/skeleton";
    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte";

    export let data: any;
    
</script>


<div class="max-w-1140">
    
    <Modal 
        flyY={10}
        duration={100}
    />

    <AppShell>
    
        <AppBar 
            slot="header"
            background="bg-primary-100"        
        >
            <a href="/" class="btn variant-filled-primary">
                home
            </a>
            <a href="/recipie" class="btn variant-filled-secondary">
                recipie
            </a>

            <LightSwitch />

        </AppBar>

        <!-- (sidebarLeft) -->
        <!-- (sidebarRight) -->
        <!-- (pageHeader) -->

        <svelte.component>
            <Breadcrumbs path={data.pathname}/>

            <AnimatedRoute key={data.pathname}>
                <slot />
            </AnimatedRoute>
        </svelte.component>

        <!-- (footer) -->
    </AppShell>



</div>

<style>
    .max-w-1140 {
        max-width: 1140px;
        margin: 0 auto;
    }
</style>