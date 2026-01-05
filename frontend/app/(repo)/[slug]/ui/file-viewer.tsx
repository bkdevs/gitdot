import { getRepositoryFile } from "@/lib/dal";
import { codeToHtml } from "shiki";

export async function FileViewer() {
  const file = await getRepositoryFile("bkdevs", "gitdot", { path: "backend/src/main.rs" });
  if (!file) {
    return null;
  }

  const html = await codeToHtml(file.content, {
    lang: "rust",
    theme: "github-light"
  })


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
