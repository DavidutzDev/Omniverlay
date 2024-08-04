<script lang="ts">
    import type { ExtensionInfo } from "../types";

    export let info: ExtensionInfo;
</script>

<div class="container" style="position: absolute; width: {info.geometry.width}px; height: {info.geometry.height}px; left: {info.geometry.x}px; top: {info.geometry.y}px;">

</div>

<style>

    .container {
        background-color: red;
    }
</style>