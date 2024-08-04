import type { ExtensionInfo } from "$lib/extensions/types";
import { invoke } from "@tauri-apps/api";
import type { SvelteComponent } from "svelte";

export const getExtensions = async (): Promise<ExtensionInfo[]> => {
    return await invoke("list_extensions");
}

export const loadExtension = async (extension: string): Promise<SvelteComponent<any> | null> => {
    try {
        const module = await import(`$lib/extensions/components/${extension}.svelte`);
        return module.default;
    } catch (error) {
        console.error(`Error loading ${extension}:`, error);
        return null; // Return null for failed imports
    }
}

export const loadExtensions = async (extensions: string[]) =>{
    const loadComponents = extensions.map(async (extension) => {
        try {
            const module = await import(`$lib/extensions/components/${extension}.svelte`);
            return module.default;
        } catch (error) {
            console.error(`Error loading ${extension}:`, error);
            return null; // Return null for failed imports
        }
    });

    // Wait for all components to be loaded\s
    return (await Promise.all(loadComponents)).filter(Boolean);
}