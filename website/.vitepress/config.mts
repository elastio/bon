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

    lastUpdated: {
      formatOptions: {
        dateStyle: "long",
        timeStyle: undefined,
        forceLocale: false,
      },
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

    socialLinks: [{ icon: "github", link: "https://github.com/elastio/bon" }],

    sidebar: {
      "/docs": [
        {
          text: "Guide",
          items: [
            {
              text: "Overview",
              link: "/docs/guide/overview",
            },
            {
              text: "Into conversions",
              link: "/docs/guide/into-conversions",
            },
            {
              text: "Limitations",
              link: "/docs/guide/limitations",
            },
            {
              text: "Alternatives",
              link: "/docs/guide/alternatives",
            },
          ],
        },
        {
          text: "Reference",
          items: [
            {
              text: "#[builder]",
              link: "/docs/reference/builder",
              items: [
                {
                  text: "Top-level attributes",
                  link: "/docs/reference/builder#top-level-attributes",
                  items: [
                    {
                      text: "builder_type",
                      link: "/docs/reference/builder#builder-type",
                    },
                    {
                      text: "expose_positional_fn",
                      link: "/docs/reference/builder#expose-positional-fn",
                    },
                    {
                      text: "finish_fn",
                      link: "/docs/reference/builder#finish-fn",
                    },
                    {
                      text: "start_fn",
                      link: "/docs/reference/builder#start-fn",
                    },
                  ],
                },
                {
                  text: "Setter-level attributes",
                  link: "/docs/reference/builder#setter-level-attributes",
                  items: [
                    {
                      text: "default",
                      link: "/docs/reference/builder#default",
                    },
                    {
                      text: "into",
                      link: "/docs/reference/builder#into",
                    },
                    {
                      text: "required",
                      link: "/docs/reference/builder#required",
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    },
  },
});
