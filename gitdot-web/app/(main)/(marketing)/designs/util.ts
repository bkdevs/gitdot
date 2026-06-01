import fs from "node:fs";
import path from "node:path";
import matter from "gray-matter";

const designsDirectory = path.join(
  process.cwd(),
  "app/(main)/(marketing)/designs/content",
);

export interface DesignMetadata {
  title: string;
  slug: string;
  order: number;
  author: string;
  date: string;
}

export interface Design {
  metadata: DesignMetadata;
  content: string;
}

export function getDesignBySlug(slug: string): Design | null {
  try {
    const filePath = path.join(designsDirectory, `${slug}.md`);
    if (!fs.existsSync(filePath)) {
      return null;
    }

    const fileContents = fs.readFileSync(filePath, "utf8");
    const { data, content } = matter(fileContents);

    return {
      metadata: data as DesignMetadata,
      content,
    };
  } catch (error) {
    console.error(`Error reading design ${slug}:`, error);
    return null;
  }
}

export function getAllDesigns(): Design[] {
  try {
    if (!fs.existsSync(designsDirectory)) {
      return [];
    }

    const fileNames = fs.readdirSync(designsDirectory);
    const designs = fileNames
      .filter((fileName) => fileName.endsWith(".md"))
      .map((fileName) => getDesignBySlug(fileName.replace(".md", "")))
      .filter((design): design is Design => design !== null)
      .sort((a, b) => a.metadata.order - b.metadata.order);

    return designs;
  } catch (error) {
    console.error("Error reading all designs:", error);
    return [];
  }
}

export function getAllSlugs(): string[] {
  try {
    if (!fs.existsSync(designsDirectory)) {
      return [];
    }

    const fileNames = fs.readdirSync(designsDirectory);
    return fileNames
      .filter((fileName) => fileName.endsWith(".md"))
      .map((fileName) => fileName.replace(".md", ""));
  } catch (error) {
    console.error("Error reading design slugs:", error);
    return [];
  }
}
