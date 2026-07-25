import { clearSession, getSession, saveSession } from "@/lib/session";
import type {
  Asset,
  AssetBalance,
  AuthResponse,
  CancelOrderResponse,
  CreateOrderResponse,
  LedgerEntry,
  Market,
  OpenOrder,
  OrderBookDepth,
  OrderType,
  Session,
  TradeView,
} from "@/lib/types";
import type { Candle, CandleInterval } from "@/lib/candles";

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

async function authedPost<T>(path: string, body?: unknown): Promise<T> {
  let session = await restoreSession();
  if (!session) throw new ApiError(401, "Please log in.");

  const options: RequestOptions = {
    method: "POST",
    token: session.accessToken,
    body: body === undefined ? undefined : JSON.stringify(body),
  };

  try {
    return await request<T>(path, options);
  } catch (error) {
    if (!(error instanceof ApiError) || error.status !== 401) throw error;
    session = await refreshSession();
    return request<T>(path, {
      ...options,
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

export function demoCredit(assetSymbol?: string): Promise<{
  credits: { command_id: string; asset_symbol: string; amount: number }[];
}> {
  return authedPost(
    "/api/v1/balances/demo-credit",
    assetSymbol ? { asset_symbol: assetSymbol } : {},
  );
}

export function listMarkets(limit = 50, skip = 0): Promise<Market[]> {
  const params = new URLSearchParams({
    limit: String(limit),
    skip: String(skip),
  });
  return request<Market[]>(`/api/v1/markets?${params}`);
}

export function listAssets(): Promise<Asset[]> {
  return request<Asset[]>("/api/v1/assets");
}

export function getOrderBook(marketSymbol: string): Promise<OrderBookDepth> {
  return request<OrderBookDepth>("/api/v1/markets/book", {
    method: "POST",
    body: JSON.stringify({ market_symbol: marketSymbol }),
  });
}

export function getRecentTrades(
  marketSymbol: string,
  limit = 50,
): Promise<TradeView[]> {
  return request<TradeView[]>("/api/v1/markets/trades", {
    method: "POST",
    body: JSON.stringify({ market_symbol: marketSymbol, limit }),
  });
}

export function getCandles(
  marketSymbol: string,
  interval: CandleInterval,
  limit = 200,
): Promise<Candle[]> {
  return request<Candle[]>("/api/v1/markets/candles", {
    method: "POST",
    body: JSON.stringify({
      market_symbol: marketSymbol,
      interval,
      limit,
    }),
  });
}

export function listOpenOrders(): Promise<OpenOrder[]> {
  return authedPost("/api/v1/orders/open");
}

export function createOrder(input: {
  market_symbol: string;
  order_type: OrderType;
  price: number;
  quantity: number;
}): Promise<CreateOrderResponse> {
  return authedPost("/api/v1/orders/create", input);
}

export function cancelOrder(orderId: string): Promise<CancelOrderResponse> {
  return authedPost("/api/v1/orders/cancel", { order_id: orderId });
}
