"use client";

import { Circle } from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useTypewriter } from "@/hooks/use-typewriter";

const DOT_FADE_MS = 400;
const POST_TITLE_PAUSE_MS = 500;
const PARAGRAPH_EXPAND_MS = 300;
const POST_EXPAND_PAUSE_MS = 300;

const NARROW_WIDTH = "6rem";
const WIDE_WIDTH = "36rem";
const LINE_1 = "gitdot is a home for great code.";
const LINE_2 = `It is built by developers, for developers, and makes no apologies for putting quality first.\nWe believe that software still counts. Things are changing, but building great software is just as hard and just as valuable as ever before.`;
const LINE_3 =
  "It's early, and there's much left to build, but thank you for being here.";
const SIGNATURE = "— baepaul & mikkelk";
const PARAGRAPH = `${LINE_1}${LINE_2}${LINE_3}`;

const LINE_2_START = LINE_1.length;
const LINE_2_END = LINE_2_START + LINE_2.length;
const LINE_3_START = LINE_2_END;

export default function Page() {
  const router = useRouter();
  const [dotVisible, setDotVisible] = useState(false);
  const [titleVisible, setTitleVisible] = useState(false);
  const [paragraphExpanded, setParagraphExpanded] = useState(false);
  const [paragraphVisible, setParagraphVisible] = useState(false);
  const [nextVisible, setNextVisible] = useState(false);

  const title = useTypewriter(titleVisible ? "Welcome." : "", 25);
  const titleDone = titleVisible && title === "Welcome.";
  const paragraph = useTypewriter(paragraphVisible ? PARAGRAPH : "", 12);
  const paragraphDone = paragraphVisible && paragraph === PARAGRAPH;

  useEffect(() => {
    const fadeIn = setTimeout(() => setDotVisible(true), 50);
    const begin = setTimeout(() => setTitleVisible(true), DOT_FADE_MS + 100);
    return () => {
      clearTimeout(fadeIn);
      clearTimeout(begin);
    };
  }, []);

  useEffect(() => {
    if (!titleDone) return;
    const expand = setTimeout(
      () => setParagraphExpanded(true),
      POST_TITLE_PAUSE_MS,
    );
    const startPara = setTimeout(
      () => setParagraphVisible(true),
      POST_TITLE_PAUSE_MS + PARAGRAPH_EXPAND_MS + POST_EXPAND_PAUSE_MS,
    );
    return () => {
      clearTimeout(expand);
      clearTimeout(startPara);
    };
  }, [titleDone]);

  useEffect(() => {
    if (!paragraphDone) return;
    const t = setTimeout(() => setNextVisible(true), 200);
    return () => clearTimeout(t);
  }, [paragraphDone]);

  return (
    <div className="min-h-screen flex items-center justify-center pb-[10vh]">
      <div
        className="text-sm transition-[width] ease-out min-h-[17em]"
        style={{
          width: paragraphExpanded ? WIDE_WIDTH : NARROW_WIDTH,
          transitionDuration: `${PARAGRAPH_EXPAND_MS}ms`,
        }}
      >
        <p className="font-mono flex items-center gap-2 min-h-[1.5em] mb-2 whitespace-nowrap">
          <Circle
            size={8}
            className="fill-current transition-opacity duration-400"
            style={{ opacity: dotVisible ? 1 : 0 }}
          />
          <span>{title}</span>
        </p>
        <p className="min-h-[1.5em] pb-2">
          {paragraph.slice(0, LINE_1.length)}
        </p>
        <p className="min-h-[4em] pb-2 whitespace-pre-wrap">
          {paragraph.length > LINE_2_START
            ? paragraph.slice(LINE_2_START, LINE_2_END)
            : ""}
        </p>
        <p className="min-h-[2em] whitespace-pre-wrap">
          {paragraph.length > LINE_3_START ? paragraph.slice(LINE_3_START) : ""}
        </p>
        <div
          className="flex justify-between items-baseline transition-opacity duration-500"
          style={{ opacity: nextVisible ? 1 : 0 }}
        >
          <p>{SIGNATURE}</p>
          <button
            type="button"
            disabled={!nextVisible}
            onClick={() => router.push("/onboarding/name")}
            className="cursor-pointer underline decoration-current text-muted-foreground hover:text-foreground transition-colors duration-200 disabled:cursor-not-allowed"
          >
            Next.
          </button>
        </div>
      </div>
    </div>
  );
}
