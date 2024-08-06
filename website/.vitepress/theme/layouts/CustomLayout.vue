<script setup lang="ts">
import DefaultTheme from 'vitepress/theme'
import PageHeader from '../components/PageHeader.vue';
import PageFooter from '../components/PageFooter.vue';
import mediumZoom from 'medium-zoom';
import { useRoute } from 'vitepress';
import { nextTick, onMounted, watch } from 'vue';

const { Layout } = DefaultTheme;

const route = useRoute();


const initZoom = () => {
    // God bless this kind person for sharing the code snippet that adds the
    // ability to zoom images.
    // https://github.com/vuejs/vitepress/issues/854#issuecomment-2222071714
    mediumZoom('[data-zoomable]', { background: 'var(--vp-c-bg)' });
};

watch(
    () => route.path,
    () => nextTick(initZoom)
);

onMounted(initZoom);

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
