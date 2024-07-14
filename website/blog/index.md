---
sidebar: false
prev: false
next: false
---

<script setup>
  import ArticleCard from '../components/ArticleCard.vue'
  import data from '../articles.json'
</script>

<!-- <Hero name="Nemo" subtitle="Welcome to my blog. This one is built with Vitepress and Vue.js. Vitepress is super cool." /> -->

# Blog posts

---


<div v-for="(article, index) in data" :key="index">
  <ArticleCard
    :title="article.title"
    :excerpt="article.excerpt"
    :image="article.image"
    :author="article.author"
    :href="article.path"
    :date="article.date"
  />
</div>
