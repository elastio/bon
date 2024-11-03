import { defineConfig, HeadConfig } from "vitepress";
import { abbr } from "@mdit/plugin-abbr";
import * as v1 from "../src/v1/config.mjs";
import * as v2 from "../src/v2/config.mjs";

const head: HeadConfig[] = [
    ["link", { rel: "icon", href: `bon-logo-thumb.png` }],
    ["meta", { property: "og:image", content: `bon-logo-thumb.png` }],
];

// Enable analytics only in the final build on CI. Locally, it's not needed.
if (process.env.CI) {
    head.push([
        "script",
        {
            defer: "",
            src: "https://umami.bon-rs.com/script.js",
            "data-website-id": "10c1ad05-7a6e-49ee-8633-5f8f75de4ab9",
        },
    ]);
}

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
            // Attributes highlighting works with better JS tokenizer 😳
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

    head,

    srcDir: "src",

    rewrites: {
        "guide/:subdir/:page": "guide/:page",
    },

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

        // Enable the search only in the final build on CI. Locally, it takes additional
        // time during the dev HMR server startup and config reloads.
        search: !process.env.CI
            ? undefined
            : {
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
            { icon: "discord", link: "https://bon-rs.com/discord" },
            { icon: "x", link: "https://x.com/veetaha" },
        ],

        sidebar: {
            ...v1.sidebars,
            ...v2.sidebars,
            "/guide": [
                {
                    text: "Overview",
                    link: "/guide/overview",
                },
                {
                    text: "Basics",
                    items: [
                        {
                            text: "Optional Members",
                            link: "/guide/optional-members",
                        },
                        {
                            text: "Into Conversions",
                            link: "/guide/into-conversions",
                        },
                        {
                            text: "Custom Conversions",
                            link: "/guide/custom-conversions",
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
                    ],
                },
                {
                    text: "Typestate API",
                    link: "/guide/typestate-api",
                    items: [
                        {
                            text: "Builder's Type Signature",
                            link: "/guide/builders-type-signature",
                        },
                        {
                            text: "Custom Methods",
                            link: "/guide/custom-methods",
                        }
                    ],
                },
                {
                    text: "Patterns",
                    items: [
                        {
                            text: "Conditional Building",
                            link: "/guide/conditional-building",
                        },
                        {
                            text: "Fallible Builders",
                            link: "/guide/fallible-builders",
                        },
                        {
                            text: "Into Conversions In-Depth",
                            link: "/guide/into-conversions-in-depth",
                        },
                        {
                            text: "Shared Configuration",
                            link: "/guide/shared-configuration",
                        },
                    ],
                },
                {
                    text: "Misc",
                    items: [
                        {
                            text: "Compatibility",
                            link: "/guide/compatibility",
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
                    text: "Internal",
                    items: [
                        {
                            text: "Contributing",
                            link: "/guide/contributing",
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
                            text: "Top-Level",
                            link: "/reference/builder#top-level-attributes",
                            collapsed: true,
                            items: [
                                {
                                    text: "builder_type",
                                    link: "/reference/builder/top-level/builder_type",
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
                                    link: "/reference/builder/top-level/finish_fn",
                                },
                                {
                                    text: "on",
                                    link: "/reference/builder/top-level/on",
                                },
                                {
                                    text: "start_fn",
                                    link: "/reference/builder/top-level/start_fn",
                                },
                                {
                                    text: "state_mod",
                                    link: "/reference/builder/top-level/state_mod",
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
                                    link: "/reference/builder/member/finish_fn",
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
                                    link: "/reference/builder/member/start_fn",
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
