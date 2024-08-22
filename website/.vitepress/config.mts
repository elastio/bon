import { defineConfig } from "vitepress";
import { abbr } from "@mdit/plugin-abbr";

const base = "/bon/";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Bon",
    description: "Generate builders for everything!",

    cleanUrls: true,
    lastUpdated: true,

    markdown: {
        theme: {
            dark: "dark-plus",
            light: "light-plus",
        },

        config: (md) => {
            // use more markdown-it plugins
            md.use(abbr);
        },
    },

    base,

    head: [
        ["link", { rel: "icon", href: `${base}bon-logo-thumb.png` }],
        [
            "meta",
            { property: "og:image", content: `${base}bon-logo-thumb.png` },
        ],
    ],

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
            { text: "Changelog", link: "/changelog" },
            { text: "Blog", link: "/blog" },
        ],

        socialLinks: [
            { icon: "github", link: "https://github.com/elastio/bon" },
        ],

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
                            text: "Optional members",
                            link: "/docs/guide/optional-members",
                        },
                        {
                            text: "Compatibility",
                            link: "/docs/guide/compatibility",
                        },
                        {
                            text: "Into conversions",
                            link: "/docs/guide/into-conversions",
                        },
                        {
                            text: "Documenting",
                            link: "/docs/guide/documenting",
                        },
                        {
                            text: "Limitations",
                            link: "/docs/guide/limitations",
                        },
                        {
                            text: "Benchmarks",
                            link: "/docs/guide/benchmarks",
                        },
                        {
                            text: "Alternatives",
                            link: "/docs/guide/alternatives",
                        },
                        {
                            text: "Troubleshooting",
                            link: "/docs/guide/troubleshooting",
                        },
                    ],
                },
                {
                    text: "Contributing",
                    link: "/docs/contributing",
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
                                        {
                                            text: "on",
                                            link: "/docs/reference/builder#on",
                                        },
                                    ],
                                },
                                {
                                    text: "Member-level attributes",
                                    link: "/docs/reference/builder#member-level-attributes",
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
                                            text: "name",
                                            link: "/docs/reference/builder#name",
                                        },
                                        {
                                            text: "skip",
                                            link: "/docs/reference/builder#skip",
                                        },
                                    ],
                                },
                            ],
                        },
                        {
                            text: "#[bon]",
                            link: "/docs/reference/bon",
                        },
                        {
                            text: "Other items on docs.rs",
                            link: "https://docs.rs/bon/latest/bon/",
                        },
                    ],
                },
                {
                    text: "Versions",
                    collapsed: true,
                    items: [
                        {
                            text: "v2 (latest)",
                            link: "/docs/guide/overview",
                        },
                        {
                            text: "v1",
                            link: "/docs/guide/overview",
                        },
                    ],
                },
            ],
        },
    },
});
