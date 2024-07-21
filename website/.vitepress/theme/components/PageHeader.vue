<template>
    <header class="vp-doc">
        <h1 v-if="frontmatter.title">
            {{ frontmatter.title }}
        </h1>
        <h3 v-if="frontmatter.date">
            {{ formatDate( frontmatter.date ) }}
        </h3>
    </header>
</template>

<script setup lang="ts">
import { useData } from 'vitepress';
import formatDate from '../utils/formatDate';

const { frontmatter } = useData();

console.log("Setup header", frontmatter);


</script>

<style scoped>
header {
    margin: 2rem 0;
}

header h3 {
    color: var( --vp-c-text-3 );
    font-family: var(--vp-font-family-mono);
    font-weight: normal;
    margin-top: 0.25rem;
}
</style>
