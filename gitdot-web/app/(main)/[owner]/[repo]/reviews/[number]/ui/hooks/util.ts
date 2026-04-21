export const readLineNumber = (line: HTMLElement): number | undefined => {
  if (line.dataset.lineNumber) return parseInt(line.dataset.lineNumber, 10);
  const lineType = line.dataset.lineType;
  if (lineType === "removed" && line.dataset.leftLineNumber)
    return parseInt(line.dataset.leftLineNumber, 10);
  if (line.dataset.rightLineNumber)
    return parseInt(line.dataset.rightLineNumber, 10);
  if (line.dataset.leftLineNumber)
    return parseInt(line.dataset.leftLineNumber, 10);
  return undefined;
};

export const readSide = (line: HTMLElement): "old" | "new" => {
  const raw = line.dataset.side;
  if (raw === "old") return "old";
  if (raw === "new") return "new";
  return line.dataset.lineType === "removed" ? "old" : "new";
};
