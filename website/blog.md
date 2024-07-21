---
title: Blog
sidebar: false
editLink: false
lastUpdated: false
---

<script setup>
import { withBase } from "vitepress";
import { data as posts } from '/data/posts.data'
import formatDate from '/.vitepress/theme/utils/formatDate';
import getSorted from '/.vitepress/theme/utils/getSorted';

const sortedPosts = getSorted( posts );
</script>

<ul>
    <li v-for="post of sortedPosts">
        <strong><a :href="withBase(post.url)">{{ post.frontmatter.title }}</a></strong><br/>
        <span>{{ formatDate( post.frontmatter.date ) }}</span>
    </li>
</ul>

<style scoped>
ul {
    list-style-type: none;
    padding-left: 0;
    font-size: 1.125rem;
    line-height: 1.75;
}

li {
    display: flex;
    justify-content: space-between;
}

li span {
    font-family: var(--vp-font-family-mono);
    font-size: var(--vp-code-font-size);
}
</style>
