"use server";

import { createSupabaseClient } from "@/lib/supabase";

export async function signup(formData: FormData) {
  const supabase = await createSupabaseClient();

  // todo: add validation
  const email = formData.get("email") as string;
  const password = formData.get("password") as string;

  const { data, error } = await supabase.auth.signUp({
    email,
    password,
  });

  console.log(data);
  console.error(error);
}

export async function login(formData: FormData) {
  const supabase = await createSupabaseClient();

  // todo: add validation
  const email = formData.get("email") as string;
  const password = formData.get("password") as string;

  const { data, error } = await supabase.auth.signInWithPassword({
    email,
    password,
  });

  console.log(data);
  console.error(error);
}

export async function signout() {
  const supabase = await createSupabaseClient();
  const { error } = await supabase.auth.signOut();

  console.log(error);
}
