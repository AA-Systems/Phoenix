import type { Metadata } from "next";

import { AuthForm } from "@/components/auth/auth-form";
import { AuthShell } from "@/components/auth/auth-shell";

export const metadata: Metadata = {
  title: "Create account",
};

export default function SignupPage() {
  return (
    <AuthShell
      description="Create one account for spot balances and future market access. No portfolio estimate, just the assets you actually hold."
      eyebrow="New account"
      title="Begin with a clean ledger."
    >
      <AuthForm mode="signup" />
    </AuthShell>
  );
}
