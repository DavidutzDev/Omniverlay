<script lang="ts">
    import Sidebar from "$lib/components/studio/Sidebar.svelte";
    import PageContainer from "$lib/components/studio/PageContainer.svelte";
    import { createPopper } from "$lib/utils/popper";
    import { getExtensions } from "$lib/extensions/loader";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { currentMonitor } from "@tauri-apps/api/window";
    import type { ExtensionInfo } from "$lib/extensions/types";

    let {
        popperContext: layoutSavePopper,
        popperRef: layoutSavePopperRef,
        showPopper: layoutSaveShowPopper,
    } = createPopper({
        placement: "top",
        strategy: "fixed",
        modifiers: [
            {
                name: "offset",
                options: {
                    offset: [0, 10],
                },
            },
        ],
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

    // Adjust preview dimensions based on screen DPI
    let previewWidth = 0;
    let previewHeight = 0;

    let xRatio = 1;
    let yRatio = 1;

    // List of draggable items
    let items: DraggableItem[] = [];

    let draggingItem: DraggableItem | null = null;
    // Dragging offsets (current position - initial position)
    let offsetX: number;
    let offsetY: number;

    const updateItemsElements = () => {
        items.forEach((item) => {
            const itemElement = document.getElementById(`item-${item.id}`)!;

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
            item.info.layout!.x = Math.round(item.x * xRatio);
            item.info.layout!.y = Math.round(item.y * yRatio);
        });

        invoke("update_extensions_layout", {
            layouts: new Map(items.map((item) => [item.info.name, item.info.layout])),
        });
    };

    const onDiscardChanges = () => {
        items.forEach((item) => {
            item.x = item.info.layout!.x / xRatio;
            item.y = item.info.layout!.y / yRatio;
        });

        updateItemsElements();
    };

    onMount(async () => {
        const monitor = await currentMonitor();

        // Update defaults values
        screenDPI = monitor!.scaleFactor;
        windowWidth = monitor!.size.width;
        windowHeight = monitor!.size.height;

        const pageContainer = document.getElementById(
            "page-container",
        ) as HTMLDivElement;

        previewWidth = (pageContainer.getBoundingClientRect().width / 100) * 94; // 92% of the page
        previewHeight =
            (pageContainer.getBoundingClientRect().height / 100) * 70; // 65% of the page

        xRatio = windowWidth / previewWidth;
        yRatio = windowHeight / previewHeight;

        // Get the list of extensions
        let infos: ExtensionInfo[] = (await getExtensions()).filter(
            (info) => info.state.is_enabled && info.layout && info.layout.width > 0 && info.layout.height > 0,
        );

        items = infos.map((info, index) => ({
            id: index,
            x: info.layout!.x / xRatio,
            y: info.layout!.y / yRatio,
            width: info.layout!.width / xRatio,
            height: info.layout!.height / yRatio,
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

        // Attach mouse listeners to each item
        const onMouseMove = (e: MouseEvent) => {
            if (draggingItem) {
                const containerRect = previewContainer.getBoundingClientRect();
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
        <div class="top-container">
            <div class="accessibility-item">
                <i class="accessibility-icon fa-solid fa-computer-mouse" />
                <span>Hold left click to move</span>
            </div>
            <div class="accessibility-item">
                <i class="accessibility-icon fa-solid fa-computer-mouse-scrollwheel" />
                <span>Scroll to resize</span>
            </div>
        </div>

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
                <div class="popper" use:layoutSavePopper>
                    Save current layout
                </div>
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

    .top-container {
        position: relative;
        display: flex;
        justify-content: flex-end;
        padding-right: 30px;
        gap: 20px;
    }

    .accessibility-item {
        color: white;
    }

    .accessibility-icon {
        color: var(--primary-color);
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
