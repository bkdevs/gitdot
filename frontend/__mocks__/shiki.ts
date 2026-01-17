import type { Element } from "hast";

export function addClassToHast(node: Element, className: string): Element {
  const props = node.properties ?? {};
  const existingClass = props.class;

  if (Array.isArray(existingClass)) {
    existingClass.push(className);
  } else if (typeof existingClass === "string") {
    props.class = [existingClass, className];
  } else {
    props.class = [className];
  }

  node.properties = props;
  return node;
}
