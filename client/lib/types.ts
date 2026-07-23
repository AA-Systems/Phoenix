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
