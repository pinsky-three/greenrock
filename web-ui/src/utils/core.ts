import type { Time } from "lightweight-charts";
import type { ApiCandle, Candle, LatestSessionResponse } from "../types/core";

// Convert API candles to chart format
export const convertApiCandlesToChart = (apiCandles: ApiCandle[]): Candle[] => {
  return apiCandles.map((candle) => ({
    time: Math.floor(candle.timestamp / 1000) as Time,
    open: candle.open,
    high: candle.high,
    low: candle.low,
    close: candle.close,
  }));
};

// Fetch latest session data from API
export const fetchLatestSession = async (): Promise<LatestSessionResponse> => {
  const response = await fetch("http://localhost:4200/session");
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};
