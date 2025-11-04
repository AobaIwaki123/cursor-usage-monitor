# Requirements Document

## Introduction

Cursor Usage Dashboardは、CursorのAPIやモデル使用状況を可視化するWebアプリケーションです。CSVファイルからusageデータを読み込み、トークン使用量、コスト、時系列での使用パターンなどを直感的なダッシュボードで表示します。開発環境はDockerで分離され、ローカル開発を容易にします。

## Glossary

- **Dashboard_System**: Cursor Usage Dashboardアプリケーション
- **Usage_Data**: Cursorの使用統計データ（日付、モデル、トークン数、コストなど）
- **CSV_File**: Usage_Dataを含むカンマ区切り値ファイル
- **Docker_Environment**: アプリケーションの実行環境を分離するコンテナ化システム
- **Visualization_Component**: データを視覚的に表示するUI要素（グラフ、チャート等）
- **Date_Range_Preset**: 事前定義された期間選択オプション（今日、今月など）
- **Cache_Hit_Rate**: キャッシュから提供されたトークンの割合（cache_read / total_input × 100）

## Requirements

### Requirement 1

**User Story:** As a Cursor user, I want to upload my usage CSV file, so that I can visualize my API usage patterns and costs.

#### Acceptance Criteria

1. WHEN a user selects a CSV file, THE Dashboard_System SHALL validate the file format and structure
2. THE Dashboard_System SHALL parse CSV_File containing Date, Kind, Model, Max Mode, Input tokens, Cache Read, Output Tokens, Total Tokens, and Cost columns
3. IF the CSV_File format is invalid, THEN THE Dashboard_System SHALL display an error message with specific validation details
4. WHEN CSV_File is successfully uploaded, THE Dashboard_System SHALL store the Usage_Data in memory for visualization
5. THE Dashboard_System SHALL support CSV files with UTF-8 encoding

### Requirement 2

**User Story:** As a Cursor user, I want to see my token usage over time with multiple viewing perspectives, so that I can understand my usage patterns and trends at different granularities.

#### Acceptance Criteria

1. THE Dashboard_System SHALL display a time-series chart showing Total Tokens usage by date
2. THE Dashboard_System SHALL provide toggle options to switch between daily, hourly, and 10-minute time granularity views
3. THE Dashboard_System SHALL display separate lines for Input tokens and Output tokens on the same chart
4. WHEN hovering over data points, THE Dashboard_System SHALL show detailed information including exact token counts and timestamps
5. THE Dashboard_System SHALL provide preset date range options including "Today", "This Week", "This Month", "Last Month", "Last 7 Days", "Last 30 Days", and "All Time"
6. THE Dashboard_System SHALL allow users to select custom date ranges using date pickers
7. THE Dashboard_System SHALL automatically scale the chart axes based on the data range

### Requirement 3

**User Story:** As a Cursor user, I want to see my cost breakdown, so that I can monitor my spending and budget accordingly.

#### Acceptance Criteria

1. THE Dashboard_System SHALL display total cost for the selected time period
2. THE Dashboard_System SHALL show cost breakdown by model type in a pie chart or bar chart
3. THE Dashboard_System SHALL display daily cost trends in a line chart
4. THE Dashboard_System SHALL calculate and display average cost per day and per request
5. THE Dashboard_System SHALL highlight the most expensive usage sessions

### Requirement 4

**User Story:** As a developer, I want to run the application in a Docker environment, so that I can have consistent development setup across different machines.

#### Acceptance Criteria

1. THE Dashboard_System SHALL run completely within Docker_Environment containers
2. THE Dashboard_System SHALL include a docker-compose.yml file for easy local development setup
3. WHEN running docker-compose up, THE Dashboard_System SHALL start all necessary services and be accessible on localhost
4. THE Docker_Environment SHALL include hot-reload functionality for development
5. THE Dashboard_System SHALL persist uploaded data during container restarts using Docker volumes

### Requirement 5

**User Story:** As a Cursor user, I want to see model usage statistics with filtering capabilities, so that I can understand which models I use most frequently and compare their performance.

#### Acceptance Criteria

