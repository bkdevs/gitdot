import type { DiffChangeResource } from "gitdot-api";
import type { Element } from "hast";
import { highlightWords } from "../hast";

function createLineNode(texts: string[]): Element {
  return {
    type: "element",
    tagName: "diffline",
    properties: {},
    children: texts.map((text) => ({
      type: "element" as const,
      tagName: "span",
      properties: { class: [] },
      children: [{ type: "text" as const, value: text }],
    })),
  };
}

function getSpanClasses(lineNode: Element): string[][] {
  return lineNode.children.map((child) => {
    if (child.type !== "element") return [];
    const classValue = child.properties?.class;
    if (Array.isArray(classValue)) return classValue as string[];
    if (typeof classValue === "string") return [classValue];
    return [];
  });
}

describe("highlightChanges", () => {
  describe("with closing delimiters: '      )' and '}'", () => {
    const texts = ["      )", "}"];
    const changes: DiffChangeResource[] = [
      { start: 6, end: 7, content: ")", highlight: "delimiter" },
      { start: 7, end: 8, content: "}", highlight: "delimiter" },
    ];

    it("should highlight right side spans with green", () => {
      const lineNode = createLineNode(texts);
      highlightWords("right", lineNode, changes);

      // First span "      )" (0-7) has partial change (6-7), so it gets split
      const span0 = lineNode.children[0];
      if (span0.type !== "element") throw new Error("Expected element");
      expect(span0.children).toHaveLength(2);
      // First child: plain text "      " (positions 0-6)
      expect(span0.children[0]).toEqual({ type: "text", value: "      " });
      // Second child: highlighted span with ")"
      const highlightedChild = span0.children[1];
      expect(highlightedChild.type).toBe("element");
      if (highlightedChild.type === "element") {
        expect(highlightedChild.properties?.class).toContain("text-green-600!");
        expect(highlightedChild.children[0]).toEqual({
          type: "text",
          value: ")",
        });
      }

      // Second span "}" (7-8) exactly matches change (7-8), so whole span highlighted
      const classes = getSpanClasses(lineNode);
      expect(classes[1]).toContain("text-green-600!");
    });

    it("should highlight left side spans with red", () => {
      const lineNode = createLineNode(texts);
      highlightWords("left", lineNode, changes);

      // First span "      )" (0-7) has partial change (6-7), so it gets split
      const span0 = lineNode.children[0];
      if (span0.type !== "element") throw new Error("Expected element");
      expect(span0.children).toHaveLength(2);
      // First child: plain text "      "
      expect(span0.children[0]).toEqual({ type: "text", value: "      " });
      // Second child: highlighted span with ")"
      const highlightedChild = span0.children[1];
      expect(highlightedChild.type).toBe("element");
      if (highlightedChild.type === "element") {
        expect(highlightedChild.properties?.class).toContain("text-red-600!");
        expect(highlightedChild.children[0]).toEqual({
          type: "text",
          value: ")",
        });
      }

      // Second span "}" (7-8) exactly matches change (7-8), so whole span highlighted
      const classes = getSpanClasses(lineNode);
      expect(classes[1]).toContain("text-red-600!");
    });
  });

  describe("with JSX-like content", () => {
    const texts = ["{", "!", "leftPath", " &&", " rightPath", " &&", " ("];
    const changes: DiffChangeResource[] = [
      { start: 6, end: 7, content: "{", highlight: "delimiter" },
      { start: 7, end: 8, content: "!", highlight: "keyword" },
      { start: 8, end: 16, content: "leftPath", highlight: "normal" },
      { start: 17, end: 19, content: "&&", highlight: "keyword" },
      { start: 20, end: 29, content: "rightPath", highlight: "normal" },
      { start: 30, end: 32, content: "&&", highlight: "keyword" },
      { start: 33, end: 34, content: "(", highlight: "delimiter" },
    ];

    it("should only highlight spans where change falls entirely within span bounds", () => {
      const lineNode = createLineNode(texts);
      highlightWords("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // First span "{" is at offset 0-1, but change is at 6-7 → no match
      expect(classes[0]).not.toContain("text-green-600!");
      // Second span "!" is at offset 1-2, but change is at 7-8 → no match
      expect(classes[1]).not.toContain("text-green-600!");
    });
  });

  describe("with aligned changes matching span positions", () => {
    const texts = ["      ", "{", "!", "leftPath"];
    const changes: DiffChangeResource[] = [
      { start: 6, end: 7, content: "{", highlight: "delimiter" },
      { start: 7, end: 8, content: "!", highlight: "keyword" },
      { start: 8, end: 16, content: "leftPath", highlight: "normal" },
    ];

    it("should highlight spans where changes align with span boundaries", () => {
      const lineNode = createLineNode(texts);
      highlightWords("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // "      " at 0-6: no change starts at 0
      expect(classes[0]).not.toContain("text-green-600!");
      // "{" at 6-7: change (6,7) matches
      expect(classes[1]).toContain("text-green-600!");
      // "!" at 7-8: change (7,8) matches
      expect(classes[2]).toContain("text-green-600!");
      // "leftPath" at 8-16: change (8,16) matches
      expect(classes[3]).toContain("text-green-600!");
    });
  });

  describe("with JSX span element", () => {
    // Span positions:
    // 0: "        <span" (0-13), 1: " className" (13-23), 2: "=" (23-24),
    // 3: '"' (24-25), 4: "ml-1.5 text-green-600" (25-46), 5: '"' (46-47),
    // 6: ">" (47-48), 7: "(created)" (48-57), 8: "</span>" (57-64)
    const texts = [
      "        <span",
      " className",
      "=",
      '"',
      "ml-1.5 text-green-600",
      '"',
      ">",
      "(created)",
      "</span>",
    ];
    const changes: DiffChangeResource[] = [
      { start: 8, end: 9, content: "<", highlight: "delimiter" },
      { start: 9, end: 13, content: "span", highlight: "normal" },
      { start: 14, end: 23, content: "className", highlight: "normal" },
      { start: 23, end: 24, content: "=", highlight: "keyword" },
      {
        start: 24,
        end: 47,
        content: '"ml-1.5 text-green-600"',
        highlight: "string",
      },
      { start: 47, end: 48, content: ">", highlight: "delimiter" },
      { start: 48, end: 57, content: "(created)", highlight: "normal" },
      { start: 57, end: 59, content: "</", highlight: "normal" },
      { start: 59, end: 63, content: "span", highlight: "normal" },
      { start: 63, end: 64, content: ">", highlight: "keyword" },
    ];

    it("should handle complex JSX with multiple spans", () => {
      const lineNode = createLineNode(texts);
      highlightWords("left", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      const span0 = lineNode.children[0];
      if (span0.type !== "element") throw new Error("Expected element");
      expect(span0.children).toHaveLength(3); // "        " + "<" + "span"

      const span1 = lineNode.children[1];
      if (span1.type !== "element") throw new Error("Expected element");
      expect(span1.children).toHaveLength(2); // " " + "className"

      // Span 2 "=" (23-24): exact match
      expect(classes[2]).toContain("text-red-600!");

      // Spans 3-5: The string change (24-47) is split by processChanges into
      // opening quote (24-25), content (25-46), closing quote (46-47)
      expect(classes[3]).toContain("text-red-600!"); // '"'
      expect(classes[4]).toContain("text-red-600!"); // "ml-1.5 text-green-600"
      expect(classes[5]).toContain("text-red-600!"); // '"'

      // Span 6 ">" (47-48): exact match
      expect(classes[6]).toContain("text-red-600!");

      // Span 7 "(created)" (48-57): exact match
      expect(classes[7]).toContain("text-red-600!");

      // Span 8 "</span>" (57-64): changes (57-59), (59-63), (63-64) are partial → split
      const span8 = lineNode.children[8];
      if (span8.type !== "element") throw new Error("Expected element");
      expect(span8.children).toHaveLength(3); // "</" + "span" + ">"
    });
  });

  describe("edge cases", () => {
    it("should handle empty changes array", () => {
      const lineNode = createLineNode(["hello", " ", "world"]);
      highlightWords("right", lineNode, []);
      const classes = getSpanClasses(lineNode);

      expect(classes[0]).not.toContain("text-green-600!");
      expect(classes[1]).not.toContain("text-green-600!");
      expect(classes[2]).not.toContain("text-green-600!");
    });

    it("should handle single span matching single change", () => {
      const lineNode = createLineNode(["hello"]);
      const changes: DiffChangeResource[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      expect(classes[0]).toContain("text-green-600!");
    });

    it("should not highlight when change extends beyond span", () => {
      const lineNode = createLineNode(["hel", "lo"]);
      const changes: DiffChangeResource[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // Change spans across both spans, but each individual span
      // check fails: start >= spanStart but end > spanEnd
      expect(classes[0]).not.toContain("text-green-600!");
      expect(classes[1]).not.toContain("text-green-600!");
    });

    it("should handle partial overlap where change is subset of span", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChangeResource[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);

      // Span should be split: highlighted "hello" + plain " world"
      const span = lineNode.children[0];
      if (span.type !== "element") throw new Error("Expected element");
      expect(span.children).toHaveLength(2);

      // First child: highlighted span with "hello"
      const highlightedChild = span.children[0];
      expect(highlightedChild.type).toBe("element");
      if (highlightedChild.type === "element") {
        expect(highlightedChild.properties?.class).toContain("text-green-600!");
        expect(highlightedChild.children[0]).toEqual({
          type: "text",
          value: "hello",
        });
      }

      // Second child: plain text " world"
      expect(span.children[1]).toEqual({ type: "text", value: " world" });
    });

    it("should handle multiple changes within single span", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChangeResource[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
        { start: 6, end: 11, content: "world", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);

      // Span should be split: highlighted "hello" + plain " " + highlighted "world"
      const span = lineNode.children[0];
      if (span.type !== "element") throw new Error("Expected element");
      expect(span.children).toHaveLength(3);

      // First child: highlighted span with "hello"
      const firstChild = span.children[0];
      expect(firstChild.type).toBe("element");
      if (firstChild.type === "element") {
        expect(firstChild.properties?.class).toContain("text-green-600!");
        expect(firstChild.children[0]).toEqual({
          type: "text",
          value: "hello",
        });
      }

      // Second child: plain text " "
      expect(span.children[1]).toEqual({ type: "text", value: " " });

      // Third child: highlighted span with "world"
      const thirdChild = span.children[2];
      expect(thirdChild.type).toBe("element");
      if (thirdChild.type === "element") {
        expect(thirdChild.properties?.class).toContain("text-green-600!");
        expect(thirdChild.children[0]).toEqual({
          type: "text",
          value: "world",
        });
      }
    });

    it("should highlight only the end of a span when change is at end", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChangeResource[] = [
        { start: 6, end: 11, content: "world", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);

      // Span should be split: plain "hello " + highlighted "world"
      const span = lineNode.children[0];
      if (span.type !== "element") throw new Error("Expected element");
      expect(span.children).toHaveLength(2);

      // First child: plain text "hello "
      expect(span.children[0]).toEqual({ type: "text", value: "hello " });

      // Second child: highlighted span with "world"
      const highlightedChild = span.children[1];
      expect(highlightedChild.type).toBe("element");
      if (highlightedChild.type === "element") {
        expect(highlightedChild.properties?.class).toContain("text-green-600!");
        expect(highlightedChild.children[0]).toEqual({
          type: "text",
          value: "world",
        });
      }
    });

    it("should highlight only the middle of a span when change is in middle", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChangeResource[] = [
        { start: 3, end: 8, content: "lo wo", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);

      // Span should be split: plain "hel" + highlighted "lo wo" + plain "rld"
      const span = lineNode.children[0];
      if (span.type !== "element") throw new Error("Expected element");
      expect(span.children).toHaveLength(3);

      // First child: plain text "hel"
      expect(span.children[0]).toEqual({ type: "text", value: "hel" });

      // Second child: highlighted span with "lo wo"
      const highlightedChild = span.children[1];
      expect(highlightedChild.type).toBe("element");
      if (highlightedChild.type === "element") {
        expect(highlightedChild.properties?.class).toContain("text-green-600!");
        expect(highlightedChild.children[0]).toEqual({
          type: "text",
          value: "lo wo",
        });
      }

      // Third child: plain text "rld"
      expect(span.children[2]).toEqual({ type: "text", value: "rld" });
    });

    it("should highlight whole span when change equals span exactly", () => {
      const lineNode = createLineNode(["hello"]);
      const changes: DiffChangeResource[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightWords("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // Change matches span exactly, so whole span should be highlighted
      // (no child spans created)
      expect(classes[0]).toContain("text-green-600!");

      // Verify structure: span should still have a single text child
      const span = lineNode.children[0];
      if (span.type !== "element") throw new Error("Expected element");
      expect(span.children).toHaveLength(1);
      expect(span.children[0]).toEqual({ type: "text", value: "hello" });
    });

    it("should use red highlight for left side partial changes", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChangeResource[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightWords("left", lineNode, changes);

      // Verify the highlighted child span uses red
      const span = lineNode.children[0];
      if (span.type !== "element") throw new Error("Expected element");
      const highlightedChild = span.children[0];
      expect(highlightedChild.type).toBe("element");
      if (highlightedChild.type === "element") {
        expect(highlightedChild.properties?.class).toContain("text-red-600!");
      }
    });
  });
});
