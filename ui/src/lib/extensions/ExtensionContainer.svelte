<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import type { ExtensionInfo } from "$lib/extensions/types";

    export let info: ExtensionInfo;
    export let child: SvelteComponent<any>;

    console.log("Viewport Width:", window.innerWidth);
    console.log("Viewport Height:", window.innerHeight);
    console.log("Div Width:", info.layout.width);
    console.log("Div Height:", info.layout.height);
    console.log("Div X Position:", info.layout.x);
    console.log("Div Y Position:", info.layout.y);
</script>

<div class="container" style="position: static; width: {info.layout.width}px; height: {info.layout.height}px; left: {info.layout.x}px; top: {info.layout.y}px;">
    <svelte:component this={child} {info} />
</div>
