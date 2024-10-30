import { defineConfig } from "vitepress";
import { abbr } from "@mdit/plugin-abbr";
import * as v1 from "../v1/config.mjs";
import * as v2 from "../v2/config.mjs";

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Bon",
    description:
        "Next-gen compile-time-checked builder generator, named function's arguments, and more!",

    cleanUrls: true,
    lastUpdated: true,

    vite: {
        plugins: [
            {
                name: "inject-abbreviations",
                transform: {
                    order: "pre",
                    handler(src, id) {
                        if (!id.endsWith(".md")) {
                            return;
                        }

                        const abbrs = {
                            Member: "Struct field or function argument",
                            member: "Struct field or function argument",
                            members: "Struct fields or function arguments",
                            ["starting function"]:
                                "Function that creates the builder (e.g. `builder()`)",
                            ["finishing function"]:
                                "Method on the builder struct that finishes building (e.g. `build()` or `call()`)",
                        };

                        const abbrsStr = Object.entries(abbrs)
                            .map(([key, value]) => `*[${key}]: ${value}`)
                            .join("\n");

                        return `${src}\n\n${abbrsStr}`;
                    },
                },
            },
        ],
    },

    markdown: {
        languageAlias: {
            attr: "js",
        },

        theme: {
            dark: "dark-plus",
            light: "light-plus",
        },

        config: (md) => {
            // use more markdown-it plugins
            md.use(abbr);
        },
    },

    srcExclude: ["README.md", "infra/**"],

    head: [
        ["link", { rel: "icon", href: `bon-logo-thumb.png` }],
        ["meta", { property: "og:image", content: `bon-logo-thumb.png` }],
        [
            "script",
            {
                defer: "",
                src: "https://umami.bon-rs.com/script.js",
                "data-website-id": "10c1ad05-7a6e-49ee-8633-5f8f75de4ab9",
            },
        ],
    ],

    // https://vitepress.dev/reference/default-theme-config
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
            ...v2.sidebars,
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
                            text: "Positional Members",
                            link: "/guide/positional-members",
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
                        {
                            text: "Shared Configuration",
                            link: "/guide/patterns/shared-configuration",
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
                    text: "#[derive(Builder)] / #[builder]",
                    link: "/reference/builder",
                    items: [
                        {
                            text: "Top-level",
                            link: "/reference/builder#top-level-attributes",
                            collapsed: true,
                            items: [
                                {
                                    text: "builder_type",
                                    link: "/reference/builder/top-level/builder-type",
                                },
                                {
                                    text: "crate",
                                    link: "/reference/builder/top-level/crate",
                                },
                                {
                                    text: "derive",
                                    link: "/reference/builder/top-level/derive",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/reference/builder/top-level/finish-fn",
                                },
                                {
                                    text: "on",
                                    link: "/reference/builder/top-level/on",
                                },
                                {
                                    text: "start_fn",
                                    link: "/reference/builder/top-level/start-fn",
                                },
                                {
                                    text: "state_mod",
                                    link: "/reference/builder/top-level/state-mod",
                                },
                            ],
                        },
                        {
                            text: "Member",
                            link: "/reference/builder#member-attributes",
                            collapsed: true,
                            items: [
                                {
                                    text: "default",
                                    link: "/reference/builder/member/default",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/reference/builder/member/finish-fn",
                                },
                                {
                                    text: "into",
                                    link: "/reference/builder/member/into",
                                },
                                {
                                    text: "name",
                                    link: "/reference/builder/member/name",
                                },
                                {
                                    text: "overwritable 🔬",
                                    link: "/reference/builder/member/overwritable",
                                },
                                {
                                    text: "setters",
                                    link: "/reference/builder/member/setters",
                                },
                                {
                                    text: "skip",
                                    link: "/reference/builder/member/skip",
                                },
                                {
                                    text: "start_fn",
                                    link: "/reference/builder/member/start-fn",
                                },
                                {
                                    text: "transparent",
                                    link: "/reference/builder/member/transparent",
                                },
                                {
                                    text: "with",
                                    link: "/reference/builder/member/with",
                                },
                            ],
                        },
                        {
                            text: "Typestate API",
                            link: "/reference/builder/typestate-api",
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
    },
});
