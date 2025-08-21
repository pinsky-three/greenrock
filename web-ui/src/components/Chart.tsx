import {
  // AreaSeries,
  createChart,
  ColorType,
  CandlestickSeries,
} from "lightweight-charts";
import type { ISeriesApi, IChartApi } from "lightweight-charts";
import { useEffect, useRef } from "react";
import type { Candle } from "../types/core";

export const ChartComponent = (props: {
  colors: {
    backgroundColor: string;
    lineColor: string;
    textColor: string;
    areaTopColor: string;
    areaBottomColor: string;
  };
  candleData?: Candle[];
  onSeriesReady?: (series: ISeriesApi<"Candlestick">) => void;
}) => {
  const { candleData, onSeriesReady } = props;

  const chartContainerRef = useRef<HTMLDivElement>(null);

  const chartRef = useRef<IChartApi | null>(null);
  const seriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);

  useEffect(() => {
    if (!chartContainerRef.current) return;

    const handleResize = () => {
      if (chartRef.current) {
        chartRef.current.applyOptions({
          width: chartContainerRef.current?.clientWidth,
          height: chartContainerRef.current?.clientHeight || 400,
        });
      }
    };

    // Only create chart if it doesn't exist
    if (!chartRef.current) {
      const chart = createChart(chartContainerRef.current, {
        layout: {
          background: { type: ColorType.Solid, color: "#000000" },
          textColor: "#ffffff",
        },
        grid: {
          vertLines: {
            color: "#1a1a1a",
          },
          horzLines: {
            color: "#1a1a1a",
          },
        },
        timeScale: {
          borderColor: "#333333",
          timeVisible: true,
        },
        rightPriceScale: {
          borderColor: "#333333",
        },
        width: chartContainerRef.current?.clientWidth,
        height: chartContainerRef.current?.clientHeight || 400,
      });

      const series = chart.addSeries(CandlestickSeries, {
        upColor: "#00a6a5",
        downColor: "#ff1e38",
        borderVisible: false,
        wickUpColor: "#00a6a5",
        wickDownColor: "#ff1e38",
      });

      chartRef.current = chart;
      seriesRef.current = series;

      // Notify parent component that series is ready for real-time updates
      if (onSeriesReady) {
        onSeriesReady(series);
      }

      chart.timeScale().fitContent();
      chart.timeScale().scrollToPosition(5, true);
    }

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [onSeriesReady]);

  // Separate effect for updating data
  useEffect(() => {
    if (seriesRef.current && candleData && candleData.length > 0) {
      seriesRef.current.setData(candleData);
    }
  }, [candleData]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (chartRef.current) {
        chartRef.current.remove();
        chartRef.current = null;
        seriesRef.current = null;
      }
    };
  }, []);

  return <div ref={chartContainerRef} className="w-full h-full" />;
};
