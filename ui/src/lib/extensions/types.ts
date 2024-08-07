export interface ExtensionInfo {
    name: string;
    state: ExtensionState;
    layout?: ExtensionLayout;
}

export interface ExtensionState {
    is_enabled: boolean;
}

export interface ExtensionLayout {
    width: number;
    height: number;
    x: number;
    y: number;
}