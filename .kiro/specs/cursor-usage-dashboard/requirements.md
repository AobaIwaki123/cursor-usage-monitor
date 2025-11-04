# Requirements Document

## Introduction

Cursor Usage Dashboardは、CursorのAPIやモデル使用状況を可視化するWebアプリケーションです。CSVファイルからusageデータを読み込み、トークン使用量、コスト、時系列での使用パターンなどを直感的なダッシュボードで表示します。開発環境はDockerで分離され、ローカル開発を容易にします。

## Glossary

- **Dashboard_System**: Cursor Usage Dashboardアプリケーション
- **Usage_Data**: Cursorの使用統計データ（日付、モデル、トークン数、コストなど）
- **CSV_File**: Usage_Dataを含むカンマ区切り値ファイル
- **Docker_Environment**: アプリケーションの実行環境を分離するコンテナ化システム
- **Visualization_Component**: データを視覚的に表示するUI要素（グラフ、チャート等）

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

**User Story:** As a Cursor user, I want to see my token usage over time, so that I can understand my usage patterns and trends.

#### Acceptance Criteria

1. THE Dashboard_System SHALL display a time-series chart showing Total Tokens usage by date
2. THE Dashboard_System SHALL display separate lines for Input tokens and Output tokens on the same chart
3. WHEN hovering over data points, THE Dashboard_System SHALL show detailed information including exact token counts and timestamps
4. THE Dashboard_System SHALL allow users to filter the time range using date selectors
5. THE Dashboard_System SHALL automatically scale the chart axes based on the data range

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

**User Story:** As a Cursor user, I want to see model usage statistics, so that I can understand which models I use most frequently.

#### Acceptance Criteria

1. THE Dashboard_System SHALL display usage frequency by model type in a bar chart
2. THE Dashboard_System SHALL show token consumption breakdown by model
3. THE Dashboard_System SHALL calculate and display efficiency metrics (tokens per request) by model
4. THE Dashboard_System SHALL allow filtering and sorting of model statistics
5. THE Dashboard_System SHALL highlight cache usage effectiveness for each model