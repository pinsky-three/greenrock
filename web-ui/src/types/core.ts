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

export type LatestSessionResponse = {
  session_id: string;
  candles: ApiCandle[];
  balance: Balance;
};
