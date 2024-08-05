<script lang="ts">
    export let title: string;
</script>

<main id="page-container" class="page-container">
    <div class="page-top">
        <h1 class="page-title">{title}</h1>
    </div>
    <slot />
</main>

<style>
    .page-container {
        width: 80%;
        height: 100vh;
        background-color: var(--background-color);
        position: relative;
    }

    .page-title {
        margin: 20px;
        color: white;
        position: relative; 
        display: inline-block;
    }

    .page-title::before {
        content: "";
        position: absolute;
        top: 110%;
        left: calc(50% - 30%);
        width: 60%;
        height: 10%;
        background-color: white;
        border-radius: 10px;
    }

    .page-top {
        width: 100%;
        height: 10%;
    }
</style>
