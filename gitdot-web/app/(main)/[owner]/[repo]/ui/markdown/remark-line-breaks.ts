import type { Break, Root } from "mdast";
import { newlineToBreak } from "mdast-util-newline-to-break";
import { visit } from "unist-util-visit";

const BR = /<br\s*\/?>/gi;

// Replaces remark-breaks: soft newlines become hard breaks (one `<br>` each),
// and any literal `<br>` typed in the source becomes a hard break too. Without
// rehype-raw a literal `<br>` parses into an mdast `html` node that gets dropped
// at render, so we rewrite each one into a break node here; stacking them stacks
// the vertical space proportionally.
export default function remarkLineBreaks() {
  return (tree: Root) => {
    newlineToBreak(tree);
    visit(tree, "html", (node, index, parent) => {
      if (index == null || !parent) return;
      const count = node.value.match(BR)?.length ?? 0;
      if (count === 0) return;
      const breaks: Break[] = Array.from({ length: count }, () => ({
        type: "break",
      }));
      parent.children.splice(index, 1, ...(breaks as never[]));
      return index + breaks.length;
    });
  };
}
