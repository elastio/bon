import { DefaultTheme } from "vitepress";

export const sidebars = {
    "/v2/guide": [
        {
            text: "Guide",
            items: [
                {
                    text: "Overview",
                    link: "/v2/guide/overview",
                },
                {
                    text: "Optional Members",
                    link: "/v2/guide/optional-members",
                },
                {
                    text: "Compatibility",
                    link: "/v2/guide/compatibility",
                },
                {
                    text: "Positional Members",
                    link: "/v2/guide/positional-members",
                },
                {
                    text: "Inspecting",
                    link: "/v2/guide/inspecting",
                },
                {
                    text: "Documenting",
                    link: "/v2/guide/documenting",
                },
                {
                    text: "Limitations",
                    link: "/v2/guide/limitations",
                },
                {
                    text: "Benchmarks",
                    link: "/v2/guide/benchmarks",
                },
                {
                    text: "Alternatives",
                    link: "/v2/guide/alternatives",
                },
                {
                    text: "Troubleshooting",
                    link: "/v2/guide/troubleshooting",
                },
            ],
        },
        {
            text: "Patterns",
            items: [
                {
                    text: "Conditional Building",
                    link: "/v2/guide/patterns/conditional-building",
                },
                {
                    text: "Fallible Builders",
                    link: "/v2/guide/patterns/fallible-builders",
                },
                {
                    text: "Into Conversions In-Depth",
                    link: "/v2/guide/patterns/into-conversions-in-depth",
                },
                {
                    text: "Shared Configuration",
                    link: "/v2/guide/patterns/shared-configuration",
                },
            ],
        },
        {
            text: "Internal",
            items: [
                {
                    text: "Contributing",
                    link: "/v2/guide/internal/contributing",
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
                    link: "/v2/reference/builder",
                    items: [
                        {
                            text: "Top-level attributes",
                            link: "/v2/reference/builder#top-level-attributes",
                            items: [
                                {
                                    text: "builder_type",
                                    link: "/v2/reference/builder#builder-type",
                                },
                                {
                                    text: "derive",
                                    link: "/v2/reference/builder#derive",
                                },
                                {
                                    text: "expose_positional_fn",
                                    link: "/v2/reference/builder#expose-positional-fn",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/v2/reference/builder#finish-fn",
                                },
                                {
                                    text: "on",
                                    link: "/v2/reference/builder#on",
                                },
                                {
                                    text: "start_fn",
                                    link: "/v2/reference/builder#start-fn",
                                },
                            ],
                        },
                        {
                            text: "Member attributes",
                            link: "/v2/reference/builder#member-attributes",
                            items: [
                                {
                                    text: "default",
                                    link: "/v2/reference/builder#default",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/v2/reference/builder#finish-fn-1",
                                },
                                {
                                    text: "into",
                                    link: "/v2/reference/builder#into",
                                },
                                {
                                    text: "name",
                                    link: "/v2/reference/builder#name",
                                },
                                {
                                    text: "skip",
                                    link: "/v2/reference/builder#skip",
                                },
                                {
                                    text: "start_fn",
                                    link: "/v2/reference/builder#start-fn-1",
                                },
                            ],
                        },
                    ],
                },
                {
                    text: "#[bon]",
                    link: "/v2/reference/bon",
                },
                {
                    text: "Other items on docs.rs",
                    link: "https://docs.rs/bon/latest/bon/",
                },
            ],
        },
    ],
} satisfies Record<string, DefaultTheme.Sidebar>;
