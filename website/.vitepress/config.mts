import { defineConfig, HeadConfig } from "vitepress";
import { abbr } from "@mdit/plugin-abbr";
import * as v1 from "../src/v1/config.mjs";
import * as v2 from "../src/v2/config.mjs";

const head: HeadConfig[] = [
    ["link", { rel: "icon", href: `/bon-logo-thumb.png` }],
    ["meta", { property: "og:image", content: `/bon-logo-thumb.png` }],
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

const srcDir = "src";

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
                            ["underlying type"]:
                                "For required members, it's the type of the member itself. " +
                                "For optional members, it's the type `T` inside of the `Option<T>`",
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
            // Attributes highlighting works better with JS tokenizer 😳
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

    srcExclude: ["README.md", "infra/**", "doctests/**"],

    head,

    srcDir,

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
            pattern: `https://github.com/elastio/bon/edit/master/website/${srcDir}/:path`,
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
            {
                icon: "opencollective",
                link: "https://opencollective.com/bon-rs",
            },
            { icon: "patreon", link: "https://patreon.com/Veetaha" },
            { icon: "kofi", link: "https://ko-fi.com/Veetaha" },
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
                    text: "Alternatives",
                    link: "/guide/alternatives",
                },
                {
                    text: "Basics",
                    link: "/guide/basics",
                    items: [
                        {
                            text: "Optional Members",
                            link: "/guide/basics/optional-members",
                        },
                        {
                            text: "Into Conversions",
                            link: "/guide/basics/into-conversions",
                        },
                        {
                            text: "Custom Conversions",
                            link: "/guide/basics/custom-conversions",
                        },
                        {
                            text: "Positional Members",
                            link: "/guide/basics/positional-members",
                        },
                        {
                            text: "Derives for Builders",
                            link: "/guide/basics/derives-for-builders",
                        },
                        {
                            text: "Documenting",
                            link: "/guide/basics/documenting",
                        },
                        {
                            text: "Compatibility",
                            link: "/guide/basics/compatibility",
                        },
                    ],
                },
                {
                    text: "Typestate API",
                    link: "/guide/typestate-api",
                    items: [
                        {
                            text: "Builder's Type Signature",
                            link: "/guide/typestate-api/builders-type-signature",
                        },
                        {
                            text: "Custom Methods",
                            link: "/guide/typestate-api/custom-methods",
                        },
                        {
                            text: "Builder Fields",
                            link: "/guide/typestate-api/builder-fields",
                        },
                        {
                            text: "Getters",
                            link: "/guide/typestate-api/getters",
                        },
                    ],
                },
                {
                    text: "Patterns",
                    link: "/guide/patterns",
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
                            text: "Optional Generic Members",
                            link: "/guide/patterns/optional-generic-members",
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
                    text: "Benchmarks",
                    link: "/guide/benchmarks",
                    items: [
                        {
                            text: "Runtime",
                            link: "/guide/benchmarks/runtime",
                        },
                        {
                            text: "Compilation",
                            link: "/guide/benchmarks/compilation",
                        },
                    ],
                },
                {
                    text: "Troubleshooting",
                    link: "/guide/troubleshooting",
                    items: [
                        {
                            text: "Limitations",
                            link: "/guide/troubleshooting/limitations",
                        },
                    ],
                },
                {
                    text: "Contributing",
                    link: "/guide/contributing",
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
                            collapsed: false,
                            items: [
                                {
                                    text: "builder_type",
                                    link: "/reference/builder/top-level/builder_type",
                                },
                                {
                                    text: "const",
                                    link: "/reference/builder/top-level/const",
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
                            collapsed: false,
                            items: [
                                {
                                    text: "default",
                                    link: "/reference/builder/member/default",
                                },
                                {
                                    text: "field",
                                    link: "/reference/builder/member/field",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/reference/builder/member/finish_fn",
                                },
                                {
                                    text: "getter",
                                    link: "/reference/builder/member/getter",
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
                                    text: "required",
                                    link: "/reference/builder/member/required",
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
