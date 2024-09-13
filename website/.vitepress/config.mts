import { defineConfig } from "vitepress";
import { abbr } from "@mdit/plugin-abbr";
import * as v1 from "../v1/config.mjs";

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
            { text: "Guide", link: "/guide/overview" },
            { text: "Reference", link: "/reference/builder" },
            { text: "Changelog", link: "/changelog" },
            { text: "Blog", link: "/blog" },
        ],

        socialLinks: [
            { icon: "github", link: "https://github.com/elastio/bon" },
            { icon: "discord", link: "https://discord.gg/QcBYSamw4c" },
            { icon: "x", link: "https://x.com/veetaha" },
        ],

        sidebar: {
            ...v1.sidebars,
            "/guide": [
                {
                    text: "Guide",
                    items: [
                        {
                            text: "Overview",
                            link: "/guide/overview",
                        },
                        {
                            text: "Optional Members",
                            link: "/guide/optional-members",
                        },
                        {
                            text: "Compatibility",
                            link: "/guide/compatibility",
                        },
                        {
                            text: "Inspecting",
                            link: "/guide/inspecting",
                        },
                        {
                            text: "Documenting",
                            link: "/guide/documenting",
                        },
                        {
                            text: "Limitations",
                            link: "/guide/limitations",
                        },
                        {
                            text: "Benchmarks",
                            link: "/guide/benchmarks",
                        },
                        {
                            text: "Alternatives",
                            link: "/guide/alternatives",
                        },
                        {
                            text: "Troubleshooting",
                            link: "/guide/troubleshooting",
                        },
                    ],
                },
                {
                    text: "Patterns",
                    items: [
                        {
                            text: "Conditional Building",
                            link: "/guide/patterns/conditional-building",
                        },
                        {
                            text: "Fallible Builders",
                            link: "/guide/patterns/fallible-builders",
                        },
                        {
                            text: "Into Conversions In-Depth",
                            link: "/guide/patterns/into-conversions-in-depth",
                        },
                    ],
                },
                {
                    text: "Internal",
                    items: [
                        {
                            text: "Contributing",
                            link: "/guide/internal/contributing",
                        },
                    ],
                },
            ],
            "/reference": [
                {
                    text: "Reference",
                    items: [
                        {
                            text: "Builder macros",
                            link: "/reference/builder",
                            items: [
                                {
                                    text: "Top-Level Attributes",
                                    link: "/reference/builder#top-level-attributes",
                                    items: [
                                        {
                                            text: "builder_type",
                                            link: "/reference/builder#builder-type",
                                        },
                                        {
                                            text: "derive",
                                            link: "/reference/builder#derive",
                                        },
                                        {
                                            text: "expose_positional_fn",
                                            link: "/reference/builder#expose-positional-fn",
                                        },
                                        {
                                            text: "finish_fn",
                                            link: "/reference/builder#finish-fn",
                                        },
                                        {
                                            text: "start_fn",
                                            link: "/reference/builder#start-fn",
                                        },
                                        {
                                            text: "on",
                                            link: "/reference/builder#on",
                                        },
                                    ],
                                },
                                {
                                    text: "Member-Level Attributes",
                                    link: "/reference/builder#member-level-attributes",
                                    items: [
                                        {
                                            text: "default",
                                            link: "/reference/builder#default",
                                        },
                                        {
                                            text: "into",
                                            link: "/reference/builder#into",
                                        },
                                        {
                                            text: "name",
                                            link: "/reference/builder#name",
                                        },
                                        {
                                            text: "skip",
                                            link: "/reference/builder#skip",
                                        },
                                        {
                                            text: "start_fn",
                                            link: "/reference/builder#start-fn-1",
                                        },
                                        {
                                            text: "finish_fn",
                                            link: "/reference/builder#finish-fn-1",
                                        },

                                    ],
                                },
                            ],
                        },
                        {
                            text: "#[bon]",
                            link: "/reference/bon",
                        },
                        {
                            text: "Other items on docs.rs",
                            link: "https://docs.rs/bon/latest/bon/",
                        },
                    ],
                },
            ],
        },
    },
});