1. THE Dashboard_System SHALL display usage frequency by model type in a bar chart
2. THE Dashboard_System SHALL show token consumption breakdown by model
3. THE Dashboard_System SHALL calculate and display efficiency metrics (tokens per request) by model
4. THE Dashboard_System SHALL allow filtering and sorting of model statistics
5. THE Dashboard_System SHALL calculate cache efficiency as the ratio of cache read tokens to total input tokens, expressed as a percentage not exceeding 100%
6. THE Dashboard_System SHALL display cache efficiency with clear explanation of the metric (e.g., "Cache Hit Rate: X% of input tokens served from cache")

### Requirement 6

**User Story:** As a Cursor user, I want to upload new CSV files and see updated visualizations, so that I can analyze my latest usage data without losing previous insights.

#### Acceptance Criteria

1. THE Dashboard_System SHALL accept new CSV_File uploads while preserving existing visualization state
2. WHEN a new CSV_File is uploaded, THE Dashboard_System SHALL merge the new Usage_Data with existing data
3. THE Dashboard_System SHALL update all Visualization_Components to reflect the combined dataset
4. THE Dashboard_System SHALL maintain user-selected filters and view preferences after new data upload
5. THE Dashboard_System SHALL provide option to replace existing data or append new data

### Requirement 7

**User Story:** As a Cursor user, I want to see comprehensive statistical insights, so that I can make informed decisions about my API usage and costs.

#### Acceptance Criteria

1. THE Dashboard_System SHALL calculate and display peak usage hours and days
2. THE Dashboard_System SHALL show cost efficiency metrics (cost per token, cost per request)
3. THE Dashboard_System SHALL calculate usage trends (growth rate, usage patterns)
4. THE Dashboard_System SHALL calculate cache hit ratio as (cache_read_tokens / total_input_tokens) × 100, ensuring the value does not exceed 100%
5. THE Dashboard_System SHALL display cache savings in monetary terms based on the difference between cached and non-cached token costs
6. THE Dashboard_System SHALL provide model comparison statistics (performance, cost-effectiveness)
7. THE Dashboard_System SHALL show usage distribution percentiles (median, 95th percentile usage)

### Requirement 8

**User Story:** As a developer, I want to ensure the Rust API performs efficiently under load, so that the dashboard remains responsive with large datasets.

#### Acceptance Criteria

1. THE Dashboard_System SHALL include performance benchmarks for CSV parsing operations
2. THE Dashboard_System SHALL measure API response times for file upload endpoints under various file sizes
3. THE Dashboard_System SHALL test memory usage efficiency when processing large CSV files (up to 100MB)
4. THE Dashboard_System SHALL benchmark statistical calculation performance for datasets with 10,000+ records
5. THE Dashboard_System SHALL include load testing for concurrent file upload scenarios
6. THE Dashboard_System SHALL measure and report API throughput (requests per second) for all endpoints

### Requirement 9

**User Story:** As a Cursor user, I want the dashboard to have good visual accessibility, so that I can easily read and understand the data regardless of lighting conditions or visual capabilities.

#### Acceptance Criteria

1. THE Dashboard_System SHALL use font sizes of at least 14px for body text and 16px for important labels
2. THE Dashboard_System SHALL maintain a minimum contrast ratio of 4.5:1 for normal text and 3:1 for large text according to WCAG AA standards
3. THE Dashboard_System SHALL use distinct colors with sufficient contrast for chart elements
4. THE Dashboard_System SHALL provide clear visual hierarchy with appropriate spacing and typography
5. THE Dashboard_System SHALL ensure all interactive elements have visible focus states

### Requirement 10

**User Story:** As a Cursor user, I want date filtering to apply consistently across all visualizations, so that I can analyze specific time periods comprehensively.

#### Acceptance Criteria

1. WHEN a user selects a Date_Range_Preset or custom date range, THE Dashboard_System SHALL apply the filter to all Visualization_Components simultaneously
2. THE Dashboard_System SHALL update token usage charts, cost breakdowns, model statistics, and comprehensive statistics based on the selected date range
3. THE Dashboard_System SHALL display the currently active date range prominently in the dashboard header
4. THE Dashboard_System SHALL persist the selected date range when uploading new data
5. THE Dashboard_System SHALL recalculate all metrics and statistics based on the filtered date range
