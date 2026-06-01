import fs from "node:fs";
import path from "node:path";
import matter from "gray-matter";

const releasesDirectory = path.join(
  process.cwd(),
  "app/(main)/(marketing)/releases/content",
);

export interface ReleaseMetadata {
  title: string;
  date: string;
  version: string;
}

export interface Release {
  metadata: ReleaseMetadata;
  content: string;
}

export function getReleaseByVersion(version: string): Release | null {
  try {
    const filePath = path.join(releasesDirectory, `${version}.md`);
    if (!fs.existsSync(filePath)) {
      return null;
    }

    const fileContents = fs.readFileSync(filePath, "utf8");
    const { data, content } = matter(fileContents);

    return {
      metadata: data as ReleaseMetadata,
      content,
    };
  } catch (error) {
    console.error(`Error reading release ${version}:`, error);
    return null;
  }
}

export function getAllReleases(): Release[] {
  try {
    if (!fs.existsSync(releasesDirectory)) {
      return [];
    }

    const fileNames = fs.readdirSync(releasesDirectory);
    const releases = fileNames
      .filter((fileName) => fileName.endsWith(".md"))
      .map((fileName) => getReleaseByVersion(fileName.replace(".md", "")))
      .filter((release): release is Release => release !== null)
      .sort((a, b) =>
        b.metadata.version.localeCompare(a.metadata.version, undefined, {
          numeric: true,
        }),
      );

    return releases;
  } catch (error) {
    console.error("Error reading all releases:", error);
    return [];
  }
}

export function getAllVersions(): string[] {
  try {
    if (!fs.existsSync(releasesDirectory)) {
      return [];
    }

    const fileNames = fs.readdirSync(releasesDirectory);
    return fileNames
      .filter((fileName) => fileName.endsWith(".md"))
      .map((fileName) => fileName.replace(".md", ""));
  } catch (error) {
    console.error("Error reading release versions:", error);
    return [];
  }
}
