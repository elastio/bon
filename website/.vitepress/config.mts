import { defineConfig } from "vitepress";

const base = "/bon/";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Bon",
  description: "Batteries-included tools for building and reshaping Rust data structures",
  cleanUrls: true,
  markdown: {
    theme: {
      dark: "dark-plus",
      light: "light-plus"
    }
  },

  base,

  head: [
    ["link", { rel: "icon", href: `${base}bon-logo-thumb.png` }]
  ],

  themeConfig: {
    logo: "/bon-logo-thumb.png",

    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "API docs", link: "/api-docs" },
      { text: "Blog", link: "/blog" },
    ],

    sidebar: [
      {
        text: "Examples",
        items: [
          { text: "Markdown Examples", link: "/markdown-examples" },
          { text: "Runtime API Examples", link: "/api-examples" }
        ]
      }
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/elastio/bon" }
    ]
  }
});
