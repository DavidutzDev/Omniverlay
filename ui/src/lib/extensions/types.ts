export interface ExtensionInfo {
    name: string;
    state: ExtensionState;
    layout?: ExtensionLayout;
}

export interface ExtensionState {
    is_enabled: boolean;
    config: ExtensionConfig;
}

export interface ExtensionConfig {
    categories: ConfigCategory[];
}

export interface ConfigCategory {
    name: string;
    values: Record<string, ConfigValue>;
}

export interface ConfigValue {
    description: string;
    value: Record<ConfigValueType, string | number | boolean | string[] | ConfigEnum | ConfigPath>
}

export type ConfigValueType = "String" | "Float" | "Int" | "Bool" | "List" | "Enum" | "Path";
export type ConfigEnum = {
    name: string;
    current: string;
    values: string[]
}

export type ConfigPath = string;

export interface ExtensionLayout {
    width: number;
    height: number;
    x: number;
    y: number;
}