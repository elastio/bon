import { useRouter, withBase } from "vitepress";

export const latestVersion = "v2";
export const versions = ["v2", "v1"];

export const sectionRoots = {
    guide: "guide/overview",
    reference: "reference/builder",
};

const base = withBase("/");

export function parseRouteAsVersioned(route: string) {
    const pathComponents = route.slice(base.length).split("/");

    const prefix = pathComponents[0];

    if (prefix == null) {
        return undefined;
    }

    let selectedVersion = latestVersion;

    // Check if we are on a non-latest route (contains a vN prefix)
    if (/^v\d+$/.test(prefix)) {
        selectedVersion = prefix;
        pathComponents.shift();
    }

    console.log(pathComponents);

    const sectionRoot = sectionRoots[pathComponents[0]];

    if (sectionRoot == null) {
        return undefined;
    }

    // Skip the version path of the path
    return {
        selectedVersion,
        sectionRoot,
    };
}
