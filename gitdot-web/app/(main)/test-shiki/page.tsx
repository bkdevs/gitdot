"use client";

import { useEffect, useRef, useState } from "react";
import { createShikiWorker } from "@/workers/index";

export default function TestShikiPage() {
  const [html, setHtml] = useState<string | null>(null);
  const workerRef = useRef<Worker | null>(null);

  useEffect(() => {
    const worker = createShikiWorker();
    workerRef.current = worker;

    worker.onmessage = (e: MessageEvent<{ html: string }>) => {
      setHtml(e.data.html);
    };

    worker.postMessage({
      code: 'const greeting = "hello world";\nconsole.log(greeting);',
      lang: "typescript",
      theme: "vitesse-light",
    });

    return () => worker.terminate();
  }, []);

  return (
    <div className="p-8">
      <h1 className="text-lg font-semibold mb-4">Shiki Worker Test</h1>
      {html ? (
        <div dangerouslySetInnerHTML={{ __html: html }} />
      ) : (
        <p className="text-sm text-gray-500">Loading...</p>
      )}
    </div>
  );
}
