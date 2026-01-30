"use client";

import { useState, type FormEvent } from "react";
import { authorizeDeviceAction } from "@/actions";

interface AuthorizeFormProps {
  initialCode?: string;
}

export default function AuthorizeForm({ initialCode }: AuthorizeFormProps) {
  const [code, setCode] = useState(initialCode || "");
  const [status, setStatus] = useState<"idle" | "loading" | "success" | "error">("idle");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setStatus("loading");
    setErrorMessage(null);

    const formData = new FormData();
    formData.set("user_code", code);

    const result = await authorizeDeviceAction(formData);

    if (result.error) {
      setStatus("error");
      setErrorMessage(result.error);
    } else {
      setStatus("success");
    }
  }

  if (status === "success") {
    return (
      <div className="text-center p-6 border rounded-lg">
        <div className="text-green-600 text-4xl mb-4">&#10003;</div>
        <h2 className="text-xl font-semibold mb-2">Device Authorized</h2>
        <p className="text-gray-600">
          You can close this window and return to your terminal.
        </p>
      </div>
    );
  }

  return (
    <form className="flex flex-col gap-4 w-full" onSubmit={handleSubmit}>
      <div>
        <label htmlFor="code" className="block text-sm font-medium mb-1">
          Enter the code shown in your terminal
        </label>
        <input
          id="code"
          type="text"
          value={code}
          onChange={(e) => setCode(e.target.value.toUpperCase())}
          placeholder="XXXX-XXXX"
          className="w-full px-4 py-2 border rounded-lg text-center text-2xl font-mono tracking-widest"
          maxLength={9}
          autoComplete="off"
          autoFocus
        />
      </div>

      {status === "error" && errorMessage && (
        <div className="text-red-600 text-sm text-center">{errorMessage}</div>
      )}

      <button
        type="submit"
        disabled={status === "loading" || code.length < 8}
        className="w-full py-2 px-4 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {status === "loading" ? "Authorizing..." : "Authorize"}
      </button>

      <p className="text-xs text-gray-500 text-center">
        Make sure this code matches the one shown in your terminal. Only
        authorize if you initiated this request.
      </p>
    </form>
  );
}
