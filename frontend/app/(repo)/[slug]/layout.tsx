import { getRepositoryTree } from "@/lib/dal";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Suspense, Fragment, JSX } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast, codeToHtml } from "shiki";
import { RepoDialogs } from "./ui/repo-dialogs";
import { RepoSidebar } from "./ui/repo-sidebar";
import { inferLanguage, parseRepositoryTree } from "./util";

async function renderFilePreviews(
  files: RepositoryTreeEntry[],
): Promise<Map<string, string>> {
  const renderFile = async (
    file: RepositoryTreeEntry,
  ): Promise<[string, string] | null> => {
    const preview = file.preview;
    if (!preview) return null;

    const html = await codeToHtml(preview, {
      lang: inferLanguage(file.path) ?? "plaintext",
      theme: "vitesse-light",
      transformers: [
        {
          pre(node) {
            this.addClassToHast(node, "outline-none");
          },
        },
      ],
    });
    return [file.path, html];
  };

  return Promise.all(files.map(renderFile))
    .then((results) =>
      results.filter((item): item is [string, any] => item !== null),
    )
    .then((entries) => new Map(entries));
}

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ slug: string }>;
}>) {
  const { slug: repo } = await params;
  const tree = await getRepositoryTree("bkdevs", repo);
  if (!tree) return null;

  const { folders, entries } = parseRepositoryTree(tree);
  const files = Array.from(entries.values()).filter(
    (entry) => entry.entry_type === "blob",
  );
  const filePreviewsPromise = renderFilePreviews(files);

  return (
    <>
      <div className="flex min-h-svh w-full max-w-screen overflow-hidden">
        <RepoSidebar repo={repo} folders={folders} entries={entries} />
        <main className="flex-1 w-full min-w-0 overflow-auto">{children}</main>
      </div>
      <RepoDialogs
        repo={repo}
        files={files}
        filePreviewsPromise={filePreviewsPromise}
      />
    </>
  );
}
