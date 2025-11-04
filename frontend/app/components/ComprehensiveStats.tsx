'use client';

import { useMemo } from 'react';
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  Radar,
} from 'recharts';
import type { UsageData } from '@/app/types';

interface ComprehensiveStatsProps {
  data: UsageData[];
}

interface PeakUsageStats {
  peak_hour: number;
  peak_day: string;
  peak_tokens_per_hour: number;
  peak_cost_per_day: number;
}

interface CostEfficiencyStats {
  cost_per_token: number;
  cost_per_request: number;
  cache_savings: number;
}

interface UsageTrendStats {
  daily_growth_rate: number;
  usage_pattern: string;
  median: number;
  p95: number;
  p99: number;
}

interface CachePerformance {
  total_cache_read: number;
  total_input: number;
  cache_hit_ratio: number;
  estimated_savings: number;
}

interface ModelComparison {
  model: string;
  avg_cost_per_request: number;
  avg_tokens_per_request: number;
  cache_efficiency: number;
  total_requests: number;
}

export default function ComprehensiveStats({ data }: ComprehensiveStatsProps) {
  // Calculate peak usage statistics
  const peakUsageStats = useMemo((): PeakUsageStats => {
    if (!data || data.length === 0) {
      return {
        peak_hour: 0,
        peak_day: '',
        peak_tokens_per_hour: 0,
        peak_cost_per_day: 0,
      };
    }

    // Peak hour analysis
    const tokensByHour = new Map<number, number>();
    data.forEach((item) => {
      const hour = new Date(item.date).getHours();
      tokensByHour.set(hour, (tokensByHour.get(hour) || 0) + item.total_tokens);
    });

    let peakHour = 0;
    let peakTokensPerHour = 0;
    tokensByHour.forEach((tokens, hour) => {
      if (tokens > peakTokensPerHour) {
        peakTokensPerHour = tokens;
        peakHour = hour;
      }
    });

    // Peak day analysis
    const costByDay = new Map<string, number>();
    data.forEach((item) => {
      const day = new Date(item.date).toISOString().split('T')[0];
      costByDay.set(day, (costByDay.get(day) || 0) + item.cost);
    });

    let peakDay = '';
    let peakCostPerDay = 0;
    costByDay.forEach((cost, day) => {
      if (cost > peakCostPerDay) {
        peakCostPerDay = cost;
        peakDay = day;
      }
    });

    return {
      peak_hour: peakHour,
      peak_day: peakDay,
      peak_tokens_per_hour: peakTokensPerHour,
      peak_cost_per_day: peakCostPerDay,
    };
  }, [data]);

  // Calculate cost efficiency metrics
  const costEfficiencyStats = useMemo((): CostEfficiencyStats => {
    if (!data || data.length === 0) {
      return {
        cost_per_token: 0,
        cost_per_request: 0,
        cache_savings: 0,
      };
    }

    const totalCost = data.reduce((sum, item) => sum + item.cost, 0);
    const totalTokens = data.reduce((sum, item) => sum + item.total_tokens, 0);
    const totalCacheRead = data.reduce((sum, item) => sum + item.cache_read, 0);

    // Estimate cache savings (assuming cache reads cost 10% of regular tokens)
    const cacheSavings = totalCacheRead * (totalCost / totalTokens) * 0.9;

    return {
      cost_per_token: totalTokens > 0 ? totalCost / totalTokens : 0,
      cost_per_request: data.length > 0 ? totalCost / data.length : 0,
      cache_savings: cacheSavings,
    };
  }, [data]);

  // Calculate usage trends
  const usageTrendStats = useMemo((): UsageTrendStats => {
    if (!data || data.length === 0) {
      return {
        daily_growth_rate: 0,
        usage_pattern: 'stable',
        median: 0,
        p95: 0,
        p99: 0,
      };
    }

    // Calculate daily totals
    const dailyTotals = new Map<string, number>();
    data.forEach((item) => {
      const day = new Date(item.date).toISOString().split('T')[0];
      dailyTotals.set(day, (dailyTotals.get(day) || 0) + item.total_tokens);
    });

    const sortedDays = Array.from(dailyTotals.entries()).sort((a, b) =>
      a[0].localeCompare(b[0])
    );

    // Calculate growth rate
    let growthRate = 0;
    if (sortedDays.length >= 2) {
      const firstWeek = sortedDays.slice(0, Math.min(7, sortedDays.length));
      const lastWeek = sortedDays.slice(-Math.min(7, sortedDays.length));

      const firstWeekAvg =
        firstWeek.reduce((sum, [, tokens]) => sum + tokens, 0) / firstWeek.length;
      const lastWeekAvg =
        lastWeek.reduce((sum, [, tokens]) => sum + tokens, 0) / lastWeek.length;

      growthRate = firstWeekAvg > 0 ? ((lastWeekAvg - firstWeekAvg) / firstWeekAvg) * 100 : 0;
    }

    // Determine usage pattern
    let usagePattern = 'stable';
    if (growthRate > 10) usagePattern = 'increasing';
    else if (growthRate < -10) usagePattern = 'decreasing';

    // Calculate percentiles
    const sortedTokens = data.map((item) => item.total_tokens).sort((a, b) => a - b);
    const median = sortedTokens[Math.floor(sortedTokens.length * 0.5)] || 0;
    const p95 = sortedTokens[Math.floor(sortedTokens.length * 0.95)] || 0;
    const p99 = sortedTokens[Math.floor(sortedTokens.length * 0.99)] || 0;

    return {
      daily_growth_rate: growthRate,
      usage_pattern: usagePattern,
      median,
      p95,
      p99,
    };
  }, [data]);

  // Calculate cache performance
  const cachePerformance = useMemo((): CachePerformance => {
    if (!data || data.length === 0) {
      return {
        total_cache_read: 0,
        total_input: 0,
        cache_hit_ratio: 0,
        estimated_savings: 0,
      };
    }

    const totalCacheRead = data.reduce((sum, item) => sum + item.cache_read, 0);
    const totalInput = data.reduce(
      (sum, item) => sum + item.input_with_cache + item.input_without_cache,
      0
    );
    const cacheHitRatio = totalInput > 0 ? (totalCacheRead / totalInput) * 100 : 0;

    const totalCost = data.reduce((sum, item) => sum + item.cost, 0);
    const totalTokens = data.reduce((sum, item) => sum + item.total_tokens, 0);
    const estimatedSavings = totalCacheRead * (totalCost / totalTokens) * 0.9;

    return {
      total_cache_read: totalCacheRead,
      total_input: totalInput,
      cache_hit_ratio: cacheHitRatio,
      estimated_savings: estimatedSavings,
    };
  }, [data]);

  // Calculate model comparison
  const modelComparison = useMemo((): ModelComparison[] => {
    if (!data || data.length === 0) return [];

    const modelStats = new Map<string, ModelComparison>();

    data.forEach((item) => {
      const existing = modelStats.get(item.model);
      const inputTokens = item.input_with_cache + item.input_without_cache;

      if (existing) {
        existing.total_requests += 1;
        existing.avg_cost_per_request += item.cost;
        existing.avg_tokens_per_request += item.total_tokens;
        existing.cache_efficiency += inputTokens > 0 ? (item.cache_read / inputTokens) * 100 : 0;
      } else {
        modelStats.set(item.model, {
          model: item.model,
          avg_cost_per_request: item.cost,
          avg_tokens_per_request: item.total_tokens,
          cache_efficiency: inputTokens > 0 ? (item.cache_read / inputTokens) * 100 : 0,
          total_requests: 1,
        });
      }
    });

    // Calculate averages
    modelStats.forEach((stats) => {
      stats.avg_cost_per_request /= stats.total_requests;
      stats.avg_tokens_per_request /= stats.total_requests;
      stats.cache_efficiency /= stats.total_requests;
    });

    return Array.from(modelStats.values()).sort((a, b) => b.total_requests - a.total_requests);
  }, [data]);

  // Prepare hourly usage data for chart
  const hourlyUsageData = useMemo(() => {
    const hourlyData = Array.from({ length: 24 }, (_, i) => ({
      hour: i,
      tokens: 0,
      cost: 0,
    }));

    data.forEach((item) => {
      const hour = new Date(item.date).getHours();
      hourlyData[hour].tokens += item.total_tokens;
      hourlyData[hour].cost += item.cost;
    });

    return hourlyData;
  }, [data]);

  const formatNumber = (num: number) => num.toLocaleString();
  const formatCurrency = (value: number) => `$${value.toFixed(2)}`;
  const formatPercentage = (value: number) => `${value.toFixed(1)}%`;

  if (!data || data.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          Comprehensive Statistics
        </h2>
        <p className="text-gray-500 dark:text-gray-400">No data available</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Peak Usage Statistics */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-6">
          Peak Usage Analysis
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
          <div className="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <p className="text-sm text-blue-600 dark:text-blue-400 mb-1">Peak Hour</p>
            <p className="text-2xl font-bold text-blue-900 dark:text-blue-100">
              {peakUsageStats.peak_hour}:00
            </p>
          </div>
          <div className="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
            <p className="text-sm text-green-600 dark:text-green-400 mb-1">Peak Day</p>
            <p className="text-2xl font-bold text-green-900 dark:text-green-100">
              {new Date(peakUsageStats.peak_day).toLocaleDateString('en-US', {
                month: 'short',
                day: 'numeric',
              })}
            </p>
          </div>
          <div className="p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
            <p className="text-sm text-purple-600 dark:text-purple-400 mb-1">Peak Tokens/Hour</p>
            <p className="text-2xl font-bold text-purple-900 dark:text-purple-100">
              {formatNumber(peakUsageStats.peak_tokens_per_hour)}
            </p>
          </div>
          <div className="p-4 bg-amber-50 dark:bg-amber-900/20 rounded-lg">
            <p className="text-sm text-amber-600 dark:text-amber-400 mb-1">Peak Cost/Day</p>
            <p className="text-2xl font-bold text-amber-900 dark:text-amber-100">
              {formatCurrency(peakUsageStats.peak_cost_per_day)}
            </p>
          </div>
        </div>

        {/* Hourly Usage Pattern */}
        <div className="w-full h-64">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={hourlyUsageData}>
              <CartesianGrid strokeDasharray="3 3" className="stroke-gray-200 dark:stroke-gray-700" />
              <XAxis
                dataKey="hour"
                label={{ value: 'Hour of Day', position: 'insideBottom', offset: -5 }}
                className="text-gray-600 dark:text-gray-400"
                tick={{ fill: 'currentColor' }}
              />
              <YAxis
                tickFormatter={formatNumber}
                className="text-gray-600 dark:text-gray-400"
                tick={{ fill: 'currentColor' }}
              />
              <Tooltip />
              <Legend />
              <Bar dataKey="tokens" name="Tokens" fill="#3b82f6" />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Cost Efficiency Metrics */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-6">
          Cost Efficiency Metrics
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="p-6 bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 rounded-lg">
            <p className="text-sm text-blue-600 dark:text-blue-400 mb-2">Cost per Token</p>
            <p className="text-3xl font-bold text-blue-900 dark:text-blue-100">
              ${costEfficiencyStats.cost_per_token.toFixed(6)}
            </p>
          </div>
          <div className="p-6 bg-gradient-to-br from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-800/20 rounded-lg">
            <p className="text-sm text-green-600 dark:text-green-400 mb-2">Cost per Request</p>
            <p className="text-3xl font-bold text-green-900 dark:text-green-100">
              {formatCurrency(costEfficiencyStats.cost_per_request)}
            </p>
          </div>
          <div className="p-6 bg-gradient-to-br from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-800/20 rounded-lg">
            <p className="text-sm text-purple-600 dark:text-purple-400 mb-2">Cache Savings</p>
            <p className="text-3xl font-bold text-purple-900 dark:text-purple-100">
              {formatCurrency(costEfficiencyStats.cache_savings)}
            </p>
          </div>
        </div>
      </div>

      {/* Usage Trends */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-6">
          Usage Trends & Distribution
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <div className="flex items-center gap-4 mb-4">
              <div className="flex-1">
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-1">Growth Rate</p>
                <div className="flex items-center gap-2">
                  <p className="text-2xl font-bold text-gray-900 dark:text-white">
                    {formatPercentage(usageTrendStats.daily_growth_rate)}
                  </p>
                  <span
                    className={`px-2 py-1 rounded text-xs font-medium ${
                      usageTrendStats.usage_pattern === 'increasing'
                        ? 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
                        : usageTrendStats.usage_pattern === 'decreasing'
                        ? 'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400'
                        : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
                    }`}
                  >
                    {usageTrendStats.usage_pattern}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <div>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-3">Usage Percentiles</p>
            <div className="space-y-2">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-700 dark:text-gray-300">Median (50th)</span>
                <span className="font-medium text-gray-900 dark:text-white">
                  {formatNumber(usageTrendStats.median)} tokens
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-700 dark:text-gray-300">95th Percentile</span>
                <span className="font-medium text-gray-900 dark:text-white">
                  {formatNumber(usageTrendStats.p95)} tokens
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-700 dark:text-gray-300">99th Percentile</span>
                <span className="font-medium text-gray-900 dark:text-white">
                  {formatNumber(usageTrendStats.p99)} tokens
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Cache Performance */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-6">
          Cache Performance & Savings
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-1">Total Cache Reads</p>
            <p className="text-2xl font-bold text-gray-900 dark:text-white">
              {formatNumber(cachePerformance.total_cache_read)}
            </p>
          </div>
          <div className="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-1">Total Input Tokens</p>
            <p className="text-2xl font-bold text-gray-900 dark:text-white">
              {formatNumber(cachePerformance.total_input)}
            </p>
          </div>
          <div className="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
            <p className="text-sm text-green-600 dark:text-green-400 mb-1">Cache Hit Ratio</p>
            <p className="text-2xl font-bold text-green-900 dark:text-green-100">
              {formatPercentage(cachePerformance.cache_hit_ratio)}
            </p>
          </div>
          <div className="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
            <p className="text-sm text-green-600 dark:text-green-400 mb-1">Estimated Savings</p>
            <p className="text-2xl font-bold text-green-900 dark:text-green-100">
              {formatCurrency(cachePerformance.estimated_savings)}
            </p>
          </div>
        </div>
      </div>

      {/* Model Comparison */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-6">
          Model Performance Comparison
        </h2>

        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-200 dark:border-gray-700">
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-700 dark:text-gray-300">
                  Model
                </th>
                <th className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300">
                  Avg Cost/Request
                </th>
                <th className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300">
                  Avg Tokens/Request
                </th>
                <th className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300">
                  Cache Efficiency
                </th>
                <th className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300">
                  Total Requests
                </th>
              </tr>
            </thead>
            <tbody>
              {modelComparison.map((model, index) => (
                <tr
                  key={model.model}
                  className={`border-b border-gray-100 dark:border-gray-700 ${
                    index % 2 === 0 ? 'bg-gray-50 dark:bg-gray-800' : 'bg-white dark:bg-gray-900'
                  }`}
                >
                  <td className="px-4 py-3 text-sm font-medium text-gray-900 dark:text-white">
                    {model.model}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatCurrency(model.avg_cost_per_request)}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(Math.round(model.avg_tokens_per_request))}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatPercentage(model.cache_efficiency)}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(model.total_requests)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
