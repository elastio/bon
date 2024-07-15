import { defineConfig } from "vitepress";

const base = "/bon/";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Bon",
  description:
    "Batteries-included tools for building and reshaping Rust data structures",

  cleanUrls: true,

  markdown: {
    theme: {
      dark: "dark-plus",
      light: "light-plus",
    },
  },

  base,

  head: [["link", { rel: "icon", href: `${base}bon-logo-thumb.png` }]],

  themeConfig: {
    logo: "/bon-logo-thumb.png",

    outline: 'deep',

    search: {
      provider: "local",
    },

    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Docs", link: "/docs/guide/overview" },
      { text: "Blog", link: "/blog" },
    ],

    sidebar: {
      "/docs": [
        {
          text: "Guide",
          items: [
            {
              text: "Overview",
              link: "/docs/guide/overview"
            },
            {
              text: "Limitations",
              link: "/docs/guide/limitations"
            },
            {
              text: "Alternatives",
              link: "/docs/guide/alternatives"
            }
          ]
        },
        {
          text: "Reference",
          items: [
            {
              text: "#[builder]",
              link: "/docs/reference/builder",
            },
          ],
        },
      ],
    },

    socialLinks: [{ icon: "github", link: "https://github.com/elastio/bon" }],
  },
});
