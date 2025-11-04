'use client';

import { useMemo, useState } from 'react';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import type { UsageData } from '@/app/types';

interface TokenUsageChartProps {
  data: UsageData[];
}

type TimeGranularity = 'daily' | 'hourly';

interface ChartDataPoint {
  date: string;
  timestamp: number;
  input_tokens: number;
  output_tokens: number;
  total_tokens: number;
}

export default function TokenUsageChart({ data }: TokenUsageChartProps) {
  const [granularity, setGranularity] = useState<TimeGranularity>('daily');
  const [dateRange, setDateRange] = useState<{ start: string; end: string }>({
    start: '',
    end: '',
  });

  // Process data based on granularity
  const chartData = useMemo(() => {
    if (!data || data.length === 0) return [];

    // Filter by date range if set
    let filteredData = data;
    if (dateRange.start || dateRange.end) {
      filteredData = data.filter((item) => {
        const itemDate = new Date(item.date);
        const startDate = dateRange.start ? new Date(dateRange.start) : null;
        const endDate = dateRange.end ? new Date(dateRange.end) : null;

        if (startDate && itemDate < startDate) return false;
        if (endDate && itemDate > endDate) return false;
        return true;
      });
    }

    // Group data by granularity
    const grouped = new Map<string, ChartDataPoint>();

    filteredData.forEach((item) => {
      const date = new Date(item.date);
      let key: string;

      if (granularity === 'daily') {
        key = date.toISOString().split('T')[0];
      } else {
        // Hourly granularity
        const hour = date.getHours();
        key = `${date.toISOString().split('T')[0]}T${hour.toString().padStart(2, '0')}:00:00`;
      }

      const existing = grouped.get(key);
      const inputTokens = item.input_with_cache + item.input_without_cache;

      if (existing) {
        existing.input_tokens += inputTokens;
        existing.output_tokens += item.output_tokens;
        existing.total_tokens += item.total_tokens;
      } else {
        grouped.set(key, {
          date: key,
          timestamp: new Date(key).getTime(),
          input_tokens: inputTokens,
          output_tokens: item.output_tokens,
          total_tokens: item.total_tokens,
        });
      }
    });

    // Sort by timestamp
    return Array.from(grouped.values()).sort((a, b) => a.timestamp - b.timestamp);
  }, [data, granularity, dateRange]);

  // Get min and max dates for date range inputs
  const dateExtent = useMemo(() => {
    if (!data || data.length === 0) return { min: '', max: '' };

    const dates = data.map((d) => new Date(d.date).getTime());
    const min = new Date(Math.min(...dates)).toISOString().split('T')[0];
    const max = new Date(Math.max(...dates)).toISOString().split('T')[0];

    return { min, max };
  }, [data]);

  const formatXAxis = (value: string) => {
    const date = new Date(value);
    if (granularity === 'daily') {
      return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    } else {
      return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
    }
  };

  const formatNumber = (num: number) => {
    return num.toLocaleString();
  };

  const CustomTooltip = ({ active, payload, label }: any) => {
    if (!active || !payload || payload.length === 0) return null;

    const date = new Date(label);
    const formattedDate =
      granularity === 'daily'
        ? date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
          })
        : date.toLocaleString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
          });

    return (
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700">
        <p className="text-sm font-semibold text-gray-900 dark:text-white mb-2">
          {formattedDate}
        </p>
        {payload.map((entry: any, index: number) => (
          <div key={index} className="flex items-center gap-2 text-sm">
            <div
              className="w-3 h-3 rounded-full"
              style={{ backgroundColor: entry.color }}
            />
            <span className="text-gray-600 dark:text-gray-400">{entry.name}:</span>
            <span className="font-medium text-gray-900 dark:text-white">
              {formatNumber(entry.value)}
            </span>
          </div>
        ))}
      </div>
    );
  };

  if (!data || data.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          Token Usage Over Time
        </h2>
        <p className="text-gray-500 dark:text-gray-400">No data available</p>
      </div>
    );
  }

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Token Usage Over Time
        </h2>

        <div className="flex flex-col sm:flex-row gap-4">
          {/* Granularity Toggle */}
          <div className="flex gap-2">
            <button
              onClick={() => setGranularity('daily')}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                granularity === 'daily'
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
              }`}
            >
              Daily
            </button>
            <button
              onClick={() => setGranularity('hourly')}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                granularity === 'hourly'
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
              }`}
            >
              Hourly
            </button>
          </div>
        </div>
      </div>

      {/* Date Range Filters */}
      <div className="flex flex-col sm:flex-row gap-4 mb-6">
        <div className="flex-1">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Start Date
          </label>
          <input
            type="date"
            value={dateRange.start}
            onChange={(e) => setDateRange({ ...dateRange, start: e.target.value })}
            min={dateExtent.min}
            max={dateExtent.max}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
        <div className="flex-1">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            End Date
          </label>
          <input
            type="date"
            value={dateRange.end}
            onChange={(e) => setDateRange({ ...dateRange, end: e.target.value })}
            min={dateExtent.min}
            max={dateExtent.max}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
        {(dateRange.start || dateRange.end) && (
          <div className="flex items-end">
            <button
              onClick={() => setDateRange({ start: '', end: '' })}
              className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white"
            >
              Clear
            </button>
          </div>
        )}
      </div>

      {/* Chart */}
      <div className="w-full h-96">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" className="stroke-gray-200 dark:stroke-gray-700" />
            <XAxis
              dataKey="date"
              tickFormatter={formatXAxis}
              className="text-gray-600 dark:text-gray-400"
              tick={{ fill: 'currentColor' }}
            />
            <YAxis
              tickFormatter={formatNumber}
              className="text-gray-600 dark:text-gray-400"
              tick={{ fill: 'currentColor' }}
            />
            <Tooltip content={<CustomTooltip />} />
            <Legend
              wrapperStyle={{
                paddingTop: '20px',
              }}
            />
            <Line
              type="monotone"
              dataKey="input_tokens"
              name="Input Tokens"
              stroke="#3b82f6"
              strokeWidth={2}
              dot={{ r: 3 }}
              activeDot={{ r: 5 }}
            />
            <Line
              type="monotone"
              dataKey="output_tokens"
              name="Output Tokens"
              stroke="#10b981"
              strokeWidth={2}
              dot={{ r: 3 }}
              activeDot={{ r: 5 }}
            />
            <Line
              type="monotone"
              dataKey="total_tokens"
              name="Total Tokens"
              stroke="#8b5cf6"
              strokeWidth={2}
              dot={{ r: 3 }}
              activeDot={{ r: 5 }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
