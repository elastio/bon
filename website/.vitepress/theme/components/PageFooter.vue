<template>
    <VPTeamMembers class="author" size="small" :members="getAuthor(frontmatter)" />
</template>

<script setup lang="ts">
import { useData } from 'vitepress';
import { VPTeamMembers } from 'vitepress/theme'

const { frontmatter } = useData();

const allPeople = [
    {
        avatar: 'https://www.github.com/Veetaha.png',
        name: 'Veetaha',
        title: 'Lead developer',
        org: "elastio",
        orgLink: "https://github.com/elastio",
        desc: "Creator of bon",
        links: [
            { icon: 'github', link: 'https://github.com/Veetaha' },
            { icon: 'x', link: 'https://x.com/Veetaha' },
        ]
    },
]

function getAuthor(frontmatter: Record<string, unknown>) {
    return allPeople.filter(member => member.name === frontmatter.author);
}

</script>

<style scoped>
.author {
    display: inline-flex;
}
</style>
