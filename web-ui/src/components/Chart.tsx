import {
  // AreaSeries,
  createChart,
  ColorType,
  CandlestickSeries,
} from "lightweight-charts";
import type { ISeriesApi, IChartApi, Time } from "lightweight-charts";
import { useEffect, useRef } from "react";
import type { Candle } from "../types/core";

export const ChartComponent = (props: {
  // colors: {
  //   backgroundColor: string;
  //   lineColor: string;
  //   textColor: string;
  //   areaTopColor: string;
  //   areaBottomColor: string;
  // };
  candleData?: Candle[];
  onSeriesReady?: (series: ISeriesApi<"Candlestick">) => void;
  autoFitContent?: boolean;
}) => {
  const { candleData, onSeriesReady, autoFitContent = true } = props;

  const chartContainerRef = useRef<HTMLDivElement>(null);

  const chartRef = useRef<IChartApi | null>(null);
  const seriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);
  const isInitialLoadRef = useRef(true);
  const previousDataLengthRef = useRef(0);

  useEffect(() => {
    if (!chartContainerRef.current) return;

    // console.log(
    //   "DEBUG: Chart created with candleData length",
    //   candleData?.length
    // );

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
          barSpacing: 50, // Much larger spacing to see individual candles
          minBarSpacing: 20,
          rightOffset: 100,
          fixLeftEdge: false,
          fixRightEdge: false,
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

      // Initial setup - force large spacing to see individual candles
      // console.log(
      //   "DEBUG: Chart created, applying LARGE spacing for 500 candles in 8 hours"
      // );
      chart.timeScale().applyOptions({
        barSpacing: 50,
        rightOffset: 100,
      });
    }

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [onSeriesReady]);

  // Separate effect for updating data
  useEffect(() => {
    if (seriesRef.current && candleData && candleData.length > 0) {
      const currentDataLength = candleData.length;
      const previousDataLength = previousDataLengthRef.current;

      if (isInitialLoadRef.current) {
        // Initial load - always use setData for all historical data
        // console.log(
        //   "Chart: Initial load: setting",
        //   currentDataLength,
        //   "candles"
        // );
        // console.log("Chart: First candle:", candleData[0]);
        // console.log("Chart: Last candle:", candleData[candleData.length - 1]);
        seriesRef.current.setData(candleData);
        previousDataLengthRef.current = currentDataLength;
        isInitialLoadRef.current = false;
        // Explicitly set visible time range to show last 200 candles on load
        if (chartRef.current) {
          const lastIndex = candleData.length - 1;
          const startIndex = Math.max(0, lastIndex - 200);
          const fromTime = candleData[startIndex].time as Time;
          const toTime = candleData[lastIndex].time as Time;
          // console.log(
          //   "Chart: setVisibleRange from",
          //   fromTime,
          //   "to",
          //   toTime,
          //   `(showing ${lastIndex - startIndex + 1} candles)`
          // );
          chartRef.current
            .timeScale()
            .setVisibleRange({ from: fromTime, to: toTime });
        }
      } else if (currentDataLength > previousDataLength) {
        // New data added - use update for the latest candle
        console.log("New candle added, updating latest");
        const latestCandle = candleData[candleData.length - 1];
        seriesRef.current.update(latestCandle);
        previousDataLengthRef.current = currentDataLength;
      } else if (
        currentDataLength === previousDataLength &&
        previousDataLength > 0
      ) {
        // Same length, but data might have changed (e.g., last candle updated)
        console.log("Updating existing candle");
        const latestCandle = candleData[candleData.length - 1];
        seriesRef.current.update(latestCandle);
      } else {
        // Data length changed significantly - full reset needed
        console.log("Full reset: setting", currentDataLength, "candles");
        seriesRef.current.setData(candleData);
        previousDataLengthRef.current = currentDataLength;
      }
    }
  }, [candleData, autoFitContent]);

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
