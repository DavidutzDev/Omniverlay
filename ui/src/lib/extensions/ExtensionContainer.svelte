<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import type { ExtensionInfo } from "$lib/extensions/types";

    export let info: ExtensionInfo;
    export let child: SvelteComponent<any>;

    console.log("Viewport Width:", window.innerWidth);
    console.log("Viewport Height:", window.innerHeight);
    console.log("Div Width:", info.geometry.width);
    console.log("Div Height:", info.geometry.height);
    console.log("Div X Position:", info.geometry.x);
    console.log("Div Y Position:", info.geometry.y);
</script>

<div class="container" style="position: static; width: {info.geometry.width}px; height: {info.geometry.height}px; left: {info.geometry.x}px; top: {info.geometry.y}px;">
    <svelte:component this={child} {info} />
</div>
