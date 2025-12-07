<script lang="ts">
    import type { ExtensionInfo } from "../types";

    export let info: ExtensionInfo;
</script>

<div class="container" style="position: absolute; width: {info.layout.width}px; height: {info.layout.height}px; left: {info.layout.x}px; top: {info.layout.y}px;">

</div>

<style>
    .container {
        background-color: red;
    }
</style>