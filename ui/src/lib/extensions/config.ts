import type { ConfigValue, ExtensionConfig } from "./types";

export const getAllConfigValues = (config: ExtensionConfig): Record<string, ConfigValue> => {
    const values: Record<string, ConfigValue> = {};

    for (const [key, value] of Object.entries(config.values)) {
        values[key] = value;
    }

    for (const category of config.categories) {
        for (const [key, value] of Object.entries(category.values)) {
            values[key] = value;
        }
    }

    return values;
}