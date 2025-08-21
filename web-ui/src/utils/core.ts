import type { Time } from "lightweight-charts";
import type {
  ApiCandle,
  Candle,
  LatestSessionResponse,
  TimeRangePresets,
} from "../types/core";

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

// Filter candles by time range
export const filterCandlesByTimeRange = (
  candles: Candle[],
  startTime: Date,
  endTime: Date
): Candle[] => {
  const startTimestamp = Math.floor(startTime.getTime() / 1000);
  const endTimestamp = Math.floor(endTime.getTime() / 1000);

  return candles.filter((candle) => {
    let candleTime: number;
    if (typeof candle.time === "number") {
      candleTime = candle.time;
    } else if (typeof candle.time === "string") {
      candleTime = new Date(candle.time).getTime() / 1000;
    } else {
      // Handle BusinessDay or other Time types
      candleTime = 0; // Skip invalid entries
    }
    return candleTime >= startTimestamp && candleTime <= endTimestamp;
  });
};

// Get predefined time ranges
export const getTimeRangePresets = (): TimeRangePresets => {
  const now = new Date();
  const presets = {
    "1h": {
      label: "1 Hour",
      start: new Date(now.getTime() - 60 * 60 * 1000),
      end: now,
    },
    "4h": {
      label: "4 Hours",
      start: new Date(now.getTime() - 4 * 60 * 60 * 1000),
      end: now,
    },
    "1d": {
      label: "1 Day",
      start: new Date(now.getTime() - 24 * 60 * 60 * 1000),
      end: now,
    },
    "3d": {
      label: "3 Days",
      start: new Date(now.getTime() - 3 * 24 * 60 * 60 * 1000),
      end: now,
    },
    "1w": {
      label: "1 Week",
      start: new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000),
      end: now,
    },
    all: {
      label: "All Data",
      start: new Date(0), // Unix epoch
      end: now,
    },
    custom: {
      label: "Custom Range",
      start: new Date(now.getTime() - 24 * 60 * 60 * 1000), // Default to 1 day back
      end: now,
    },
  };
  return presets;
};

// Fetch latest session data from API
export const fetchLatestSession = async (): Promise<LatestSessionResponse> => {
  const response = await fetch("http://localhost:4200/session");
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};
