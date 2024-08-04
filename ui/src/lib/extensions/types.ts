export interface ExtensionInfo {
    name: string;
    is_enabled: boolean;
    geometry: ExtensionGeometry;
}

export interface ExtensionGeometry {
    width: number;
    height: number;
    x: number;
    y: number;
}