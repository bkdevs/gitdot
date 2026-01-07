"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";

import { formatLineSelection, type LineSelection } from "../util";

type FileViewerContextType = {
  isLineSelected: (lineNumber: number) => boolean;
  handleLineMouseDown: (lineNumber: number) => void;
  handleLineMouseEnter: (lineNumber: number) => void;
};

const FileViewerContext = createContext<FileViewerContextType | null>(null);

export function useFileViewer() {
  const context = useContext(FileViewerContext);
  if (!context) {
    throw new Error("useFileViewer must be used within FileViewerClient");
  }
  return context;
}

export function FileViewerClient({
  children,
  selectedLines: initialSelectedLines,
}: {
  children: React.ReactNode;
  selectedLines: LineSelection | null;
}) {
  const [selectedLines, setSelectedLines] = useState<LineSelection | null>(
    initialSelectedLines,
  );
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState<number | null>(null);

  useEffect(() => {
    if (selectedLines) {
      setTimeout(() => {
        const element = document.querySelector(
          `[data-line-number="${selectedLines.start}"]`,
        );
        if (element) {
          element.scrollIntoView({ behavior: "smooth", block: "center" });
        }
      }, 100);
    }
  }, [selectedLines]);

  const updateUrl = useCallback((selection: LineSelection | null) => {
    const params = new URLSearchParams(window.location.search);

    if (selection) {
      params.set("lines", formatLineSelection(selection));
    } else {
      params.delete("lines");
    }

    const newUrl = params.toString()
      ? `${window.location.pathname}?${params.toString()}`
      : window.location.pathname;

    window.history.replaceState(null, "", newUrl);
  }, []);

  const handleLineMouseDown = useCallback((lineNumber: number) => {
    setIsDragging(true);
    setDragStart(lineNumber);
    setSelectedLines({ start: lineNumber, end: lineNumber });
  }, []);

  const handleLineMouseEnter = useCallback(
    (lineNumber: number) => {
      if (!isDragging || dragStart === null) return;

      const start = Math.min(dragStart, lineNumber);
      const end = Math.max(dragStart, lineNumber);
      setSelectedLines({ start, end });
    },
    [isDragging, dragStart],
  );

  useEffect(() => {
    const handleMouseUp = () => {
      if (isDragging) {
        setIsDragging(false);
        if (selectedLines) {
          updateUrl(selectedLines);
        }
      }
    };
    document.addEventListener("mouseup", handleMouseUp);
    return () => document.removeEventListener("mouseup", handleMouseUp);
  }, [isDragging, selectedLines, updateUrl]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && selectedLines) {
        setSelectedLines(null);
        setDragStart(null);
        updateUrl(null);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [selectedLines, updateUrl]);

  const isLineSelected = useCallback(
    (lineNumber: number): boolean => {
      if (!selectedLines) return false;
      return (
        lineNumber >= selectedLines.start && lineNumber <= selectedLines.end
      );
    },
    [selectedLines],
  );

  return (
    <FileViewerContext.Provider
      value={{
        isLineSelected,
        handleLineMouseDown,
        handleLineMouseEnter,
      }}
    >
      {children}
    </FileViewerContext.Provider>
  );
}
