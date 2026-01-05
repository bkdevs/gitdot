"use client";

import type { FormEvent } from "react";
import { signup } from "@/actions";

export default function SignupForm() {
  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);

    await signup(formData);
  }

  return (
    <form className="flex flex-col" onSubmit={handleSubmit}>
      <p className="pb-4">Sign-up.</p>

      <input type="email" name="email" placeholder="Email" />
      <input type="password" name="password" placeholder="Password" />

      <div className="flex flex-row left-align pt-4">
        <button type="submit" className="font-bold">
          Submit
        </button>
      </div>
    </form>
  );
}
