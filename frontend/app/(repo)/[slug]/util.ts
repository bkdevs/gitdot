import type { RepositoryTreeEntry } from "@/lib/dto";

export function fuzzyMatch(query: string, target: string): boolean {
  const lowerQuery = query.toLowerCase();
  const lowerTarget = target.toLowerCase();
  let queryIndex = 0;

  for (
    let i = 0;
    i < lowerTarget.length && queryIndex < lowerQuery.length;
    i++
  ) {
    if (lowerTarget[i] === lowerQuery[queryIndex]) {
      queryIndex++;
    }
  }

  return queryIndex === lowerQuery.length;
}

export function getMockPreview(entry: RepositoryTreeEntry): string {
  if (entry.entry_type === "tree") {
    return "// Directory\n// Contents preview will be available soon";
  }

  const ext = entry.path.split(".").pop() || "";

  if (ext === "yaml" || ext === "yml") {
    return `# \n\npackages:\n  - example\n\ndependencies:\n  - typescript: ^5.0.0\n  - react: ^19.0.0`;
  }

  if (ext === "json") {
    return `{\n  "name": "",\n  "version": "1.0.0",\n  "description": "Mock preview"\n}`;
  }

  if (ext === "ts" || ext === "tsx" || ext === "js" || ext === "jsx") {
    return `// \n\nexport default function Component() {\n  return <div>Preview coming soon</div>;\n}`;
  }

  if (ext === "md") {
    return `# \n\nThis is a mock preview of the file content.\n\nActual content will be loaded soon.`;
  }

  return `// \n// File preview will be available soon\n// Type: ${entry.entry_type}\n// SHA: ${entry.sha}`;
}
