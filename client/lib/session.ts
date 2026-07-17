import type { AuthResponse, Session } from "@/lib/types";

const SESSION_KEY = "cex_session";

export function saveSession(auth: AuthResponse): Session {
  const session = {
    user: auth.user,
    accessToken: auth.access_token,
    expiresAt: Date.now() + auth.expires_in * 1000,
  };

  sessionStorage.setItem(SESSION_KEY, JSON.stringify(session));
  window.dispatchEvent(new Event("cex-session"));
  return session;
}

export function getSession(): Session | null {
  if (typeof window === "undefined") return null;

  const stored = sessionStorage.getItem(SESSION_KEY);
  if (!stored) return null;

  try {
    return JSON.parse(stored) as Session;
  } catch {
    clearSession();
    return null;
  }
}

export function clearSession() {
  if (typeof window === "undefined") return;
  sessionStorage.removeItem(SESSION_KEY);
  window.dispatchEvent(new Event("cex-session"));
}
