<script lang="ts">
    import Sidebar from "$lib/components/studio/Sidebar.svelte";
    import PageContainer from "$lib/components/studio/PageContainer.svelte";
    import { createPopper } from "$lib/utils/popper";
    import { getExtensions } from "$lib/extensions/loader";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { currentMonitor } from '@tauri-apps/api/window';
    import type { ExtensionInfo } from "$lib/extensions/types";

    let { popperContext: layoutSavePopper, popperRef: layoutSavePopperRef, showPopper: layoutSaveShowPopper } = createPopper({
        placement: "top",
        strategy: "fixed",
        modifiers: [
            {
                name: "offset",
                options: {
                    offset: [0, 10],
                },
            }
        ]
    });

    // Define the interface for draggable items
    interface DraggableItem {
        id: number;
        x: number;
        y: number;
        width: number;
        height: number;
        info: ExtensionInfo;
    }

    // Defaults values (data updated in onMount)
    let windowWidth = 1920;
    let windowHeight = 1080;
    let screenDPI = 1;

    let scaleFactor = 2 / screenDPI;

    // Adjust preview dimensions based on screen DPI
    let previewWidth = windowWidth / scaleFactor;
    let previewHeight = windowHeight / scaleFactor;

    // List of draggable items
    let items: DraggableItem[] = [];

    let draggingItem: DraggableItem | null = null;
    // Dragging offsets (current position - initial position)
    let offsetX: number;
    let offsetY: number;

    const updateItemsElements = () => {
        items.forEach((item) => {
            const itemElement = document.getElementById(`item-${item.id}`)!;

            console.log(item);

            itemElement.style.left = `${item.x * screenDPI}px`;
            itemElement.style.top = `${item.y * screenDPI}px`;
            itemElement.style.width = `${item.width * screenDPI}px`;
            itemElement.style.height = `${item.height * screenDPI}px`;

            itemElement.textContent = item.info.name;
            // OTHER CSS IN STATIC APP.CSS
        });
    };

    const onSaveChanges = () => {
        items.forEach((item) => {
            item.info.geometry.x = item.x * scaleFactor;
            item.info.geometry.y = item.y * scaleFactor;
        });

        invoke("update_extensions", {
            extensions: items.map((item) => item.info),
        });
    };

    const onDiscardChanges = () => {
        items.forEach((item) => {
            item.x = item.info.geometry.x / scaleFactor;
            item.y = item.info.geometry.y / scaleFactor;
        });

        updateItemsElements();
    };

    onMount(async () => {
        const monitor = await currentMonitor();

        if (!monitor) {
            console.error("No monitor found");

            //TODO PROPER ERROR
            return;
        }

        windowWidth = monitor.size.width;
        windowHeight = monitor.size.height;
        screenDPI = monitor.scaleFactor;

        console.log("Screen DPI: ", screenDPI);

        let infos: ExtensionInfo[] = (await getExtensions()).filter(
            (info) => info.is_enabled,
        );

        console.log(infos);

        items = infos.map((info, index) => ({
            id: index,
            x: info.geometry.x / scaleFactor,
            y: info.geometry.y / scaleFactor,
            width: info.geometry.width / scaleFactor,
            height: info.geometry.height / scaleFactor,
            info: info,
        }));

        //Populate the DOM with the items
        const previewContainer = document.getElementById(
            "preview-container",
        ) as HTMLDivElement;

        items.forEach((item) => {
            const itemElement = document.createElement("div");

            itemElement.id = `item-${item.id}`;
            itemElement.classList.add("studio-preview-item");
            itemElement.textContent = item.info.name;

            previewContainer.appendChild(itemElement);
        });

        updateItemsElements();

        const containerElement = document.querySelector(
            ".preview-container",
        ) as HTMLDivElement;

        const onMouseMove = (e: MouseEvent) => {
            if (draggingItem) {
                const containerRect = containerElement.getBoundingClientRect();
                const itemElement = document.getElementById(
                    `item-${draggingItem.id}`,
                )!;
                const elementRect = itemElement.getBoundingClientRect();

                let x = e.clientX - offsetX;
                let y = e.clientY - offsetY;

                // Constrain the movement within boundaries of the container
                x = Math.max(
                    containerRect.left,
                    Math.min(x, containerRect.right - elementRect.width),
                );
                y = Math.max(
                    containerRect.top,
                    Math.min(y, containerRect.bottom - elementRect.height),
                );

                // Update position of the dragging item
                itemElement.style.left = `${x - containerRect.left}px`;
                itemElement.style.top = `${y - containerRect.top}px`;

                // Update the position in the items array
                draggingItem.x = (x - containerRect.left) / screenDPI; // Adjust for DPI
                draggingItem.y = (y - containerRect.top) / screenDPI; // Adjust for DPI
            }
        };

        const onMouseUp = () => {
            if (draggingItem) {
                const itemElement = document.getElementById(
                    `item-${draggingItem.id}`,
                )!;
                const containerRect = containerElement.getBoundingClientRect();
                const itemRect = itemElement.getBoundingClientRect();

                // Calculate the real boundaries of the item
                const realWidth = itemRect.width * (scaleFactor * screenDPI);
                const realHeight = itemRect.height * (scaleFactor * screenDPI);

                // Calculate the real position in relation to the original window size
                const realX =
                    (itemRect.left - containerRect.left) *
                    (scaleFactor * screenDPI);
                const realY =
                    (itemRect.top - containerRect.top) *
                    (scaleFactor * screenDPI);

                // Print information
                console.log(`Item ID: ${draggingItem.id}`);
                console.log(`Width: ${realWidth}px`);
                console.log(`Height: ${realHeight}px`);
                console.log(`Real Position - X: ${realX}px, Y: ${realY}px`);

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
        items.forEach((item) => {
            const itemElement = document.getElementById(`item-${item.id}`)!;

            console.log(itemElement);

            itemElement.addEventListener("mousedown", (e: MouseEvent) =>
                onMouseDown(e, item),
            );
        });
    });
</script>

<main class="container">
    <Sidebar />
    <PageContainer title="Layout Editor">
        <div
            class="dashed-container"
            style="width: {previewWidth + 10}px; height: {previewHeight +
                10}px;"
        >
            <div
                id="preview-container"
                class="preview-container"
                style="width: {previewWidth}px; height: {previewHeight}px;"
            ></div>
        </div>

        <div class="buttons-container">
            <button
                use:layoutSavePopperRef
                on:mouseenter={() => (layoutSaveShowPopper = true)}
                on:mouseleave={() => (layoutSaveShowPopper = false)}
                class="button button-square"
                ><i class="fa-solid fa-floppy-disk" /></button
            >
            {#if layoutSaveShowPopper}
                <div class="popper" use:layoutSavePopper>Save current layout</div>
            {/if}
            <button class="button" on:click={onDiscardChanges}
                >Discard Changes</button
            >
            <button class="button button-primary" on:click={onSaveChanges}
                >Save Changes</button
            >
        </div>
    </PageContainer>
</main>

<style>
    .container {
        display: flex;
        flex-direction: row;
    }

    .dashed-container {
        position: relative;
        margin: 20px;
        border: dashed 4px var(--primary-color);
    }

    .preview-container {
        position: relative;
        margin: 5px;
    }

    .buttons-container {
        position: relative;
        width: 100%;
        display: flex;
        justify-content: flex-end;
        gap: 20px;
        padding-right: 30px;
        box-sizing: border-box;
    }

    .button {
        background-color: transparent;
        color: var(--primary-color);
        border: 1px solid var(--primary-color);
        border-radius: 10px;
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
        transition:
            background-color 0.3s ease,
            color 0.3s ease,
            transform 0.5s ease;
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

    .button-square {
        width: 40px;
        height: 40%;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .popper {
        background-color: var(--primary-color);
        color: white;
        padding: 5px;
        border: solid 1px var(--background-color);
        border-radius: 10px;
    }
</style>
