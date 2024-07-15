import { defineConfig } from "vitepress";

const base = "/bon/";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Bon",
  description:
    "Batteries-included tools for building and reshaping Rust data structures",

  cleanUrls: true,
  lastUpdated: true,

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

    lastUpdated: {
      formatOptions: {
        dateStyle: "long",
        timeStyle: undefined,
        forceLocale: false,
      }
    },

    editLink: {
      pattern: "https://github.com/elastio/bon/edit/master/website/:path",
      text: "Edit this page on GitHub",
    },

    search: {
      provider: "local",
    },

    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Docs", link: "/docs/guide/overview" },
      { text: "Blog", link: "/blog" },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/elastio/bon" }
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
              text: "Into conversions",
              link: "/docs/guide/into-conversions"
            },
            {
              text: "Limitations",
              link: "/docs/guide/limitations"
            },
            {
              text: "Alternatives",
              link: "/docs/guide/alternatives"
            },
          ]
        },
        {
          text: "Reference",
          items: [
            {
              text: "#[builder]",
              link: "/docs/reference/builder",
              collapsed: false,
              items: [
                {
                  text: "Attributes applicability",
                  link: "/docs/reference/builder#attributes-applicability"
                },
                {
                  text: "finish_fn",
                  link: "/docs/reference/builder#finish-fn"
                },
                {
                  text: "into",
                  link: "/docs/reference/builder#into"
                }
              ]
            },
          ],
        },
      ],
    },
  },
});
