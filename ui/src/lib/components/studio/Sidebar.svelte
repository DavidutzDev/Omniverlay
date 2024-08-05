<script lang="ts">
    import appBanner from "$lib/assets/images/app-banner.png";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    interface NavItem {
        icon: string;
        name: string;
        href: string;
        onClick?: () => void;
    }

    let navbarItems: NavItem[] = [
        { icon: "fa-solid fa-house", name: "Overview", href: "/studio" },
        { icon: "fa-solid fa-puzzle-piece", name: "Extensions", href: "/studio/extensions", },
        { icon: "fa-solid fa-desktop", name: "Layout", href: "/studio/layout" },
    ];

    let bottomNavbarItems: NavItem[] = [
        { icon: "fa-solid fa-gear", name: "Settings", href: "/studio/settings" },
        { icon: "fa-brands fa-github", name: "Contribute", href: "/studio/contribute", onClick: () => invoke("open_url", { url: "https://github.com/DavidutzDev/Omniverlay" }) },
    ];

    const isActiveItem = (link: string) => {
        const path = window.location.pathname;
        return path === link;
    };

    onMount(() => {
        console.log("Sidebar mounted");

        //Register click events
        const items = [];
        items.push(...navbarItems, ...bottomNavbarItems);

        items.forEach((item: NavItem) => {
            const element = document.getElementById(item.href);
            if (element && item.onClick) {
                element.addEventListener("click", item.onClick);
            }
        });
    });
</script>

<nav class="sidebar">
    <img
        src={appBanner}
        alt="Omniverlay Banner"
        class="banner"
    /> 
    <ul class="nav">

        {#each navbarItems as item}
            <li id={item.href} class="nav-item">
                <i class="nav-link-icon {item.icon}"></i>
                <a class="nav-link" href={item.href}>{item.name}</a>

                {#if isActiveItem(item.href)}
                    <div class="nav-link-indicator"></div>
                {/if}
            </li>
        {/each}
    </ul>

    <ul class="nav nav-bottom">
        {#each bottomNavbarItems as item}
            <li id={item.href} class="nav-item">
                <i class="nav-link-icon {item.icon}"></i>
                <a class="nav-link" href={item.href}>{item.name}</a>
            </li>
        {/each}
    </ul>

    <footer class="footer">
        <p class="footer-text">© 2022 Omniverlay</p>
    </footer>
</nav>

<style>
    .sidebar {
        background-color: var(--foreground-color);
        width: 20%;
        height: 100vh;
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column;
    }

    .banner {
        width: 100%;
    }

    .nav {
        padding: 0;
        margin: 0;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .nav-bottom {
        height: 20%;
    }

    .nav-item {
        position: relative;
        list-style: none;
        background-color: var(--secondary-color);
        border-radius: 10px;
        margin: 10px;
        width: 90%;
        height: 45px;
        transition: 0.4s ease;
        overflow: hidden;
        display: flex;
        flex-direction: row;
        align-items: center;
        padding-left: 10px;
    }

    .nav-item::before {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        width: 0px;
        height: 100%;
        background-color: var(--primary-color);
        transition: 0.5s ease;
        z-index: 1;
    }

    .nav-item:hover::before {
        width: 100%;
    }

    .nav-link {
        position: relative;
        color: white;
        text-decoration: none;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        margin-left: 10px;
        font: 1em sans-serif;
        z-index: 2;
    }

    .nav-link-icon {
        color: var(--primary-color);
        z-index: 2;
        transition: 0.4s ease;
    }

    .nav-item:hover .nav-link-icon {
        color: white;
    }

    .nav-link-indicator::before {
        position: relative;
        content: "";
        display: inline-block;
        left: -20px;
        width: 15px;
        height: 15px;
        border-radius: 7.5px;
        background-color: var(--primary-color);
        z-index: 2;
        transition: 0.4s ease;
    }

    .nav-item:hover .nav-link-indicator::before {
        background-color: white;
    }

    .footer {
        position: relative;
        width: 100%;
        height: 15%;
        background-color: var(--background-color);
        border-radius: 10px 10px 0 0;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 0 10px var(--background-color);
    }

    .footer-text {
        position: relative;
        color: white;
    }

    .footer-text::before {
        content: "";
        position: absolute;
        margin-top: 5px;
        top: 100%;
        left: calc(50% - 30%);
        width: 60%;
        height: 30%;
        background-color: white;
        border-radius: 10px;
    }
</style>
