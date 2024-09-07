---
title: Blog
sidebar: false
editLink: false
lastUpdated: false
layout: home
---

<script setup>
import { withBase } from "vitepress";
import { data as posts } from '/data/posts.data'
import formatDate from '/.vitepress/theme/utils/formatDate';
import getSorted from '/.vitepress/theme/utils/getSorted';

const filteredPosts = posts.filter(post => !post.frontmatter.hidden);
const sortedPosts = getSorted(filteredPosts);
</script>

<ul>
    <li v-for="post of sortedPosts">
        <span>{{ formatDate( post.frontmatter.date ) }}</span>
        <strong><a :href="withBase(post.url)">{{ post.frontmatter.title }}</a></strong>
    </li>
</ul>

<style scoped>
ul {
    list-style-type: none;
    padding-left: 0;
    font-size: 1.125rem;
    line-height: 1.75;
}

a {
    text-decoration: none;
}

li {
    display: flex;
    gap: 20px;
    align-items: baseline
}

li span {
    font-family: var(--vp-font-family-mono);
    font-size: var(--vp-code-font-size);
}
</style>
