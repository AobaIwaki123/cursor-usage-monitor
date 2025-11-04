'use client';

import { useMemo, useState } from 'react';
import {
  PieChart,
  Pie,
  Cell,
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

interface CostBreakdownChartProps {
  data: UsageData[];
}

type ModelView = 'individual' | 'aggregated';

interface ModelCostData {
  model: string;
  cost: number;
  percentage: number;
}

interface DailyCostData {
  date: string;
  cost: number;
}

const COLORS = [
  '#3b82f6', // blue
  '#10b981', // green
  '#f59e0b', // amber
  '#ef4444', // red
  '#8b5cf6', // purple
  '#ec4899', // pink
  '#06b6d4', // cyan
  '#f97316', // orange
];

export default function CostBreakdownChart({ data }: CostBreakdownChartProps) {
  const [modelView, setModelView] = useState<ModelView>('individual');

  // Calculate model cost breakdown
  const modelCostData = useMemo(() => {
    if (!data || data.length === 0) return [];

    const costByModel = new Map<string, number>();
    let totalCost = 0;

    data.forEach((item) => {
      const model = modelView === 'individual' ? item.model : 'All Models';
      costByModel.set(model, (costByModel.get(model) || 0) + item.cost);
      totalCost += item.cost;
    });

    return Array.from(costByModel.entries())
      .map(([model, cost]) => ({
        model,
        cost,
        percentage: (cost / totalCost) * 100,
      }))
      .sort((a, b) => b.cost - a.cost);
  }, [data, modelView]);

  // Calculate daily cost trends
  const dailyCostData = useMemo(() => {
    if (!data || data.length === 0) return [];

    const costByDate = new Map<string, number>();

    data.forEach((item) => {
      const date = new Date(item.date).toISOString().split('T')[0];
      costByDate.set(date, (costByDate.get(date) || 0) + item.cost);
    });

    return Array.from(costByDate.entries())
      .map(([date, cost]) => ({ date, cost }))
      .sort((a, b) => a.date.localeCompare(b.date));
  }, [data]);

  // Calculate summary statistics
  const costSummary = useMemo(() => {
    if (!data || data.length === 0) {
      return { total: 0, average: 0, perRequest: 0 };
    }

    const total = data.reduce((sum, item) => sum + item.cost, 0);
    const uniqueDates = new Set(data.map((item) => new Date(item.date).toISOString().split('T')[0]));
    const average = total / uniqueDates.size;
    const perRequest = total / data.length;

    return { total, average, perRequest };
  }, [data]);

  const formatCurrency = (value: number) => {
    return `$${value.toFixed(2)}`;
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  const CustomPieTooltip = ({ active, payload }: any) => {
    if (!active || !payload || payload.length === 0) return null;

    const data = payload[0].payload;

    return (
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700">
        <p className="text-sm font-semibold text-gray-900 dark:text-white mb-2">
          {data.model}
        </p>
        <div className="space-y-1">
          <div className="flex justify-between gap-4 text-sm">
            <span className="text-gray-600 dark:text-gray-400">Cost:</span>
            <span className="font-medium text-gray-900 dark:text-white">
              {formatCurrency(data.cost)}
            </span>
          </div>
          <div className="flex justify-between gap-4 text-sm">
            <span className="text-gray-600 dark:text-gray-400">Percentage:</span>
            <span className="font-medium text-gray-900 dark:text-white">
              {data.percentage.toFixed(1)}%
            </span>
          </div>
        </div>
      </div>
    );
  };

  const CustomLineTooltip = ({ active, payload, label }: any) => {
    if (!active || !payload || payload.length === 0) return null;

    return (
      <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700">
        <p className="text-sm font-semibold text-gray-900 dark:text-white mb-2">
          {formatDate(label)}
        </p>
        <div className="flex items-center gap-2 text-sm">
          <div className="w-3 h-3 rounded-full bg-blue-500" />
          <span className="text-gray-600 dark:text-gray-400">Cost:</span>
          <span className="font-medium text-gray-900 dark:text-white">
            {formatCurrency(payload[0].value)}
          </span>
        </div>
      </div>
    );
  };

  if (!data || data.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          Cost Breakdown
        </h2>
        <p className="text-gray-500 dark:text-gray-400">No data available</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Cost Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">
            Total Cost
          </h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white">
            {formatCurrency(costSummary.total)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">
            Average Cost per Day
          </h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white">
            {formatCurrency(costSummary.average)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h3 className="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">
            Average Cost per Request
          </h3>
          <p className="text-3xl font-bold text-gray-900 dark:text-white">
            {formatCurrency(costSummary.perRequest)}
          </p>
        </div>
      </div>

      {/* Cost Breakdown by Model */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
            Cost Breakdown by Model
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
            <PieChart>
              <Pie
                data={modelCostData}
                dataKey="cost"
                nameKey="model"
                cx="50%"
                cy="50%"
                outerRadius={100}
                label={({ model, percentage }) =>
                  `${model}: ${percentage.toFixed(1)}%`
                }
                labelLine={true}
              >
                {modelCostData.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                ))}
              </Pie>
              <Tooltip content={<CustomPieTooltip />} />
            </PieChart>
          </ResponsiveContainer>
        </div>

        {/* Model Cost Legend */}
        <div className="mt-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
          {modelCostData.map((item, index) => (
            <div key={item.model} className="flex items-center gap-3">
              <div
                className="w-4 h-4 rounded-full flex-shrink-0"
                style={{ backgroundColor: COLORS[index % COLORS.length] }}
              />
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900 dark:text-white truncate">
                  {item.model}
                </p>
                <p className="text-xs text-gray-600 dark:text-gray-400">
                  {formatCurrency(item.cost)} ({item.percentage.toFixed(1)}%)
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Daily Cost Trends */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-6">
          Daily Cost Trends
        </h2>

        <div className="w-full h-80">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={dailyCostData}>
              <CartesianGrid strokeDasharray="3 3" className="stroke-gray-200 dark:stroke-gray-700" />
              <XAxis
                dataKey="date"
                tickFormatter={formatDate}
                className="text-gray-600 dark:text-gray-400"
                tick={{ fill: 'currentColor' }}
              />
              <YAxis
                tickFormatter={formatCurrency}
                className="text-gray-600 dark:text-gray-400"
                tick={{ fill: 'currentColor' }}
              />
              <Tooltip content={<CustomLineTooltip />} />
              <Legend />
              <Line
                type="monotone"
                dataKey="cost"
                name="Daily Cost"
                stroke="#3b82f6"
                strokeWidth={2}
                dot={{ r: 4 }}
                activeDot={{ r: 6 }}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
}
