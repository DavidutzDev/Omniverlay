<script lang="ts">
    import { onMount } from "svelte";
    import { writable, get } from "svelte/store";
    import { getExtensions, loadExtension } from "$lib/extensions/loader";
    import { emit, listen } from "@tauri-apps/api/event";
    import type { ExtensionInfo } from "$lib/extensions/types";
    import { invoke } from "@tauri-apps/api";
    import appBanner from "$lib/assets/images/app-banner.png";

    let componentsStore = writable(new Map<string, any>());

    onMount(async () => {
        const time = Date.now();

        invoke("bootstrap_backend").then(() => {
            console.log(
                "Backend bootstrapped in " + (Date.now() - time) + "ms",
            );

            window.location.href = "/overlay";
        }).catch((err) => console.error(err));;

        await listen("test", async (event) => {
            console.log("Event " + event.payload);
        });
    })
</script>

<main class="container">
    <div class="loader-container">
        <img src={appBanner} alt="Omniverlay Banner" class="banner" />
        <div class="load-2">
            <div class="line"></div>
            <div class="line"></div>
            <div class="line"></div>
        </div>
    </div>
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
        opacity: 0.9;
    }

    .loader-container {
        width: 25%;
        background-color: var(--background-color);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        border: solid 2px var(--primary-color);
        border-radius: 10px;
    }

    .banner {
        width: 100%;
        height: 100%;
        object-fit: contain;
        border-radius: 10px 10px 0 0;
    }

    .line {
        display: inline-block;
        width: 15px;
        height: 15px;
        border-radius: 15px;
        background-color: var(--primary-color);
    }

    .load-2 {
        margin: 10px;
    }

    .load-2 .line:nth-last-child(1) {
        animation: loadingB 1.5s 1s infinite;
    }
    .load-2 .line:nth-last-child(2) {
        animation: loadingB 1.5s 0.5s infinite;
    }
    .load-2 .line:nth-last-child(3) {
        animation: loadingB 1.5s 0s infinite;
    }

    @keyframes loadingB {
        0% {
            width: 15px;
        }
        50% {
            width: 35px;
        }
        100% {
            width: 15px;
        }
    }
</style>
