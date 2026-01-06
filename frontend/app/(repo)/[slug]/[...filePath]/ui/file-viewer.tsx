import { codeToHtml } from "shiki";
import { getRepositoryFile } from "@/lib/dal";
import { inferLanguage } from "@/util";

export async function FileViewer({
  repo,
  filePath,
}: {
  repo: string;
  filePath: string;
}) {
  const file = await getRepositoryFile("bkdevs", repo, { path: filePath });
  if (!file) {
    return <div>File not found.</div>;
  }

  const html = await codeToHtml(file.content, {
    lang: inferLanguage(filePath) ?? "plaintext",
    themes: {
      light: "vitesse-light",
      dark: "vitesse-dark",
    },
  });

  return (
    <div className="w-full h-full overflow-auto px-2">
      <div
        className="text-sm"
        // biome-ignore lint: required for shiki
        dangerouslySetInnerHTML={{ __html: html }}
      />
    </div>
  );
}
