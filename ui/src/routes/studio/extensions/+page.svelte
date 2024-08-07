<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { getExtensions } from "$lib/extensions/loader";
    import Sidebar from "$lib/components/studio/Sidebar.svelte";
    import PageContainer from "$lib/components/studio/PageContainer.svelte";
    import type { ExtensionInfo } from "$lib/extensions/types";

    let extensions: ExtensionInfo[] = [];

    const onClickPrimary = (extension: ExtensionInfo) => {
        extension.state.is_enabled = !extension.state.is_enabled;

        extensions = extensions;

        invoke("update_extensions_state", {
            states: new Map(extensions.map((extension) => [extension.name, extension.state])),
        });
    };

    onMount(async () => {
        extensions = await getExtensions();
    });
</script>

<main class="container">
    <Sidebar />
    <PageContainer title="Extensions">
        <div class="extensions-wrapper">
            <div class="extensions-container">
                {#each extensions as extension}
                    <div class="extension-card">
                        <div class="extension-top">
                            <i
                                class="extension-icon fa-solid fa-pen-to-square"
                            />
                            <p>{extension.name}</p>
                        </div>
                        <div class="extension-bottom">
                            <button 
                                class="button button-primary"
                                on:click={() => onClickPrimary(extension)}
                            >
                                { extension.state.is_enabled ? "Disable" : "Enable" }
                            </button>
                            <button class="button button-secondary">
                                <i class="fa-solid fa-gear" />
                            </button>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    </PageContainer>
</main>

<style>
    .container {
        display: flex;
        flex-direction: row;
    }

    .extensions-wrapper {
        width: 100%;
        height: calc(100% - 10px - 10%);
        overflow-y: scroll;
    }

    .extensions-container {
        position: relative;
        padding: 10px;
        display: flex;
        flex-wrap: wrap;
    }

    .extension-card {
        box-sizing: border-box;
        min-width: calc(25% - 10px);
        height: 10rem;
        position: relative;
        display: flex;
        flex-direction: column;
        border: solid 2px var(--primary-color);
        border-radius: 10px;
        margin: 0 10px 10px 0;
    }

    .extension-top {
        width: 100%;
        height: 75%;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        color: white;
    }

    .extension-icon {
        font-size: 40px;
        margin-top: 20px;
    }

    .extension-bottom {
        width: 100%;
        height: 25%;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .button {
        width: 100%;
        height: 100%;
        background-color: transparent;
        color: white;
        border: solid 2px var(--primary-color);
        cursor: pointer;
        transition: all 0.4s ease;
    }

    .button-primary {
        width: 240%;
        border-radius: 0 0 0 5px;
    }

    .button-primary:hover {
        background-color: var(--primary-color);
        color: white;
    }

    .button-secondary {
        border-radius: 0 0 5px 0;
    }

    .button-secondary i {
        transition: all 0.7s ease;
    }

    .button-secondary:hover {
        background-color: var(--secondary-color);
        color: white;
    }

    .button-secondary:hover i {
        transform: rotate(180deg);
    }
</style>
