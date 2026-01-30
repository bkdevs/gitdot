"use client";

import { validateEmail } from "@/util";
import { createClient } from "@supabase/supabase-js";
import { useState } from "react";

export function SubscribeButton() {
  const [isClicked, setIsClicked] = useState(false);
  const [email, setEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = async () => {
    // TODO: remove so we don't have to maintain two dbs.
    const supabase = createClient(
      "https://ttvxrkljjbcapscsqopv.supabase.co",
      "sb_publishable_ob5MXkeMmhi_zMgH5yMs-g_TGg5YjpQ",
    );
    supabase
      .from("waitlist")
      .insert({ email: email })
      .then(({ error }) => {
        if (error) {
          console.error(error);
        }
      });

    setSubmitted(true);
    setTimeout(() => {
      setSubmitted(false);
      setIsClicked(false);
      setEmail("");
    }, 2000);
  };

  return submitted ? (
    <p className="h-5">subscribed.</p>
  ) : isClicked ? (
    <input
      className="border-b border-bg ring-0 outline-none h-5 w-40"
      type="email"
      autoFocus
      placeholder="enter email..."
      value={email}
      onChange={(e) => setEmail(e.target.value)}
      onBlur={() => {
        if (email.length === 0) {
          setIsClicked(false);
        }
      }}
      onKeyDown={(e) => {
        if (e.key === "Escape") {
          setIsClicked(false);
        } else if (e.key === "Enter" && validateEmail(email)) {
          handleSubmit();
        }
      }}
    />
  ) : (
    <button
      type="submit"
      className="underline cursor-pointer h-5"
      onClick={() => setIsClicked(true)}
    >
      subscribe
    </button>
  );
}
