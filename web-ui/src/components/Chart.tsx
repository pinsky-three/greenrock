import {
  createChart,
  ColorType,
  CandlestickSeries,
  LineSeries,
} from "lightweight-charts";
import type { ISeriesApi, IChartApi } from "lightweight-charts";
import { useEffect, useRef } from "react";
import type { Candle } from "../types/core";

export const ChartComponent = (props: {
  candleData?: Candle[];
  onSeriesReady?: (series: ISeriesApi<"Candlestick">) => void;
  autoFitContent?: boolean;
}) => {
  const { candleData, onSeriesReady, autoFitContent = true } = props;

  const chartContainerRef = useRef<HTMLDivElement>(null);

  const chartRef = useRef<IChartApi | null>(null);
  const seriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);
  const ma20SeriesRef = useRef<ISeriesApi<"Line"> | null>(null);
  const ma50SeriesRef = useRef<ISeriesApi<"Line"> | null>(null);
  const ma200SeriesRef = useRef<ISeriesApi<"Line"> | null>(null);
  const isInitialLoadRef = useRef(true);
  const previousDataLengthRef = useRef(0);

  // Function to calculate moving average
  const calculateMovingAverage = (candleData: Candle[], period: number) => {
    const maData = [];

    for (let i = 0; i < candleData.length; i++) {
      if (i < period - 1) {
        // Provide whitespace data points until the MA can be calculated
        maData.push({ time: candleData[i].time });
      } else {
        // Calculate the moving average
        let sum = 0;
        for (let j = 0; j < period; j++) {
          sum += candleData[i - j].close;
        }
        const maValue = sum / period;
        maData.push({ time: candleData[i].time, value: maValue });
      }
    }

    return maData;
  };

  // Function to update moving averages for real-time updates
  const updateMovingAverages = (candleData: Candle[], index: number) => {
    const updateMA = (
      period: number,
      seriesRef: React.MutableRefObject<ISeriesApi<"Line"> | null>
    ) => {
      if (index >= period - 1 && seriesRef.current) {
        let sum = 0;
        for (let j = 0; j < period; j++) {
          sum += candleData[index - j].close;
        }
        const maValue = sum / period;
        seriesRef.current.update({
          time: candleData[index].time,
          value: maValue,
        });
      }
    };

    updateMA(20, ma20SeriesRef);
    updateMA(50, ma50SeriesRef);
    updateMA(200, ma200SeriesRef);
  };

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
          // barSpacing: 20, // Good default spacing
          // minBarSpacing: 1, // Allow zooming out much more
          // rightOffset: 20,
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

      // Add moving average series
      const ma20Series = chart.addSeries(LineSeries, {
        color: "#2962FF", // Blue
        lineWidth: 1,
        title: "MA 20",
      });

      const ma50Series = chart.addSeries(LineSeries, {
        color: "#FF6D00", // Orange
        lineWidth: 1,
        title: "MA 50",
      });

      const ma200Series = chart.addSeries(LineSeries, {
        color: "#E91E63", // Pink
        lineWidth: 1,
        title: "MA 200",
      });

      chartRef.current = chart;
      seriesRef.current = series;
      ma20SeriesRef.current = ma20Series;
      ma50SeriesRef.current = ma50Series;
      ma200SeriesRef.current = ma200Series;

      if (onSeriesReady) {
        onSeriesReady(series);
      }

      chart.timeScale().applyOptions({
        barSpacing: 10,
        rightOffset: 5,
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
        seriesRef.current.setData(candleData);

        const ma20Data = calculateMovingAverage(candleData, 20);
        const ma50Data = calculateMovingAverage(candleData, 50);
        const ma200Data = calculateMovingAverage(candleData, 200);

        if (ma20SeriesRef.current) ma20SeriesRef.current.setData(ma20Data);
        if (ma50SeriesRef.current) ma50SeriesRef.current.setData(ma50Data);
        if (ma200SeriesRef.current) ma200SeriesRef.current.setData(ma200Data);

        previousDataLengthRef.current = currentDataLength;
        isInitialLoadRef.current = false;

        if (chartRef.current) {
          chartRef.current.timeScale().scrollToPosition(5, true);
        }
      } else if (currentDataLength > previousDataLength) {
        const latestCandle = candleData[candleData.length - 1];
        seriesRef.current.update(latestCandle);

        updateMovingAverages(candleData, currentDataLength - 1);

        previousDataLengthRef.current = currentDataLength;
      } else if (
        currentDataLength === previousDataLength &&
        previousDataLength > 0
      ) {
        const latestCandle = candleData[candleData.length - 1];
        seriesRef.current.update(latestCandle);

        updateMovingAverages(candleData, currentDataLength - 1);
      } else {
        seriesRef.current.setData(candleData);

        const ma20Data = calculateMovingAverage(candleData, 20);
        const ma50Data = calculateMovingAverage(candleData, 50);
        const ma200Data = calculateMovingAverage(candleData, 200);

        if (ma20SeriesRef.current) ma20SeriesRef.current.setData(ma20Data);
        if (ma50SeriesRef.current) ma50SeriesRef.current.setData(ma50Data);
        if (ma200SeriesRef.current) ma200SeriesRef.current.setData(ma200Data);

        previousDataLengthRef.current = currentDataLength;
      }
    }
  }, [candleData, autoFitContent]);

  useEffect(() => {
    return () => {
      if (chartRef.current) {
        chartRef.current.remove();
        chartRef.current = null;
        seriesRef.current = null;
        ma20SeriesRef.current = null;
        ma50SeriesRef.current = null;
        ma200SeriesRef.current = null;
      }
    };
  }, []);

  return <div ref={chartContainerRef} className="w-full h-full" />;
};
