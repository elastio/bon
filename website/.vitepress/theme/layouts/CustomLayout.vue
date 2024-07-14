<script setup lang="ts">
import DefaultTheme from 'vitepress/theme'
import PageHeader from '../components/PageHeader.vue';

const { Layout } = DefaultTheme
</script>

<template>
    <Layout>
        <template #doc-before>
            <PageHeader />
        </template>
    </Layout>
</template>

<style scoped>
.Layout {
    min-height: 100dvh;
}
</style>
