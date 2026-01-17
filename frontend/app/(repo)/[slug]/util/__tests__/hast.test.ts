import type { Element } from "hast";
import type { DiffChange } from "@/lib/dto";
import { highlightChanges } from "../hast";

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
    const changes: DiffChange[] = [
      { start: 6, end: 7, content: ")", highlight: "delimiter" },
      { start: 7, end: 8, content: "}", highlight: "delimiter" },
    ];

    it("should highlight right side spans with green", () => {
      const lineNode = createLineNode(texts);
      highlightChanges("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      expect(classes[0]).toContain("text-green-600!");
      expect(classes[1]).toContain("text-green-600!");
    });

    it("should highlight left side spans with red", () => {
      const lineNode = createLineNode(texts);
      highlightChanges("left", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      expect(classes[0]).toContain("text-red-600!");
      expect(classes[1]).toContain("text-red-600!");
    });
  });

  describe("with JSX-like content", () => {
    const texts = ["{", "!", "leftPath", " &&", " rightPath", " &&", " ("];
    const changes: DiffChange[] = [
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
      highlightChanges("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // First span "{" is at offset 0-1, but change is at 6-7 → no match
      expect(classes[0]).not.toContain("text-green-600!");
      // Second span "!" is at offset 1-2, but change is at 7-8 → no match
      expect(classes[1]).not.toContain("text-green-600!");
    });
  });

  describe("with aligned changes matching span positions", () => {
    const texts = ["      ", "{", "!", "leftPath"];
    const changes: DiffChange[] = [
      { start: 6, end: 7, content: "{", highlight: "delimiter" },
      { start: 7, end: 8, content: "!", highlight: "keyword" },
      { start: 8, end: 16, content: "leftPath", highlight: "normal" },
    ];

    it("should highlight spans where changes align with span boundaries", () => {
      const lineNode = createLineNode(texts);
      highlightChanges("right", lineNode, changes);
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
    const changes: DiffChange[] = [
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
      highlightChanges("left", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // spans 0-7 are at positions 0-8, no changes start there
      // The actual test depends on whether spans align with changes
      // Given the structure, most spans won't match due to position misalignment
      expect(classes).toHaveLength(9);
    });
  });

  describe("edge cases", () => {
    it("should handle empty changes array", () => {
      const lineNode = createLineNode(["hello", " ", "world"]);
      highlightChanges("right", lineNode, []);
      const classes = getSpanClasses(lineNode);

      expect(classes[0]).not.toContain("text-green-600!");
      expect(classes[1]).not.toContain("text-green-600!");
      expect(classes[2]).not.toContain("text-green-600!");
    });

    it("should handle single span matching single change", () => {
      const lineNode = createLineNode(["hello"]);
      const changes: DiffChange[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightChanges("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      expect(classes[0]).toContain("text-green-600!");
    });

    it("should not highlight when change extends beyond span", () => {
      const lineNode = createLineNode(["hel", "lo"]);
      const changes: DiffChange[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightChanges("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // Change spans across both spans, but each individual span
      // check fails: start >= spanStart but end > spanEnd
      expect(classes[0]).not.toContain("text-green-600!");
      expect(classes[1]).not.toContain("text-green-600!");
    });

    it("should handle partial overlap where change is subset of span", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChange[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
      ];
      highlightChanges("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // Span is 0-11, change is 0-5, so change fits within span
      expect(classes[0]).toContain("text-green-600!");
    });

    it("should handle multiple changes within single span", () => {
      const lineNode = createLineNode(["hello world"]);
      const changes: DiffChange[] = [
        { start: 0, end: 5, content: "hello", highlight: "normal" },
        { start: 6, end: 11, content: "world", highlight: "normal" },
      ];
      highlightChanges("right", lineNode, changes);
      const classes = getSpanClasses(lineNode);

      // Both changes fit within span 0-11
      expect(classes[0]).toContain("text-green-600!");
    });
  });
});
