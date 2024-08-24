import { DefaultTheme } from "vitepress";

export const sidebars = {
    guide: [
        {
            text: "Guide",
            items: [
                {
                    text: "Overview",
                    link: "/v1/guide/overview",
                },
                {
                    text: "Optional members",
                    link: "/v1/guide/optional-members",
                },
                {
                    text: "Compatibility",
                    link: "/v1/guide/compatibility",
                },
                {
                    text: "Into conversions",
                    link: "/v1/guide/into-conversions",
                },
                {
                    text: "Documenting",
                    link: "/v1/guide/documenting",
                },
                {
                    text: "Limitations",
                    link: "/v1/guide/limitations",
                },
                {
                    text: "Benchmarks",
                    link: "/v1/guide/benchmarks",
                },
                {
                    text: "Alternatives",
                    link: "/v1/guide/alternatives",
                },
                {
                    text: "Troubleshooting",
                    link: "/v1/guide/troubleshooting",
                },
            ],
        },
    ],
    reference: [
        {
            text: "Reference",
            items: [
                {
                    text: "#[builder]",
                    link: "/v1/reference/builder",
                    items: [
                        {
                            text: "Top-level attributes",
                            link: "/v1/reference/builder#top-level-attributes",
                            items: [
                                {
                                    text: "builder_type",
                                    link: "/v1/reference/builder#builder-type",
                                },
                                {
                                    text: "expose_positional_fn",
                                    link: "/v1/reference/builder#expose-positional-fn",
                                },
                                {
                                    text: "finish_fn",
                                    link: "/v1/reference/builder#finish-fn",
                                },
                                {
                                    text: "start_fn",
                                    link: "/v1/reference/builder#start-fn",
                                },
                            ],
                        },
                        {
                            text: "Member-level attributes",
                            link: "/v1/reference/builder#member-level-attributes",
                            items: [
                                {
                                    text: "default",
                                    link: "/v1/reference/builder#default",
                                },
                                {
                                    text: "into",
                                    link: "/v1/reference/builder#into",
                                },
                                {
                                    text: "name",
                                    link: "/v1/reference/builder#name",
                                },
                                {
                                    text: "skip",
                                    link: "/v1/reference/builder#skip",
                                },
                            ],
                        },
                    ],
                },
                {
                    text: "#[bon]",
                    link: "/v1/reference/bon",
                },
                {
                    text: "Other items on docs.rs (v1)",
                    link: "https://docs.rs/bon/1/bon/",
                },
            ],
        },
    ],
} satisfies Record<string, DefaultTheme.Sidebar>;
