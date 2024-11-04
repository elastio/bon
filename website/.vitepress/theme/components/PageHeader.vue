<script setup lang="ts">
import { useData, useRouter, withBase } from "vitepress";
import formatDate from "../utils/formatDate";
import { computed } from "vue";
import { latestVersion, parseRouteAsVersioned } from "../utils/versioning";

const { frontmatter } = useData();

const router = useRouter();

const versionedRoute = computed(() => parseRouteAsVersioned(router.route.path));
const isLatestVersion = computed(
    () =>
        versionedRoute.value == null ||
        versionedRoute.value.selectedVersion === latestVersion,
);
</script>

<template>
    <header class="vp-doc">
        <h1 v-if="frontmatter.title">
            {{ frontmatter.title }}
        </h1>
        <h3 v-if="frontmatter.date">
            {{ formatDate(frontmatter.date) }}
        </h3>
        <div v-if="!isLatestVersion" class="warning custom-block">
            <div class="custom-block-title">WARNING</div>
            <p>
                You are viewing the docs for an older major version of
                <code>bon</code> ({{ versionedRoute.selectedVersion }}).
            </p>
            <p>
                <a :href="withBase(`/${versionedRoute.sectionRoot}`)"
                    >Click here</a
                >
                to view the docs for the latest version ({{ latestVersion }}).
            </p>
        </div>
    </header>
</template>

<style scoped>
header {
    margin: 2rem 0;
}

header h3 {
    color: var(--vp-c-text-3);
    font-family: var(--vp-font-family-mono);
    font-weight: normal;
    margin-top: 0.25rem;
}
</style>
