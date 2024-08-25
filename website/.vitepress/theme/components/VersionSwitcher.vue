<script setup lang="ts">

// This implementation was based on the component from `vitepress-versioning-plugin`:
// https://github.com/IMB11/vitepress-versioning-plugin/blob/16bc30996b2245bfd48cb15732900ba214c3affb/src/components/VersionSwitcher.vue#L1

import { useRouter } from 'vitepress';
import { computed } from 'vue';
import VPMenuLink from 'vitepress/dist/client/theme-default/components/VPMenuLink.vue';
import VPFlyout from 'vitepress/dist/client/theme-default/components/VPFlyout.vue';
import { parseRouteAsVersioned, latestVersion, versions } from '../utils/versioning';

const router = useRouter();

const latestVersionText = `${latestVersion} (latest)`;

const versionedRoute = computed(() => parseRouteAsVersioned(router.route.path));
const selectedVersion = computed(() => versionedRoute.value.selectedVersion);
const sectionRoot = computed(() => versionedRoute.value.sectionRoot);

const selectedVersionText = computed(() =>
    selectedVersion.value === latestVersion
        ? latestVersionText
        : selectedVersion.value
);

</script>

<template>
    <VPFlyout v-if="versionedRoute" icon="versioning-icon" :button="selectedVersionText" label="Switch Version">
        <div class="items">
            <template v-for="version in versions" :key="version">
                <VPMenuLink v-if="selectedVersion != version" :item="{
                    text: version === latestVersion ? latestVersionText : version,
                    link: version === selectedVersion
                        ? undefined
                        : version === latestVersion
                            ? `/${sectionRoot}`
                            : `/${version}/${sectionRoot}`,
                }" />
            </template>
        </div>
    </VPFlyout>
</template>

<style>
.versioning-icon.option-icon {
    margin-right: 2px !important;
}

.versioning-icon {
    --icon: url("data:image/svg+xml;charset=utf-8;base64,PHN2ZyB3aWR0aD0iNjRweCIgaGVpZ2h0PSI2NHB4IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHN0cm9rZS13aWR0aD0iMi4yIiBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGNvbG9yPSIjMDAwMDAwIj48cGF0aCBkPSJNMTcgN0MxOC4xMDQ2IDcgMTkgNi4xMDQ1NyAxOSA1QzE5IDMuODk1NDMgMTguMTA0NiAzIDE3IDNDMTUuODk1NCAzIDE1IDMuODk1NDMgMTUgNUMxNSA2LjEwNDU3IDE1Ljg5NTQgNyAxNyA3WiIgc3Ryb2tlPSIjMDAwMDAwIiBzdHJva2Utd2lkdGg9IjIuMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48L3BhdGg+PHBhdGggZD0iTTcgN0M4LjEwNDU3IDcgOSA2LjEwNDU3IDkgNUM5IDMuODk1NDMgOC4xMDQ1NyAzIDcgM0M1Ljg5NTQzIDMgNSAzLjg5NTQzIDUgNUM1IDYuMTA0NTcgNS44OTU0MyA3IDcgN1oiIHN0cm9rZT0iIzAwMDAwMCIgc3Ryb2tlLXdpZHRoPSIyLjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PC9wYXRoPjxwYXRoIGQ9Ik03IDIxQzguMTA0NTcgMjEgOSAyMC4xMDQ2IDkgMTlDOSAxNy44OTU0IDguMTA0NTcgMTcgNyAxN0M1Ljg5NTQzIDE3IDUgMTcuODk1NCA1IDE5QzUgMjAuMTA0NiA1Ljg5NTQzIDIxIDcgMjFaIiBzdHJva2U9IiMwMDAwMDAiIHN0cm9rZS13aWR0aD0iMi4yIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjwvcGF0aD48cGF0aCBkPSJNNyA3VjE3IiBzdHJva2U9IiMwMDAwMDAiIHN0cm9rZS13aWR0aD0iMi4yIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjwvcGF0aD48cGF0aCBkPSJNMTcgN1Y4QzE3IDEwLjUgMTUgMTEgMTUgMTFMOSAxM0M5IDEzIDcgMTMuNSA3IDE2VjE3IiBzdHJva2U9IiMwMDAwMDAiIHN0cm9rZS13aWR0aD0iMi4yIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjwvcGF0aD48L3N2Zz4=")
}

.VPFlyout .button {
    padding: 0;
}

.VPFlyout .menu {
    position: absolute;
    right: unset;
    z-index: 1;
}
</style>

<style scoped></style>
