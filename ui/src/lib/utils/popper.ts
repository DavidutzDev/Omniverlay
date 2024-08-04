import { createPopperActions, type PopperOptions } from "svelte-popperjs";

/**
 * Creates a popper with default options and allows customization through initOptions.
 * @param {Partial<PopperOptions>} initOptions - Optional configuration for the popper.
 * @returns An object containing `popperRef` and `popperContext`.
 */
export const createPopper = (initOptions?: Partial<PopperOptions<any>>) => {
    // Define default options and merge with provided initOptions
    const defaultOptions: PopperOptions<any> = {
        placement: "top",
        strategy: "fixed",
        ...initOptions,
    };

    // Create popper actions with the merged options
    const [popperRef, popperContext] = createPopperActions(defaultOptions);

    return { popperRef, popperContext, showPopper: false };
}
