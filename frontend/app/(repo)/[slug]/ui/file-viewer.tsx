"use client";

import { useState } from "react";

const fileStructure: Record<
  string,
  Array<{ name: string; type: "folder" | "file" }>
> = {
  root: [
    { name: "app", type: "folder" },
    { name: "components", type: "folder" },
    { name: "lib", type: "folder" },
    { name: "public", type: "folder" },
    { name: "README.md", type: "file" },
    { name: "package.json", type: "file" },
    { name: "next.config.ts", type: "file" },
    { name: "tsconfig.json", type: "file" },
  ],
  app: [
    { name: "(auth)", type: "folder" },
    { name: "(repo)", type: "folder" },
    { name: "ui", type: "folder" },
    { name: "layout.tsx", type: "file" },
    { name: "page.tsx", type: "file" },
    { name: "globals.css", type: "file" },
  ],
  "(auth)": [
    { name: "login", type: "folder" },
    { name: "signup", type: "folder" },
    { name: "ui", type: "folder" },
  ],
  "(repo)": [{ name: "[slug]", type: "folder" }],
  "[slug]": [
    { name: "issues", type: "folder" },
    { name: "pulls", type: "folder" },
    { name: "ui", type: "folder" },
    { name: "layout.tsx", type: "file" },
    { name: "page.tsx", type: "file" },
  ],
  ui: [
    { name: "file-header.tsx", type: "file" },
    { name: "file-viewer.tsx", type: "file" },
    { name: "repo-commits.tsx", type: "file" },
    { name: "repo-sidebar.tsx", type: "file" },
  ],
  components: [
    { name: "ui", type: "folder" },
    { name: "header.tsx", type: "file" },
    { name: "footer.tsx", type: "file" },
  ],
  lib: [
    { name: "utils.ts", type: "file" },
    { name: "hooks.ts", type: "file" },
    { name: "constants.ts", type: "file" },
  ],
  public: [
    { name: "favicon.ico", type: "file" },
    { name: "vercel.svg", type: "file" },
    { name: "images", type: "folder" },
  ],
};

const filePreviews: Record<string, string> = {
  "layout.tsx": `export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased">
        {children}
      </body>
    </html>
  );
}`,
  "page.tsx": `export default function Page() {
  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold">
        Welcome to the page
      </h1>
    </div>
  );
}`,
  "utils.ts": `export function cn(...inputs: string[]) {
  return inputs.filter(Boolean).join(' ');
}

export function formatDate(date: Date) {
  return new Intl.DateTimeFormat('en-US').format(date);
}`,
  "README.md": `# Project Name

This is a sample README file for the project.

## Getting Started

Run \`npm install\` to install dependencies.`,
  "package.json": `{
  "name": "gitdot-web",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build"
  }
}`,
};

export function FileViewer() {
  const [selectedLeft, setSelectedLeft] = useState("app");
  const [selectedMiddle, setSelectedMiddle] = useState("layout.tsx");

  const leftItems = fileStructure.root;
  const middleItems = fileStructure[selectedLeft] || [];
  const preview = filePreviews[selectedMiddle] || "// No preview available";

  const handleLeftClick = (name: string) => {
    setSelectedLeft(name);
    setSelectedMiddle("");
  };

  const handleMiddleClick = (name: string) => {
    setSelectedMiddle(name);
  };

  return (
    <div className="w-full h-full flex">
      {/* Left column */}
      <div className="w-48 h-full border-r overflow-auto">
        {leftItems.map((item) => (
          <button
            key={item.name}
            type="button"
            onClick={() => handleLeftClick(item.name)}
            className={`w-full px-3 py-1.5 text-sm text-left border-b font-mono hover:bg-accent/50 transition-colors ${
              item.name === selectedLeft ? "bg-accent" : ""
            }`}
          >
            {item.type === "folder" ? `${item.name}/` : item.name}
          </button>
        ))}
      </div>

      {/* Middle column */}
      <div className="w-48 h-full border-r overflow-auto">
        {middleItems.map((item) => (
          <button
            key={item.name}
            type="button"
            onClick={() => handleMiddleClick(item.name)}
            className={`w-full px-3 py-1.5 text-sm text-left border-b font-mono hover:bg-accent/50 transition-colors ${
              item.name === selectedMiddle ? "bg-accent" : ""
            }`}
          >
            {item.type === "folder" ? `${item.name}/` : item.name}
          </button>
        ))}
      </div>

      {/* Right column - preview */}
      <div className="flex-1 h-full p-3 overflow-auto">
        <pre className="text-sm font-mono whitespace-pre text-muted-foreground">
          {preview}
        </pre>
      </div>
    </div>
  );
}
