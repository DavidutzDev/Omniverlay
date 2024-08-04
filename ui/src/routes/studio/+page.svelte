<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    // Define the interface for draggable items
    interface DraggableItem {
        id: number;
        name: string;
        x: number;  // Initial X position relative to the container
        y: number;  // Initial Y position relative to the container
    }

    let windowWidth = 1920;
    let windowHeight = 1080;
    let scaleFactor = 3;
    let screen_DPI = window.devicePixelRatio; // Use devicePixelRatio to estimate screen DPI

    // Adjust preview dimensions based on screen DPI
    let previewWidth = windowWidth / scaleFactor;
    let previewHeight = windowHeight / scaleFactor;

    let extensionWidth = 500;
    let extensionHeight = 50;

    // List of draggable items
    let items: DraggableItem[] = [
        { id: 1, name: 'Performance', x: 100, y: 100 },
        // Add more items as needed
    ];

    let draggingItem: DraggableItem | null = null;
    // Dragging offsets (current position - initial position)
    let offsetX: number;
    let offsetY: number;

    onMount(() => {
        const containerElement = document.querySelector(".preview-container") as HTMLDivElement;

        const onMouseMove = (e: MouseEvent) => {
            if (draggingItem) {
                const containerRect = containerElement.getBoundingClientRect();
                const itemElement = document.getElementById(`item-${draggingItem.id}`)!;
                const elementRect = itemElement.getBoundingClientRect();

                let x = e.clientX - offsetX;
                let y = e.clientY - offsetY;

                // Constrain the movement within boundaries of the container
                x = Math.max(containerRect.left, Math.min(x, containerRect.right - elementRect.width));
                y = Math.max(containerRect.top, Math.min(y, containerRect.bottom - elementRect.height));

                // Update position of the dragging item
                itemElement.style.left = `${x - containerRect.left}px`;
                itemElement.style.top = `${y - containerRect.top}px`;

                // Update the position in the items array
                draggingItem.x = (x - containerRect.left) / screen_DPI; // Adjust for DPI
                draggingItem.y = (y - containerRect.top) / screen_DPI; // Adjust for DPI
            }
        };

        const onMouseUp = () => {
            if (draggingItem) {
                const itemElement = document.getElementById(`item-${draggingItem.id}`)!;
                const containerRect = containerElement.getBoundingClientRect();
                const itemRect = itemElement.getBoundingClientRect();

                // Calculate the real boundaries of the item
                const realWidth = itemRect.width * (scaleFactor * screen_DPI);
                const realHeight = itemRect.height * (scaleFactor * screen_DPI);

                // Calculate the real position in relation to the original window size
                const realX = (itemRect.left - containerRect.left) * (scaleFactor * screen_DPI);
                const realY = (itemRect.top - containerRect.top) * (scaleFactor * screen_DPI);

                // Print information
                console.log(`Item ID: ${draggingItem.id}`);
                console.log(`Width: ${realWidth}px`);
                console.log(`Height: ${realHeight}px`);
                console.log(`Real Position - X: ${realX}px, Y: ${realY}px`);
                
                console.log("Updating extension geometry...");
                invoke("update_extension_geometry", {
                    name: draggingItem.name,
                    geometry: {
                        width: Math.round(realWidth),
                        height: Math.round(realHeight),
                        x: Math.round(realX),
                        y: Math.round(realY)
                    }
                });

                // Clear dragging state
                draggingItem = null;
                window.removeEventListener("mousemove", onMouseMove);
                window.removeEventListener("mouseup", onMouseUp);
            }
        };

        const onMouseDown = (e: MouseEvent, item: DraggableItem) => {
            draggingItem = item;
            const itemElement = document.getElementById(`item-${item.id}`)!;
            offsetX = e.clientX - itemElement.getBoundingClientRect().left;
            offsetY = e.clientY - itemElement.getBoundingClientRect().top;

            window.addEventListener("mousemove", onMouseMove);
            window.addEventListener("mouseup", onMouseUp);
        };

        // Attach mousedown listeners to each item
        items.forEach(item => {
            const itemElement = document.getElementById(`item-${item.id}`)!;
            itemElement.addEventListener("mousedown", (e: MouseEvent) => onMouseDown(e, item));
        });
    });
</script>

<main class="container">
    <div class="preview-container" style="width: {previewWidth}px; height: {previewHeight}px;">
        {#each items as item}
            <div
                id={"item-" + item.id}
                class="preview-item"
                style="width: {extensionWidth / scaleFactor}px; height: {extensionHeight / scaleFactor}px; left: {item.x * screen_DPI}px; top: {item.y * screen_DPI}px;"
            >
                {item.name}
            </div>
        {/each}
    </div>
</main>

<style>
    .container {
        width: 100%;
        height: calc(100vh - 32px);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        position: relative;
    }

    .preview-container {
        background-color: purple;
        position: relative;
    }

    .preview-item {
        position: absolute;
        background-color: blue;
        cursor: move;
        color: white;
        display: flex;
        align-items: center;
        justify-content: center;
        user-select: none; /* Prevent text selection during drag */
    }

    .item-dragging {
        opacity: 0.8;
    }
</style>
