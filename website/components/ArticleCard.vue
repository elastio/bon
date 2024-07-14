<template>
    <a :href="href" class="root">
        <div class="card">
            <div class="flex">
                <div class="media">
                    <img :src="image()" :alt="title" />
                </div>
                <div class="details">
                    <h2 class="title">{{ title }}</h2>
                    <p class="excerpt">{{ truncateText(excerpt, 50) }}</p>
                    <div class="author">
                        <div>
                            <h3 class="name">{{ author }}</h3>
                            <p class="date">{{ date() }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </a>
</template>


<script>
import { withBase } from "vitepress";

export default {
  props: {
    title: {
      type: String,
      required: true,
    },
    excerpt: {
      type: String,
      required: true,
    },
    image: {
      type: String,
      required: true,
    },
    author: {
      type: String,
      required: true,
    },
    date: {
      type: String,
      required: true,
    },
    href: {
      type: String,
      required: true,
    },
  },
  methods: {
    truncateText(text, length) {
      if (text.length > length) {
        return text.substring(0, length) + "...";
      }
      return text;
    },

    image() {
        return withBase(this.image);
    },

    date() {
        return new Date(this.date).toISOString().split("T")[0];
    }
  },
};
</script>

<style scoped>

.root {
    text-decoration: none;
}

.card {
  border-radius: 0.5rem;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15);
  margin-bottom: 1.5rem;
  overflow: hidden;
  width: 100%;
}

.card:hover {
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.25);
  transition: ease-in-out 0.2s all;
}

.flex {
  display: flex;
}

.media {
  width: 45%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  padding: 20px;
}

.details {
  margin-left: 1.2rem;
}

.title {
  border-top: none;
  margin: 0 0;
}

.name {
  margin: 0 0;
  font-size: 0.7rem;
  color: #999;
}
</style>
