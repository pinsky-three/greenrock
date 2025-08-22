import type { Time } from "lightweight-charts";

export type Candle = {
  time: Time;
  open: number;
  high: number;
  low: number;
  close: number;
};

export type ApiCandle = {
  close: number;
  high: number;
  low: number;
  open: number;
  timestamp: number;
  ts: string;
  volume: number;
};

export type Balance = {
  [symbol: string]: number;
};

// Chat API types
export type ChatRequest = {
  query: string;
  session_id?: string;
};

export type ChatResponse = {
  session_id: string;
  answer: string;
  status: string;
};

export type PauseResponse = {
  session_id: string;
  status: string;
  next_task: string;
  reason: string;
};

// Portfolio types
export type Portfolio = {
  [symbol: string]: number;
};

// Order Book types
export type OrderBookEntry = {
  price: number;
  quantity: number;
};

export type OrderBook = {
  symbol: string;
  bids: OrderBookEntry[];
  asks: OrderBookEntry[];
  timestamp: number;
};

// Trading types
export type Order = {
  id: string;
  symbol: string;
  side: "buy" | "sell";
  type: "market" | "limit";
  quantity: number;
  price?: number;
  status: string;
  timestamp: number;
};

export type Trade = {
  id: string;
  symbol: string;
  side: "buy" | "sell";
  quantity: number;
  price: number;
  timestamp: number;
  fee?: number;
};

// API Query types
export type CandlesQuery = {
  symbol: string;
  interval: string;
  start?: string;
  end?: string;
};

export type OrderBookQuery = {
  symbol: string;
  depth: number;
};

// Legacy type for backward compatibility
export type LatestSessionResponse = {
  session_id: string;
  candles: ApiCandle[];
  balance: Balance;
};

export type TimeRange = {
  start: Date;
  end: Date;
};

export type TimeRangePreset = {
  label: string;
  start: Date;
  end: Date;
};

export type TimeRangePresets = {
  [key: string]: TimeRangePreset;
};
