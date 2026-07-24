export type User = {
  id: string;
  name: string;
  email: string;
  created_at: string;
  updated_at: string;
};

export type AuthResponse = {
  user: User;
  access_token: string;
  expires_in: number;
};

export type AssetBalance = {
  asset_id: string;
  symbol: string;
  name: string;
  decimals: number;
  available: number;
  locked: number;
  updated_at: string;
};

export type LedgerEntryType =
  "deposit" | "withdrawal" | "lock" | "unlock" | "trade" | "fee" | "adjustment";

export type LedgerEntry = {
  id: string;
  asset_id: string;
  asset_symbol: string;
  asset_decimals: number;
  entry_type: LedgerEntryType;
  available_delta: number;
  locked_delta: number;
  available_after: number;
  locked_after: number;
  reference_id: string | null;
  reference_type: string | null;
  command_id: string | null;
  created_at: string;
};

export type Session = {
  user: User;
  accessToken: string;
  expiresAt: number;
};

export type MarketStatus = "trading" | "halted" | "archived";

export type Market = {
  id: string;
  symbol: string;
  name: string;
  base_asset_id: string;
  quote_asset_id: string;
  status: MarketStatus;
  price_tick_size: number;
  quantity_step_size: number;
  min_order_quantity: number;
  min_order_notional: number;
  created_at: string;
};
