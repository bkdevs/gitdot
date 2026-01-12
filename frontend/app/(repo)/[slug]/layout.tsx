import { getRepositoryTree } from "@/lib/dal";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment, JSX } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import { RepoDialogs } from "./ui/repo-dialogs";
import { RepoSidebar } from "./ui/repo-sidebar";
import { inferLanguage, parseRepositoryTree } from "./util";

async function renderFilePreviews(files: RepositoryTreeEntry[]): Promise<Map<string, JSX.Element>> {
  return Promise.all(files.map(async (file) => {
    const preview = file.preview;
    if (!preview) return null;
    const hast = await codeToHast(preview, {
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

    const content = (
      <div key={file.path}>
        {
          toJsxRuntime(hast, {
            Fragment,
            jsx,
            jsxs,
          }) as JSX.Element
        }
      </div>
    );

    return [file.path, content];
  })
  ).then((results) => {
    const entries = results.filter((item): item is [string, JSX.Element] => item !== null);
    return new Map(entries);
  });
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
      <RepoDialogs repo={repo} files={files} filePreviewsPromise={filePreviewsPromise}  />
    </>
  );
}
