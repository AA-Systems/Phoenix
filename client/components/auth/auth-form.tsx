"use client";

import { ArrowRight, Check, LoaderCircle } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { FormEvent, useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { login, signup } from "@/lib/api";
import { saveSession } from "@/lib/session";

type AuthFormProps = {
  mode: "login" | "signup";
};

export function AuthForm({ mode }: AuthFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();
  const isSignup = mode === "signup";
  const passwordChecks = [
    { label: "Uppercase", valid: /\p{Lu}/u.test(password) },
    { label: "Lowercase", valid: /\p{Ll}/u.test(password) },
    { label: "Number", valid: /\p{N}/u.test(password) },
    {
      label: "Special character",
      valid: /[^\p{L}\p{N}\s]/u.test(password),
    },
  ];
  const passwordValid =
    password.length >= 12 && passwordChecks.every(({ valid }) => valid);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setLoading(true);
    setError("");

    const form = new FormData(event.currentTarget);
    const email = String(form.get("email"));
    const password = String(form.get("password"));

    try {
      const auth = isSignup
        ? await signup(String(form.get("name")), email, password)
        : await login(email, password);
      saveSession(auth);
      router.push("/balances");
    } catch (caught) {
      setError(
        caught instanceof Error ? caught.message : "Unable to continue.",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <form className="space-y-5" onSubmit={handleSubmit}>
      {isSignup && (
        <Input
          autoComplete="name"
          label="Full name"
          name="name"
          placeholder="Your name"
          required
        />
      )}
      <Input
        autoComplete="email"
        label="Email"
        name="email"
        placeholder="name@domain.com"
        required
        type="email"
      />
      <Input
        autoComplete={isSignup ? "new-password" : "current-password"}
        hint={isSignup ? "12+ characters" : undefined}
        label="Password"
        minLength={12}
        name="password"
        onChange={(event) => setPassword(event.target.value)}
        placeholder="••••••••••••"
        required
        type="password"
        value={password}
      />

      {isSignup && (
        <div className="grid grid-cols-2 gap-2 text-xs text-[#777b84]">
          {passwordChecks.map(({ label, valid }) => (
            <span
              className={`flex items-center gap-2 transition-colors ${
                valid ? "text-[#74ddbd]" : "text-[#777b84]"
              }`}
              key={label}
            >
              <Check size={13} />
              {label}
            </span>
          ))}
        </div>
      )}

      {error && (
        <p className="rounded-xl border border-[#6e353f] bg-[#211318] px-4 py-3 text-sm text-[#ff9e96]">
          {error}
        </p>
      )}

      <Button
        className="w-full"
        disabled={loading || (isSignup && !passwordValid)}
        type="submit"
      >
        {loading ? <LoaderCircle className="animate-spin" size={17} /> : null}
        {isSignup ? "Create account" : "Enter exchange"}
        {!loading && <ArrowRight size={17} />}
      </Button>

      <p className="text-center text-sm text-[#817884]">
        {isSignup ? "Already have an account?" : "New to PHEONIX?"}{" "}
        <Link
          className="font-medium text-[#eee8ef] underline decoration-[#625769] underline-offset-4 hover:text-[#ff8175]"
          href={isSignup ? "/login" : "/signup"}
        >
          {isSignup ? "Log in" : "Create one"}
        </Link>
      </p>
    </form>
  );
}
