<script lang="ts">
    export let title: string;
</script>

<main class="page-container">
    <h1 class="page-title">{title}</h1>
    <slot />
</main>

<style>
    .page-container {
        width: 80%;
        height: 100%;
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
</style>
