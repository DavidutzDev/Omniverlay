<script lang="ts">
    import { onMount } from "svelte";
    import { writable, get } from "svelte/store";
    import { getExtensions, loadExtension } from "$lib/extensions/loader";
    import { emit, listen } from "@tauri-apps/api/event";
    import type { ExtensionInfo } from "$lib/extensions/types";
    import { invoke } from "@tauri-apps/api";

    let componentsStore = writable(new Map<string, any>());

    onMount(async () => {
        const time = Date.now();

        invoke("bootstrap_backend").then(() => {
            console.log("Backend bootstrapped in " + (Date.now() - time) + "ms");

            window.location.href = "/overlay";
        });

        await listen("test", async (event) => {
            console.log("Event "  + event.payload);
        })
    });
</script>

<main class="container">
    <h1>Loading Omniverlay.....</h1>
</main>

<style>
    .container {
        width: 100%;
        height: 100vh;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: green;
    }
</style>
