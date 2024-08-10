<script lang="ts">
    import type {
        ConfigValue,
        ExtensionInfo,
        ConfigValueType,
        ConfigCategory,
    } from "$lib/extensions/types";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    export let extension: ExtensionInfo;
    export let onClose: () => void;

    let categories: ConfigCategory[] = [];
    let configTypes: Record<string, ConfigValueType> = {}; // To store the type of each config

    const onDiscardChanges = () => {
        onClose();
    };

    const onSaveChanges = () => {
        invoke("update_extensions_state", {
            states: new Map([[extension.name, extension.state]]),
        });

        onClose();
    };

    const handleInputChange = (
        categoryName: string,
        key: string,
        type: ConfigValueType,
        event: Event,
    ) => {
        const target = event.target as HTMLInputElement | HTMLSelectElement;
        const newValue = target.value;

        // Find the category in the extension's state config
        const category = extension.state.config.categories.find(
            (cat) => cat.name === categoryName,
        );
        if (!category) {
            console.error(`Category "${categoryName}" not found.`);
            return;
        }

        // Find the config value within the category
        const configValue = category.values[key];
        if (!configValue) {
            console.error(
                `Config key "${key}" not found in category "${categoryName}".`,
            );
            return;
        }

        // Update the config value directly
        if (type === "Bool") {
            (target as HTMLInputElement).checked
                ? (configValue.value[type] = true)
                : (configValue.value[type] = false);
        } else if (type === "List") {
            const options = (target as HTMLSelectElement).selectedOptions;
            const selectedValues = Array.from(options).map(
                (option) => option.value,
            );
            configValue.value[type] = selectedValues;
        } else if (type === "Enum") {
            // @ts-expect-error
            configValue.value[type].current = newValue;
        } else {
            configValue.value[type] = newValue;
        }
    };

    const getConfigType = (value: ConfigValue): ConfigValueType => {
        // Determine the type of config value
        if ("String" in value.value) return "String";
        if ("Int" in value.value) return "Int";
        if ("Float" in value.value) return "Float";
        if ("Bool" in value.value) return "Bool";
        if ("List" in value.value) return "List";
        if ("Enum" in value.value) return "Enum";
        if ("Path" in value.value) return "Path";
        throw new Error("Unknown config value type");
    };

    onMount(() => {
        console.log("ExtensionConfigModal mounted");

        // Directly use the config from extension.state.config
        categories = [
            ...extension.state.config.categories,
        ];

        // Determine types of configurations
        for (const category of categories) {
            for (const [key, value] of Object.entries(category.values)) {
                configTypes[key] = getConfigType(value);
            }
        }
    });
</script>

