import { clearSession, getSession, saveSession } from "@/lib/session";
import type { AssetBalance, AuthResponse } from "@/lib/types";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3000";

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

async function refreshSession() {
  const auth = await request<AuthResponse>("/api/v1/auth/refresh", {
    method: "POST",
  });
  return saveSession(auth);
}

export async function getBalances(): Promise<AssetBalance[]> {
  let session = getSession();
  if (!session) throw new ApiError(401, "Please log in to view balances.");

  try {
    return await request<AssetBalance[]>("/api/v1/balances/get", {
      method: "POST",
      token: session.accessToken,
    });
  } catch (error) {
    if (!(error instanceof ApiError) || error.status !== 401) throw error;
    session = await refreshSession();
    return request<AssetBalance[]>("/api/v1/balances/get", {
      method: "POST",
      token: session.accessToken,
    });
  }
}
