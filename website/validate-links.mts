/**
 * This script was copied almost verbatim from https://github.com/hyperledger/iroha-2-docs/blob/c1520e4cb06216428e21b4b7305e29247bdf0737/etc/validate-links.ts#L1
 * The only custom edits done to this file are the ability to run it as a standalone script
 * and use `file.html` search paths instead of `file/index.html`.
 *
 * Huge thanks to the author of this script 💓: https://github.com/0x009922
 */

import { readFile } from "fs/promises";
import chalk from "chalk";
import { P, match } from "ts-pattern";
import { globby } from "globby";
import path from "path";
import * as htmlparser from "htmlparser2";
import * as cssSelect from "css-select";
import leven from "leven";
import fastDiff from "fast-diff";

interface Options {
    root: string;
    publicPath?: string;
}

type LinkIssues = Map<string, LinkIssue[]>;

type LinkIssue =
    | IssueMissingOtherFile
    | IssueMissingAnchorInOtherFile
    | IssueMissingAnchorInSelf;

interface IssueMissingOtherFile {
    type: "missing-other-file";
    file: string;
}

interface IssueMissingAnchorInOtherFile {
    type: "missing-id-in-other";
    file: string;
    id: string;
    similar?: string[];
}

interface IssueMissingAnchorInSelf {
    type: "missing-id-in-self";
    id: string;
    similar?: string[];
}

export async function scanAndReport(options: Options) {
    const issues = await scan(options);

    const count = countIssues(issues);
    if (count === 0) {
        console.log(chalk`{green ✓ Haven't detected any broken links}`);
    } else {
        const sortByFileName = <T extends [string, unknown]>(
            items: T[],
        ): T[] => {
            const arr = [...items];
            arr.sort(([a], [b]) => {
                return a < b ? -1 : a > b ? 1 : 0;
            });
            return arr;
        };

        const formatFile = (file: string): string => {
            const relative = path.relative(options.root, file);
            const ext = path.extname(relative);
            return chalk`${relative.slice(0, -ext.length)}{reset.dim ${ext}}`;
        };

        const formatSimilar = (origin: string, similar?: string[]) => {
            if (!similar?.length) return "";

            // format diffs
            const diffs = similar.map((x) => {
                const diff = fastDiff(origin, x)
                    .map(([kind, piece]) => {
                        return match(kind)
                            .with(fastDiff.EQUAL, () => chalk.gray.dim(piece))
                            .with(fastDiff.INSERT, () =>
                                chalk.bgGreenBright.black(piece),
                            )
                            .with(fastDiff.DELETE, () =>
                                chalk.bgRedBright.black(piece),
                            )
                            .exhaustive();
                    })
                    .join("");

                return chalk`{blue ${x}} {gray.dim (diff: ${diff}})`;
            });

            return `. Here are similar ones:\n      ` + diffs.join("\n      ");
        };

        const formattedIssues = sortByFileName([...issues])
            .map(([file, issues]) => {
                const issuesFormatted: string = issues
                    .map((issue) => {
                        return (
                            match(issue)
                                // actually, it should never happen: VitePress disallows dead links to other pages
                                .with({ type: "missing-other-file" }, (x) => {
                                    return (
                                        chalk`  Broken link: {bold.red ${formatFile(x.file)}}\n    ` +
                                        chalk`{red Cannot find the file.}`
                                    );
                                })
                                .with({ type: "missing-id-in-other" }, (x) => {
                                    return (
                                        chalk`  Broken link: {bold ${formatFile(x.file)}{red #${x.id}}}` +
                                        `\n    ` +
                                        chalk`{red Cannot find the ID in the other file}` +
                                        formatSimilar(x.id, x.similar)
                                    );
                                })
                                .with({ type: "missing-id-in-self" }, (x) => {
                                    return (
                                        chalk`  Broken link: {bold.red #${x.id}}\n    ` +
                                        chalk`{red Cannot find the ID within the file itself}` +
                                        formatSimilar(x.id, x.similar)
                                    );
                                })
                                .exhaustive()
                        );
                    })
                    .join("\n\n");

                return `${chalk.underline.bold(formatFile(file))} (issues: ${issues.length})\n${issuesFormatted}`;
            })
            .join("\n\n");

        console.error(
            `${formattedIssues}\n\n${chalk.red(`× Found broken links. Total issues count: ${chalk.bold(count)}`)}`,
        );
        process.exit(1);
    }
}

function countIssues(issues: LinkIssues): number {
    let count = 0;
    for (const x of issues.values()) {
        count += x.length;
    }
    return count;
}

async function scan(options: Options): Promise<LinkIssues> {
    const files = await findFiles(options.root);

    const entries = await Promise.all(
        files.map(async (file) => {
            const html = await readFile(file, { encoding: "utf-8" });
            const { links, anchors } = scanLinksAndAnchorsInHTML(html);

            const parsedLinks = links
                .map((x) =>
                    parseLink({
                        root: options.root,
                        source: file,
                        href: x,
                        publicPath: options.publicPath,
                    }),
                )
                .filter(
                    (x): x is Exclude<ParsedLink, LinkExternal> =>
                        x.type !== "external",
                );

            return { file, links: parsedLinks, anchors };
        }),
    );

    const graph = entries.reduce<Graph>((acc, { file, links, anchors }) => {
        acc.set(file, { links, anchors });
        return acc;
    }, new Map());

    return findIssuesInGraph(graph);
}