<div class="modal-overlay">
    <div class="modal-content">
        <h1 class="modal-title">Configuration for {extension.name}</h1>

        <!-- Content area for configuration -->
        <div class="modal-body">
            {#if categories.length > 0}
                <!-- Display categories and their configurations -->
                {#each categories as category}
                    <h2 class="category-title">{category.name}</h2>
                    {#each Object.entries(category.values) as [key, value]}
                        <div class="config-item">
                            <p class="config-key">{key}</p>
                            <p class="config-desc">{value.description}</p>

                            <!-- Display config value based on its type -->
                            {#if configTypes[key] === "String"}
                                <input
                                    type="text"
                                    value={value.value.String}
                                    on:input={(e) =>
                                        handleInputChange(
                                            category.name,
                                            key,
                                            "String",
                                            e,
                                        )}
                                    class="config-input"
                                />
                            {:else if configTypes[key] === "Int"}
                                <input
                                    type="number"
                                    value={value.value.Int}
                                    on:input={(e) =>
                                        handleInputChange(
                                            category.name,
                                            key,
                                            "Int",
                                            e,
                                        )}
                                    class="config-input"
                                />
                            {:else if configTypes[key] === "Float"}
                                <input
                                    type="number"
                                    step="0.01"
                                    value={value.value.Float}
                                    on:input={(e) =>
                                        handleInputChange(
                                            category.name,
                                            key,
                                            "Float",
                                            e,
                                        )}
                                    class="config-input"
                                />
                            {:else if configTypes[key] === "Bool"}
                                <input
                                    type="checkbox"
                                    checked={value.value.Bool}
                                    on:change={(e) =>
                                        handleInputChange(
                                            category.name,
                                            key,
                                            "Bool",
                                            e,
                                        )}
                                    class="config-input config-checkbox"
                                />
                            {:else if configTypes[key] === "List"}
                                <select
                                    multiple
                                    on:change={(e) =>
                                        handleInputChange(
                                            category.name,
                                            key,
                                            "List",
                                            e,
                                        )}
                                    class="config-input"
                                >
                                    {#each value.value.List as item}
                                        <option value={item}>
                                            {item}
                                        </option>
                                    {/each}
                                </select>
                            {:else if configTypes[key] === "Enum"}
                                <select
                                    on:change={(e) =>
                                        handleInputChange(
                                            category.name,
                                            key,
                                            "Enum",
                                            e,
                                        )}
                                    class="config-input"
                                >
                                    {#each value.value.Enum.values as enumValue}
                                        <option
                                            value={enumValue}
                                            selected={enumValue ===
                                                value.value.Enum.current}
                                        >
                                            {enumValue}
                                        </option>
                                    {/each}
                                </select>
                            {/if}
                        </div>
                    {/each}
                {/each}
            {:else}
                <p>No configuration values available.</p>
            {/if}
        </div>

        <!-- Footer with close button -->
        <div class="modal-footer">
            <button class="button" on:click={onDiscardChanges}
                >Discard Changes</button
            >
            <button class="button button-primary" on:click={onSaveChanges}
                >Save Changes</button
            >
        </div>
    </div>
</div>

<style>
    .modal-overlay {
        position: fixed;
        z-index: 100;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .modal-content {
        background: var(--background-color);
        padding: 20px;
        border-radius: 10px;
        min-width: 80%;
        min-height: 60%;
        max-width: 80%;
        max-height: 80%;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
    }

    .modal-title {
        color: white;
    }

    .category-title {
        color: var(--primary-color);
        font-size: larger;
        font-weight: bold;
        margin: 20px 0 10px;
        border-bottom: 1px solid var(--primary-color);
        padding-bottom: 10px;
    }

    .modal-body {
        display: flex;
        flex: 1;
        overflow-y: auto;
        flex-direction: column;
    }

    .config-item {
        display: flex;
        justify-content: flex-start;
        align-items: flex-start;
        flex-direction: column;
        margin-bottom: 10px;
    }

    .config-key {
        color: var(--secondary-color);
        font-weight: bold;
        font-size: larger;
        margin: 0;
    }

    .config-desc {
        color: white;
        opacity: 30%;
        margin: 10px 0 10px 0;
    }

    .config-input {
        background-color: var(--background-color);
        border: 1px solid var(--primary-color);
        max-width: 95%;
        border-radius: 10px;
        padding: 10px;
        font-size: 16px;
        color: white;
        width: 100%;
        outline: none;
        appearance: unset;
    }

    .config-checkbox {
        width: 20px;
        height: 20px;
        border: solid 1px var(--primary-color);
        border-radius: 5px;
        background-color: transparent;
        color: white;
        transition: all 0.8s ease;
    }

    .config-checkbox:checked {
        background-color: var(--primary-color);
    }

    .modal-footer {
        display: flex;
        justify-content: flex-end;
        gap: 20px;
        margin-top: 10px;
    }

    .button {
        background-color: transparent;
        color: var(--primary-color);
        border: 1px solid var(--primary-color);
        border-radius: 10px;
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
        transition: all 0.4s ease;
    }

    .button:hover {
        background-color: var(--primary-color);
        color: white;
        transform: scale(0.9);
    }

    .button-primary {
        background-color: var(--primary-color);
        color: white;
    }

    .button-primary:hover {
        background-color: transparent;
        color: var(--primary-color);
    }
</style>
