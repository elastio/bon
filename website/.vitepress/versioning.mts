import { DefaultTheme } from "vitepress";
import * as v1 from "../v1/config.mjs";

const latestVersion = "v2";

/**
 * All historical versions of the site with their sidebar configurations.
 */
const olderConfigs = {
    v1,
};

/**
 * Generates the sidebar that lists all the historical versions of the page.
 */
function versionsSidebarItem(link: string): DefaultTheme.SidebarItem {
    const oldItems = Object.keys(olderConfigs).map((version) => ({
        text: version,
        link: `/${version}/${link}`,
    }));

    return {
        text: "Versions",
        collapsed: true,
        items: [
            {
                text: `${latestVersion} (latest)`,
                link: `/${link}`,
            },
            ...oldItems,
        ],
    };
}

export const versionsSidebar = {
    guide: versionsSidebarItem("guide/overview"),
    reference: versionsSidebarItem("reference/builder"),
};

export const sidebarsByVersion = Object.fromEntries(
    Object.entries(olderConfigs).flatMap(([version, { sidebars }]) =>
        Object.entries(sidebars).map(([page, items]) => [
            `/${version}/${page}`,
            [...items, versionsSidebar[page]],
        ])
    )
);