async function findFiles(root: string): Promise<string[]> {
    return globby(path.join(root, "**/*.html"));
}

const ANCHORS_QUERY = cssSelect.compile("main [id]");

const LINKS_QUERY = cssSelect.compile("main a[href]");

/**
 * TODO: Here we only look into `<main>`. There are also links in `<aside>` and `<header>`, but unlike `<main>`, they
 *       repeat from page to page. Current scan-validate logic doesn't handle such repetition and the report will
 *       look cumbersome.
 */
function scanLinksAndAnchorsInHTML(html: string): {
    links: string[];
    anchors: Set<string>;
} {
    const doc = htmlparser.parseDocument(html);

    const links = cssSelect.selectAll(LINKS_QUERY, doc.children).map((elem) => {
        return match(elem)
            .with(
                { name: "a", attribs: { href: P.select(P.string) } },
                (href) => href,
            )
            .otherwise(() => {
                throw new Error("unexpected <a>");
            });
    });

    const anchors = new Set(
        cssSelect.selectAll(ANCHORS_QUERY, doc.children).map((elem) => {
            return match(elem)
                .with({ attribs: { id: P.select(P.string) } }, (id) => id)
                .otherwise(() => {
                    throw new Error("unexpected element");
                });
        }),
    );

    return { links, anchors };
}

type ParsedLink = LinkSelfAnchor | LinkOtherFile | LinkExternal;

export interface LinkSelfAnchor {
    type: "self";
    anchor: string;
}

export interface LinkOtherFile {
    type: "other";
    file: string;
    anchor?: string;
}

export interface LinkExternal {
    type: "external";
    url: URL;
}

// export for tests
export function parseLink(opts: {
    root: string;
    source: string;
    href: string;
    publicPath?: string;
}): ParsedLink {
    const relative = path.relative(opts.root, opts.source);
    const DUMMY_ORIGIN = "http://dummy.dummy";
    const url = new URL(
        opts.href,
        DUMMY_ORIGIN + (opts.publicPath ?? "/") + `${relative}`,
    );

    return match(url)
        .with(
            {
                origin: DUMMY_ORIGIN,
                pathname: P.select("pathname"),
                hash: P.select("hash"),
            },
            ({ pathname, hash }): ParsedLink => {
                const anchor = hash ? hash.slice(1) : undefined;

                let pathProcessed = pathname;
                if (
                    opts.publicPath &&
                    pathProcessed.startsWith(opts.publicPath)
                ) {
                    pathProcessed = pathProcessed.slice(opts.publicPath.length);
                }
                let file = path.join(opts.root, pathProcessed);
                if (path.extname(file) !== ".html") file = `${file}.html`;

                if (path.normalize(file) === path.normalize(opts.source)) {
                    if (!anchor)
                        throw new Error(
                            `found self link without anchor: ${opts.source}`,
                        );
                    return { type: "self", anchor };
                }

                if (!file) throw new Error("STOP");

                return {
                    type: "other",
                    file,
                    anchor,
                };
            },
        )
        .otherwise((url): ParsedLink => {
            return {
                type: "external",
                url,
            };
        });
}

type Graph = Map<
    string,
    { links: Exclude<ParsedLink, LinkExternal>[]; anchors: Set<string> }
>;

function findIssuesInGraph(graph: Graph): LinkIssues {
    const issues: LinkIssues = new Map();

    for (const [file, { links, anchors: selfAnchors }] of graph) {
        const fileIssues: LinkIssue[] = [];

        for (const link of links) {
            match(link)
                .with({ type: "self" }, ({ anchor }) => {
                    if (!selfAnchors.has(anchor))
                        fileIssues.push({
                            type: "missing-id-in-self",
                            id: anchor,
                            similar: findSimilarIds(selfAnchors, anchor),
                        });
                })
                .with({ type: "other" }, ({ file: otherFile, anchor }) => {
                    const otherFileInGraph = graph.get(otherFile);
                    if (!otherFileInGraph) {
                        fileIssues.push({
                            type: "missing-other-file",
                            file: otherFile,
                        });
                    } else if (
                        anchor &&
                        !otherFileInGraph.anchors.has(anchor)
                    ) {
                        fileIssues.push({
                            type: "missing-id-in-other",
                            file: otherFile,
                            id: anchor,
                            similar: findSimilarIds(
                                otherFileInGraph.anchors,
                                anchor,
                            ),
                        });
                    }
                })
                // eslint-disable-next-line @typescript-eslint/no-empty-function
                .otherwise(() => {});
        }

        if (fileIssues.length) issues.set(file, fileIssues);
    }

    return issues;
}

function findSimilarIds(existing: Set<string>, id: string): string[] {
    const withDist = [...existing]
        .map((x) => ({ dist: leven(x, id), id: x }))
        .filter(({ dist }) => dist >= 1 && dist <= 3);

    withDist.sort((a, b) => a.dist - b.dist);

    return withDist.map(({ id }) => id);
}

scanAndReport({
    root: ".vitepress/dist",
});
