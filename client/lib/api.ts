import { clearSession, getSession, saveSession } from "@/lib/session";
import type {
  AssetBalance,
  AuthResponse,
  LedgerEntry,
  Session,
} from "@/lib/types";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3000";
let refreshPromise: Promise<Session> | null = null;

type RequestOptions = RequestInit & {
  token?: string;
};

async function request<T>(
  path: string,
  options: RequestOptions = {},
): Promise<T> {
  const headers = new Headers(options.headers);
  if (options.body) headers.set("Content-Type", "application/json");
  if (options.token) headers.set("Authorization", `Bearer ${options.token}`);

  const response = await fetch(`${API_URL}${path}`, {
    ...options,
    headers,
    credentials: "include",
  });

  if (!response.ok) {
    const message = (await response.text()) || "Something went wrong";
    throw new ApiError(response.status, message);
  }

  if (response.status === 204) return undefined as T;
  return response.json() as Promise<T>;
}

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
  ) {
    super(message);
  }
}

export function login(email: string, password: string) {
  return request<AuthResponse>("/api/v1/auth/login", {
    method: "POST",
    body: JSON.stringify({ email, password }),
  });
}

export function signup(name: string, email: string, password: string) {
  return request<AuthResponse>("/api/v1/auth/register", {
    method: "POST",
    body: JSON.stringify({ name, email, password }),
  });
}

export async function logout() {
  try {
    await request<void>("/api/v1/auth/logout", { method: "POST" });
  } finally {
    clearSession();
  }
}

function refreshSession(): Promise<Session> {
  if (!refreshPromise) {
    refreshPromise = request<AuthResponse>("/api/v1/auth/refresh", {
      method: "POST",
    })
      .then(saveSession)
      .catch((error) => {
        if (error instanceof ApiError && error.status === 401) clearSession();
        throw error;
      })
      .finally(() => {
        refreshPromise = null;
      });
  }

  return refreshPromise;
}

export async function restoreSession(): Promise<Session | null> {
  const session = getSession();
  if (session && session.expiresAt > Date.now() + 5_000) return session;

  try {
    return await refreshSession();
  } catch {
    return null;
  }
}

async function authedPost<T>(path: string): Promise<T> {
  let session = await restoreSession();
  if (!session) throw new ApiError(401, "Please log in.");

  try {
    return await request<T>(path, {
      method: "POST",
      token: session.accessToken,
    });
  } catch (error) {
    if (!(error instanceof ApiError) || error.status !== 401) throw error;
    session = await refreshSession();
    return request<T>(path, {
      method: "POST",
      token: session.accessToken,
    });
  }
}

export function getBalances(): Promise<AssetBalance[]> {
  return authedPost("/api/v1/balances/get");
}

export function getLedger(): Promise<LedgerEntry[]> {
  return authedPost("/api/v1/balances/ledger");
}
