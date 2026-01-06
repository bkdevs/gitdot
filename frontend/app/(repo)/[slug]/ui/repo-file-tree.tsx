"use client";

import { File, Folder, FolderOpen } from "lucide-react";
import Link from "next/link";
import { useMemo, useState } from "react";
import type { RepositoryTree, RepositoryTreeEntry } from "@/lib/dto";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/ui/collapsible";
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuSub,
} from "@/ui/sidebar";

type TreeNode = RepositoryTreeEntry & {
  children?: TreeNode[];
};

function buildFileTree(entries: RepositoryTreeEntry[]): TreeNode[] {
  const root: TreeNode[] = [];
  const map = new Map<string, TreeNode>();

  entries.forEach((entry) => {
    map.set(entry.path, { ...entry, children: [] });
  });

  // 2. Build relationships
  entries.forEach((entry) => {
    const node = map.get(entry.path);
    if (!node) return;
    const parts = entry.path.split("/");

    if (parts.length === 1) {
      // If it's a root item (no slashes or just one part)
      root.push(node);
    } else {
      // Find the parent folder
      const parentPath = parts.slice(0, -1).join("/");
      const parent = map.get(parentPath);

      if (parent) {
        parent.children?.push(node);
      } else {
        root.push(node);
      }
    }
  });

  const sortNodes = (nodes: TreeNode[]) => {
    nodes.sort((a, b) => {
      const aIsFolder = a.entry_type === "tree";
      const bIsFolder = b.entry_type === "tree";

      if (aIsFolder === bIsFolder) {
        return a.name.localeCompare(b.name);
      }
      return aIsFolder ? -1 : 1;
    });

    nodes.forEach((node) => {
      if (node.children) sortNodes(node.children);
    });
  };

  sortNodes(root);
  return root;
}

function FileTreeNode({ repo, item }: { repo: string; item: TreeNode }) {
  const isFolder = item.entry_type === "tree";
  const [isOpen, setIsOpen] = useState(false);

  if (!isFolder) {
    return (
      <SidebarMenuItem>
        <Link
          href={`/${repo}/${item.path}`}
          className="flex items-center gap-2 w-full py-1 text-sm hover:bg-accent hover:text-accent-foreground rounded-md transition-colors text-left pl-2 cursor-default"
        >
          <File className="h-4 w-4 shrink-0 opacity-70" />
          <span className="truncate">{item.name}</span>
        </Link>
      </SidebarMenuItem>
    );
  }

  return (
    <SidebarMenuItem>
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <button
            type="button"
            className="flex items-center gap-2 w-full px-1 py-1 text-sm hover:bg-accent hover:text-accent-foreground rounded-md transition-colors text-left"
          >
            {isOpen ? (
              <FolderOpen className="h-4 w-4 shrink-0" />
            ) : (
              <Folder className="h-4 w-4 shrink-0" />
            )}
            <span className="font-medium truncate">{item.name}</span>
          </button>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <SidebarMenuSub className="mr-0 pr-0 border-l-slate-200 ml-3.5 border-l">
            {item.children?.map((subItem) => (
              <FileTreeNode key={subItem.path} repo={repo} item={subItem} />
            ))}
          </SidebarMenuSub>
        </CollapsibleContent>
      </Collapsible>
    </SidebarMenuItem>
  );
}

export function RepoFileTree({
  repo,
  tree,
}: {
  repo: string;
  tree: RepositoryTree;
}) {
  const treeStructure = useMemo(() => {
    return tree?.entries ? buildFileTree(tree.entries) : [];
  }, [tree]);

  if (!tree || !tree.entries) return null;

  return (
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          {treeStructure.map((item) => (
            <FileTreeNode key={item.path} repo={repo} item={item} />
          ))}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
