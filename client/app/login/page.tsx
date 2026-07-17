import type { Metadata } from "next";

import { AuthForm } from "@/components/auth/auth-form";
import { AuthShell } from "@/components/auth/auth-shell";

export const metadata: Metadata = {
  title: "Log in",
};

export default function LoginPage() {
  return (
    <AuthShell
      description="Return to your balances and active session. Your refresh token stays in a protected browser cookie."
      eyebrow="Session access"
      title="Back to the market."
    >
      <AuthForm mode="login" />
    </AuthShell>
  );
}
