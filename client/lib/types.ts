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

export type Session = {
  user: User;
  accessToken: string;
  expiresAt: number;
};
