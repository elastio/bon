import { DefaultTheme } from "vitepress";

export const sidebars = {
    "/v2/guide": [
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
    "/v2/reference": [
        {
            text: "Reference",
            items: [
                {
                    text: "#[derive(Builder)] / #[builder]",
                    link: "/reference/builder",
                    items: [
                        {
                            text: "Top-level attributes",
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
                                    text: "on",
                                    link: "/reference/builder#on",
                                },
                                {
                                    text: "start_fn",
                                    link: "/reference/builder#start-fn",
                                },
                            ],
                        },
                        {
                            text: "Member attributes",
                            link: "/reference/builder#member-attributes",
                            items: [
                                {
                                    text: "default",
                                    link: "/reference/builder#default",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/reference/builder#finish-fn-1",
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
} satisfies Record<string, DefaultTheme.Sidebar>;