<script lang="ts">
    import { onMount } from "svelte";
    import { writable, get } from "svelte/store";
    import { emit, listen } from "@tauri-apps/api/event";
    import { getExtensions, loadExtension } from "$lib/extensions/loader";
    import type { ExtensionInfo } from "$lib/extensions/types";
    import type { SvelteComponent } from "svelte";
    import ExtensionContainer from "$lib/extensions/ExtensionContainer.svelte";

    let componentsStore = writable(
        new Map<ExtensionInfo, SvelteComponent<any>>(),
    );
    // Convert map to an array for easier rendering in the template
    $: componentsArray = Array.from($componentsStore.entries());

    onMount(async () => {
        console.log(window.innerWidth);
        console.log(window.innerHeight);

        await listen("refresh-extensions", async () => {
            const infos: ExtensionInfo[] = await getExtensions();

            const storeValue = get(componentsStore);

            const loadPromises = infos
                .filter((info) => info.is_enabled && !storeValue.has(info))
                .map(async (info) => {
                    console.log(`Loading ${info.name}...`);
                    const component = await loadExtension(info.name);
                    storeValue.set(info, component);
                });

            await Promise.all(loadPromises);

            infos
                .filter((info) => !info.is_enabled && storeValue.has(info))
                .forEach((info) => {
                    storeValue.delete(info);
                });

            componentsStore.set(storeValue);
        });

        await listen("Omniverlay://refresh_extensions_data", async (event) => {
            const extensionsInfos: ExtensionInfo[] = await getExtensions();

            const storeValue = get(componentsStore);

            storeValue.clear();

            componentsStore.set(storeValue);

            const loadPromises = extensionsInfos
                .filter((info) => info.is_enabled && !storeValue.has(info))
                .map(async (info) => {
                    const component = await loadExtension(info.name);

                    if (!component) return; //TODO:ERROR LOADING

                    storeValue.set(info, component);
                });

            await Promise.all(loadPromises);

            componentsStore.set(storeValue);
        });

        emit("refresh-extensions");

        const div = document.querySelector(".container");
        console.log(div?.getBoundingClientRect());
    });
</script>

<main class="container">
    <h1>Extensions</h1>

    {#each componentsArray as [info, component]}
        <ExtensionContainer {info} child={component} />
    {/each}

    <h1>Extension Container</h1>
</main>

<style>
    .container {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        margin: 0;
        padding: 0;

        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        align-items: flex-end;
        color: green;
    }
</style>
