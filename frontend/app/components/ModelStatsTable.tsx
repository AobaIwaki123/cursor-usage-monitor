'use client';

import { useMemo, useState } from 'react';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import type { UsageData } from '@/app/types';

interface ModelStatsTableProps {
  data: UsageData[];
}

type SortField = 'model' | 'requests' | 'tokens' | 'cost' | 'efficiency' | 'cache_efficiency';
type SortDirection = 'asc' | 'desc';
type ModelView = 'individual' | 'aggregated';

interface ModelStats {
  model: string;
  requests: number;
  tokens: number;
  cost: number;
  efficiency: number; // tokens per request
  cache_efficiency: number; // percentage
  cache_read: number;
  total_input: number;
}

export default function ModelStatsTable({ data }: ModelStatsTableProps) {
  const [sortField, setSortField] = useState<SortField>('requests');
  const [sortDirection, setSortDirection] = useState<SortDirection>('desc');
  const [filterText, setFilterText] = useState('');
  const [modelView, setModelView] = useState<ModelView>('individual');

  // Calculate model statistics
  const modelStats = useMemo(() => {
    if (!data || data.length === 0) return [];

    const statsByModel = new Map<string, ModelStats>();

    data.forEach((item) => {
      const model = modelView === 'individual' ? item.model : 'All Models';
      const existing = statsByModel.get(model);
      const inputTokens = item.input_with_cache + item.input_without_cache;

      if (existing) {
        existing.requests += 1;
        existing.tokens += item.total_tokens;
        existing.cost += item.cost;
        existing.cache_read += item.cache_read;
        existing.total_input += inputTokens;
      } else {
        statsByModel.set(model, {
          model,
          requests: 1,
          tokens: item.total_tokens,
          cost: item.cost,
          efficiency: 0,
          cache_efficiency: 0,
          cache_read: item.cache_read,
          total_input: inputTokens,
        });
      }
    });

    // Calculate derived metrics
    statsByModel.forEach((stats) => {
      stats.efficiency = stats.tokens / stats.requests;
      stats.cache_efficiency =
        stats.total_input > 0 ? (stats.cache_read / stats.total_input) * 100 : 0;
    });

    return Array.from(statsByModel.values());
  }, [data, modelView]);

  // Filter and sort
  const filteredAndSortedStats = useMemo(() => {
    let filtered = modelStats;

    // Apply filter
    if (filterText) {
      filtered = filtered.filter((stat) =>
        stat.model.toLowerCase().includes(filterText.toLowerCase())
      );
    }

    // Apply sort
    filtered.sort((a, b) => {
      let aVal: number | string = a[sortField];
      let bVal: number | string = b[sortField];

      if (typeof aVal === 'string' && typeof bVal === 'string') {
        return sortDirection === 'asc'
          ? aVal.localeCompare(bVal)
          : bVal.localeCompare(aVal);
      }

      aVal = aVal as number;
      bVal = bVal as number;

      return sortDirection === 'asc' ? aVal - bVal : bVal - aVal;
    });

    return filtered;
  }, [modelStats, filterText, sortField, sortDirection]);

  // Prepare data for bar chart
  const chartData = useMemo(() => {
    return filteredAndSortedStats.slice(0, 10); // Top 10 models
  }, [filteredAndSortedStats]);

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('desc');
    }
  };

  const formatNumber = (num: number) => {
    return num.toLocaleString();
  };

  const formatCurrency = (value: number) => {
    return `$${value.toFixed(2)}`;
  };

  const SortIcon = ({ field }: { field: SortField }) => {
    if (sortField !== field) {
      return (
        <svg className="w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
        </svg>
      );
    }

    return sortDirection === 'asc' ? (
      <svg className="w-4 h-4 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 15l7-7 7 7" />
      </svg>
    ) : (
      <svg className="w-4 h-4 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
      </svg>
    );
  };

  const CustomBarTooltip = ({ active, payload, label }: any) => {
    if (!active || !payload || payload.length === 0) return null;

    return (
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700">
        <p className="text-sm font-semibold text-gray-900 dark:text-white mb-2">{label}</p>
        <div className="flex items-center gap-2 text-sm">
          <div className="w-3 h-3 rounded-full bg-blue-500" />
          <span className="text-gray-600 dark:text-gray-400">Requests:</span>
          <span className="font-medium text-gray-900 dark:text-white">
            {formatNumber(payload[0].value)}
          </span>
        </div>
      </div>
    );
  };

  if (!data || data.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          Model Usage Statistics
        </h2>
        <p className="text-gray-500 dark:text-gray-400">No data available</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Model Usage Frequency Chart */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
            Model Usage Frequency
          </h2>

          <div className="flex gap-2">
            <button
              onClick={() => setModelView('individual')}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                modelView === 'individual'
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
              }`}
            >
              Individual Models
            </button>
            <button
              onClick={() => setModelView('aggregated')}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                modelView === 'aggregated'
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
              }`}
            >
              Aggregated
            </button>
          </div>
        </div>

        <div className="w-full h-80">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" className="stroke-gray-200 dark:stroke-gray-700" />
              <XAxis
                dataKey="model"
                className="text-gray-600 dark:text-gray-400"
                tick={{ fill: 'currentColor' }}
              />
              <YAxis
                tickFormatter={formatNumber}
                className="text-gray-600 dark:text-gray-400"
                tick={{ fill: 'currentColor' }}
              />
              <Tooltip content={<CustomBarTooltip />} />
              <Legend />
              <Bar dataKey="requests" name="Requests" fill="#3b82f6" />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Model Statistics Table */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
            Model Statistics
          </h2>

          <div className="flex-1 max-w-md">
            <input
              type="text"
              placeholder="Filter by model name..."
              value={filterText}
              onChange={(e) => setFilterText(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-200 dark:border-gray-700">
                <th
                  className="px-4 py-3 text-left text-sm font-medium text-gray-700 dark:text-gray-300 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
                  onClick={() => handleSort('model')}
                >
                  <div className="flex items-center gap-2">
                    Model
                    <SortIcon field="model" />
                  </div>
                </th>
                <th
                  className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
                  onClick={() => handleSort('requests')}
                >
                  <div className="flex items-center justify-end gap-2">
                    Requests
                    <SortIcon field="requests" />
                  </div>
                </th>
                <th
                  className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
                  onClick={() => handleSort('tokens')}
                >
                  <div className="flex items-center justify-end gap-2">
                    Total Tokens
                    <SortIcon field="tokens" />
                  </div>
                </th>
                <th
                  className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
                  onClick={() => handleSort('cost')}
                >
                  <div className="flex items-center justify-end gap-2">
                    Total Cost
                    <SortIcon field="cost" />
                  </div>
                </th>
                <th
                  className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
                  onClick={() => handleSort('efficiency')}
                >
                  <div className="flex items-center justify-end gap-2">
                    Tokens/Request
                    <SortIcon field="efficiency" />
                  </div>
                </th>
                <th
                  className="px-4 py-3 text-right text-sm font-medium text-gray-700 dark:text-gray-300 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
                  onClick={() => handleSort('cache_efficiency')}
                >
                  <div className="flex items-center justify-end gap-2">
                    Cache Efficiency
                    <SortIcon field="cache_efficiency" />
                  </div>
                </th>
              </tr>
            </thead>
            <tbody>
              {filteredAndSortedStats.map((stat, index) => (
                <tr
                  key={stat.model}
                  className={`border-b border-gray-100 dark:border-gray-700 ${
                    index % 2 === 0 ? 'bg-gray-50 dark:bg-gray-800' : 'bg-white dark:bg-gray-900'
                  }`}
                >
                  <td className="px-4 py-3 text-sm font-medium text-gray-900 dark:text-white">
                    {stat.model}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(stat.requests)}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(stat.tokens)}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatCurrency(stat.cost)}
                  </td>
                  <td className="px-4 py-3 text-sm text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(Math.round(stat.efficiency))}
                  </td>
                  <td className="px-4 py-3 text-sm text-right">
                    <div className="flex items-center justify-end gap-2">
                      <div className="flex-1 max-w-[100px] bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                        <div
                          className="bg-green-500 h-2 rounded-full"
                          style={{ width: `${Math.min(stat.cache_efficiency, 100)}%` }}
                        />
                      </div>
                      <span className="text-gray-700 dark:text-gray-300 min-w-[50px]">
                        {stat.cache_efficiency.toFixed(1)}%
                      </span>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {filteredAndSortedStats.length === 0 && (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            No models match your filter
          </div>
        )}
      </div>
    </div>
  );
}
