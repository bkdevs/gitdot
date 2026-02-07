import fs from "node:fs";
import path from "node:path";
import matter from "gray-matter";

const postsDirectory = path.join(process.cwd(), "app/(blog)/content/week");

export interface PostMetadata {
  title: string;
  date: string;
  week: number;
}

export interface Post {
  metadata: PostMetadata;
  content: string;
}

export function getPostByWeek(number: number): Post | null {
  try {
    const filePath = path.join(postsDirectory, `${number}.md`);
    if (!fs.existsSync(filePath)) {
      return null;
    }

    const fileContents = fs.readFileSync(filePath, "utf8");
    const { data, content } = matter(fileContents);

    return {
      metadata: data as PostMetadata,
      content,
    };
  } catch (error) {
    console.error(`Error reading post ${number}:`, error);
    return null;
  }
}

export function getAllPosts(): Post[] {
  try {
    if (!fs.existsSync(postsDirectory)) {
      return [];
    }

    const fileNames = fs.readdirSync(postsDirectory);
    const posts = fileNames
      .filter((fileName) => fileName.endsWith(".md"))
      .map((fileName) => {
        const weekNumber = parseInt(fileName.replace(".md", ""), 10);
        return getPostByWeek(weekNumber);
      })
      .filter((post): post is Post => post !== null)
      .sort((a, b) => b.metadata.week - a.metadata.week);

    return posts;
  } catch (error) {
    console.error("Error reading all posts:", error);
    return [];
  }
}

export function getAllWeeks(): number[] {
  try {
    if (!fs.existsSync(postsDirectory)) {
      return [];
    }

    const fileNames = fs.readdirSync(postsDirectory);
    return fileNames
      .filter((fileName) => fileName.endsWith(".md"))
      .map((fileName) => parseInt(fileName.replace(".md", ""), 10))
      .filter((num) => !Number.isNaN(num))
      .sort((a, b) => a - b);
  } catch (error) {
    console.error("Error reading week numbers:", error);
    return [];
  }
}
