<script setup lang="ts">
import DefaultTheme from 'vitepress/theme'
import PageHeader from '../components/PageHeader.vue';
import PageFooter from '../components/PageFooter.vue';

const { Layout } = DefaultTheme
</script>

<template>
    <Layout>
        <template #doc-before>
            <PageHeader />
        </template>
        <template #doc-after>
            <PageFooter />
        </template>
    </Layout>
</template>

<style scoped>
.Layout {
    min-height: 100dvh;
}
</style>
