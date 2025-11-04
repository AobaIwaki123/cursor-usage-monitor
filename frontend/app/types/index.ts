// Core data structures matching backend Rust models

export interface UsageData {
  date: string;
  kind: string;
  model: string;
  max_mode: boolean;
  input_with_cache: number;
  input_without_cache: number;
  cache_read: number;
  output_tokens: number;
  total_tokens: number;
  cost: number;
}

export interface DateRange {
  start: string;
  end: string;
}

export interface ModelStats {
  model: string;
  total_requests: number;
  total_tokens: number;
  total_cost: number;
  average_tokens_per_request: number;
  cache_efficiency: number;
}

export interface UsageSummary {
  total_cost: number;
  total_tokens: number;
  average_cost_per_day: number;
  most_used_model: string;
  date_range: DateRange;
  model_breakdown: ModelStats[];
}

// Comprehensive statistics structures
export interface PeakUsageStats {
  peak_hour: number;
  peak_day: string;
  peak_tokens_per_hour: number;
  peak_cost_per_day: number;
}

export interface CostEfficiencyStats {
  cost_per_token: number;
  cost_per_request: number;
  cache_savings: number;
}

export interface UsagePercentiles {
  median: number;
  p95: number;
  p99: number;
}

export interface UsageTrendStats {
  daily_growth_rate: number;
  usage_pattern: string;
  usage_percentiles: UsagePercentiles;
}

export interface ComprehensiveStats {
  peak_usage: PeakUsageStats;
  cost_efficiency: CostEfficiencyStats;
  usage_trends: UsageTrendStats;
}

// API response types
export interface UploadResponse {
  success: boolean;
  data: UsageData[];
  summary: UsageSummary;
}

export interface ErrorDetails {
  code: string;
  message: string;
  details?: any;
}

export interface ErrorResponse {
  success: boolean;
  error: ErrorDetails;
}

export interface HealthResponse {
  status: string;
  timestamp: string;
}

// Chart data structures
export interface TimeSeriesDataPoint {
  date: string;
  timestamp: number;
  input_tokens: number;
  output_tokens: number;
  total_tokens: number;
  cost: number;
}

export interface CostBreakdownDataPoint {
  model: string;
  cost: number;
  percentage: number;
}

export interface ModelUsageDataPoint {
  model: string;
  requests: number;
  tokens: number;
  cost: number;
  cache_efficiency: number;
}

export interface DailyCostDataPoint {
  date: string;
  cost: number;
}

export interface HourlyUsageDataPoint {
  hour: number;
  tokens: number;
  cost: number;
}

// Filter and view preference types
export interface DateRangeFilter {
  start: Date | null;
  end: Date | null;
}

export interface ViewPreferences {
  timeGranularity: 'daily' | 'hourly';
  modelView: 'individual' | 'aggregated';
  selectedModels: string[];
}

// Component prop types
export interface ChartDataProps {
  data: UsageData[];
  dateRange?: DateRangeFilter;
  viewPreferences?: ViewPreferences;
}
