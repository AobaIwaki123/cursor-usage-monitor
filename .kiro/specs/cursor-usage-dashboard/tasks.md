# Implementation Plan

- [x] 1. Set up project structure and Docker environment
  - Create root directory structure with frontend, backend, and shared folders
  - Create docker-compose.yml with frontend (React) and backend (Rust) services
  - Set up volume mounting for hot reload development
  - Configure environment variables and port mapping
  - _Requirements: 4.1, 4.2, 4.3_
  
  **ユーザー確認ポイント:**
  - [x] プロジェクトフォルダ構造が正しく作成されているか
  - [x] `docker compose up --build` でエラーなく起動するか
  - [x] フロントエンド (http://localhost:3000) とバックエンド (http://localhost:3001) にアクセス可能か
  - [x] ファイル変更時にホットリロードが動作するか（ファイルを編集して確認）

- [-] 2. Initialize Rust backend API server
  - [x] 2.1 Create Rust Axum server project
    - Initialize Cargo.toml with required dependencies (axum, tokio, serde, csv, tower-http)
    - Set up basic Axum server with CORS middleware
    - Create project structure with handlers, models, services, and utils modules
    - _Requirements: 4.1, 4.4_
    
    **ユーザー確認ポイント:**
    - [x] `docker compose exec api cargo build` でエラーなくコンパイルできるか
    - [x] `docker compose up api` でサーバーが起動するか
    - [x] `curl http://localhost:3001/api/health` で `GET /api/health` エンドポイントが応答するか
    - [x] CORS設定でフロントエンドからのリクエストが通るか
  
  - [x] 2.2 Implement CSV upload and parsing endpoints
    - Create POST /api/upload endpoint with multipart file handling
    - Create POST /api/upload/append endpoint for merging new data
    - Implement CSV parsing logic with comprehensive validation
    - Add error handling for invalid CSV formats and file size limits
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 6.1, 6.2, 6.3_
    
    **ユーザー確認ポイント:**
    - [x] example.csvファイルが正常にアップロード・解析されるか（`curl`または`docker compose exec api`内でテスト）
    - [x] 無効なCSVファイルで適切なエラーメッセージが返されるか
    - [x] ファイルサイズ制限（100MB）が正しく動作するか
    - [x] `/api/upload/append` で既存データにマージできるか
  
  - [x] 2.3 Create data processing and summary calculation services
    - Implement UsageData struct and parsing logic
    - Calculate usage summary statistics (total cost, model breakdown, etc.)
    - Add model statistics calculation with cache efficiency
    - Implement data merging logic for append functionality
    - _Requirements: 2.1, 3.1, 3.4, 5.2, 5.3, 6.4_
    
    **ユーザー確認ポイント:**
    - [x] アップロード後に正確な統計サマリーが返されるか
    - [x] 総コスト、総トークン数が正しく計算されているか
    - [x] モデル別の統計が適切に分類されているか
    - [x] キャッシュ効率が正しく計算されているか
  
  - [ ] 2.6 Fix cache efficiency calculation and add date filtering support
    - Update cache efficiency calculation to: (cache_read / total_input) × 100, capped at 100%
    - Rename cache_efficiency field to cache_hit_rate in ModelStats struct
    - Add cache_savings field to calculate monetary savings from cache usage
    - Implement date range filtering in backend statistics endpoints
    - Update comprehensive stats endpoint to accept date range parameters
    - _Requirements: 5.5, 5.6, 7.4, 7.5, 10.1, 10.2, 10.5_
    
    **ユーザー確認ポイント:**
    - [ ] キャッシュヒット率が100%を超えないか（`curl`でテスト）
    - [ ] cache_hit_rateとcache_savingsフィールドが正しく返されるか
    - [ ] 日付範囲パラメータでフィルタリングできるか
    - [ ] フィルタリング後の統計が正確に計算されているか
  
  - [x] 2.4 Implement comprehensive statistics calculation
    - Create services for peak usage analysis (hours, days)
    - Implement cost efficiency metrics calculation
    - Add usage trend analysis and growth rate calculation
    - Calculate usage percentiles and distribution statistics
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_
    
    **ユーザー確認ポイント:**
    - [x] `curl http://localhost:3001/api/stats/comprehensive` で `/api/stats/comprehensive` エンドポイントが正常に応答するか
    - [x] ピーク使用時間・日付が正しく特定されているか
    - [x] コスト効率メトリクス（トークンあたりコスト等）が計算されているか
    - [x] 使用量のパーセンタイル（中央値、95%等）が算出されているか
  
  - [x] 2.5 Write backend API tests and performance benchmarks
    - Create unit tests for CSV parsing functions
    - Write integration tests for upload endpoints
    - Add error handling test cases
    - Implement performance benchmarks using criterion crate
    - Add load testing for concurrent file uploads
    - Create memory usage profiling tests for large datasets
    - _Requirements: 1.3, 8.1, 8.2, 8.3, 8.4, 8.5, 8.6_
    
    **ユーザー確認ポイント:**
    - [x] `docker compose exec api cargo test` で全テストがパスするか
    - [x] `docker compose exec api cargo bench` でベンチマークが実行されるか
    - [x] 大容量ファイル（100MB）のテストが成功するか
    - [x] 同時アップロードのテストが正常に動作するか

- [-] 3. Initialize frontend Next.js application
  - [x] 3.1 Create Next.js TypeScript project with App Router
    - Initialize package.json with Next.js 16, TypeScript, and Tailwind CSS 4
    - Set up next.config.ts for development server and API proxy
    - Configure Tailwind CSS 4 and basic styling setup
    - Create app directory structure with layout.tsx and page.tsx
    - Configure Bun as package manager
    - _Requirements: 4.4_
    
    **ユーザー確認ポイント:**
    - [x] `docker compose exec view bun install` でエラーなく依存関係がインストールされるか
    - [x] `docker compose exec view bun run dev` で開発サーバーが起動するか
    - [x] TypeScriptのコンパイルエラーがないか
    - [x] Tailwind CSSのスタイルが適用されるか
  
  - [x] 3.2 Create shared TypeScript interfaces and API routes
    - Define UsageData, UsageSummary, and ModelStats interfaces in app/types
    - Create API response types and error handling types
    - Set up type definitions for chart data structures
    - Create Next.js API routes for proxying backend requests (app/api/proxy)
    - _Requirements: 1.4, 2.1, 3.1, 5.1_
    
    **ユーザー確認ポイント:**
    - [ ] TypeScript型定義がRustの構造体と一致しているか
    - [ ] APIレスポンスの型チェックが正常に動作するか（`docker compose exec view bun run build`でエラーなし）
    - [ ] Next.js API routesが正しく設定されているか
    - [ ] 型安全性が保たれているか（コンパイルエラーなし）
  
  - [x] 3.3 Implement file upload client component
    - Create FileUpload client component ('use client') with drag-and-drop functionality
    - Add file validation (CSV format, size limits)
    - Implement upload progress and error state handling
    - Use Next.js API routes for file upload
    - _Requirements: 1.1, 1.2, 1.3_
    
    **ユーザー確認ポイント:**
    - [ ] ドラッグ&ドロップでファイルアップロードできるか（ブラウザで http://localhost:3000 にアクセスして確認）
    - [ ] ファイル選択ボタンが正常に動作するか
    - [ ] CSV以外のファイルで適切なエラーが表示されるか
    - [ ] アップロード進行状況が表示されるか

- [x] 4. Implement data visualization client components with enhanced features
  - [x] 4.1 Create token usage time-series chart with granularity controls
    - Install and configure Recharts library (`docker compose exec view bun add recharts`)
    - Implement TokenUsageChart client component ('use client') with line chart
    - Add daily/hourly granularity toggle functionality
    - Add hover tooltips with detailed token information
    - Implement date range filtering functionality
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6_
    
    **ユーザー確認ポイント:**
    - [ ] 時系列チャートが正しく描画されるか（ブラウザで確認）
    - [ ] 日毎・時間ごとの切り替えが動作するか
    - [ ] ホバー時に詳細情報が表示されるか
    - [ ] 日付範囲フィルターが正常に機能するか
  
  - [x] 4.2 Create cost breakdown visualization with model filtering
    - Implement CostBreakdownChart client component ('use client') with pie chart for model costs
    - Add toggle between individual model view and aggregated overall view
    - Create daily cost trends line chart
    - Add cost summary cards with total and average calculations
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_
    
    **ユーザー確認ポイント:**
    - [ ] モデル別コスト内訳の円グラフが表示されるか（ブラウザで確認）
    - [ ] 個別モデル・全体表示の切り替えが動作するか
    - [ ] 日別コストトレンドが正しく描画されるか
    - [ ] コストサマリーカードに正確な数値が表示されるか
  
  - [x] 4.3 Create model usage statistics display with advanced filtering
    - Implement ModelStatsTable client component ('use client') with sorting and filtering
    - Add model usage frequency bar chart
    - Implement toggle between individual model and aggregated views
    - Calculate and display cache efficiency metrics
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6_
    
    **ユーザー確認ポイント:**
    - [ ] モデル統計テーブルのソート・フィルター機能が動作するか（ブラウザで確認）
    - [ ] モデル使用頻度の棒グラフが表示されるか
    - [ ] 個別・集約表示の切り替えが正常に機能するか
    - [ ] キャッシュ効率メトリクスが正確に表示されるか
  
  - [x] 4.4 Create comprehensive statistics dashboard
    - Implement client component ('use client') for peak usage hours and days visualization
    - Create cost efficiency metrics display (cost per token, per request)
    - Add usage trends visualization with growth rate indicators
    - Display cache performance and savings metrics
    - Create model comparison charts for performance analysis
    - Show usage distribution percentiles (median, 95th, 99th)
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_
    
    **ユーザー確認ポイント:**
    - [ ] ピーク使用時間・日付が視覚的に表示されるか（ブラウザで確認）
    - [ ] コスト効率メトリクス（トークンあたりコスト等）が表示されるか
    - [ ] 使用傾向と成長率が適切に可視化されるか
    - [ ] パーセンタイル分布が正確に表示されるか

- [ ] 5. Integrate components and implement enhanced dashboard layout
  - [ ] 5.1 Implement global date filtering and improved UI accessibility
    - Create DateRangeSelector component with preset options (Today, This Week, This Month, Last Month, Last 7 Days, Last 30 Days, All Time) and custom date picker
    - Implement global date range state that applies to all visualization components
    - Update TokenUsageChart to support 10-minute granularity in addition to daily and hourly
    - Remove aggregated view toggle from all components (show individual model data only)
    - Fix cache efficiency calculation to cap at 100% and display as "Cache Hit Rate: X%"
    - Update all text to minimum 14px font size and ensure 4.5:1 contrast ratio for accessibility
    - Display active date range prominently in dashboard header
    - _Requirements: 2.2, 2.5, 2.6, 5.5, 5.6, 7.4, 7.5, 9.1, 9.2, 9.3, 9.4, 9.5, 10.1, 10.2, 10.3, 10.4, 10.5_
    
    **ユーザー確認ポイント:**
    - [ ] プリセット期間選択（今日、今週、今月など）が正常に動作するか（ブラウザで確認）
    - [ ] カスタム日付選択が機能するか
    - [ ] 10分単位の粒度表示が正しく動作するか
    - [ ] 日付フィルターが全てのチャート・統計に適用されるか
    - [ ] Aggregated表示が削除されているか
    - [ ] キャッシュ効率が100%を超えないか、表示が分かりやすいか
    - [ ] フォントサイズとコントラストが改善されているか
    - [ ] 選択中の期間がヘッダーに表示されるか
  
  - [ ] 5.2 Create main Dashboard page with advanced state management
    - Update app/page.tsx as main dashboard page
    - Implement responsive grid layout for all visualization components
    - Add client-side state management for uploaded data, filters, and view preferences
    - Connect file upload with data visualization components
    - Implement data merging functionality for new CSV uploads
    - Add view preference persistence using localStorage
    - _Requirements: 1.4, 2.4, 6.4, 6.5_
    
    **ユーザー確認ポイント:**
    - [ ] レスポンシブレイアウトが異なる画面サイズで正常に表示されるか（ブラウザで確認）
    - [ ] ファイルアップロード後に全ての可視化コンポーネントが更新されるか
    - [ ] フィルター設定が全コンポーネントに反映されるか
    - [ ] ページリロード後も設定が保持されるか（localStorageの動作確認）
  
  - [ ] 5.3 Implement enhanced file upload functionality
    - Add support for appending new CSV data to existing dataset
    - Implement option to replace or merge data on new upload
    - Add progress indicators for large file processing
    - _Requirements: 6.1, 6.2, 6.3_
    
    **ユーザー確認ポイント:**
    - [ ] 新しいCSVファイルを既存データに追加できるか（ブラウザで確認）
    - [ ] データ置換・マージの選択オプションが動作するか
    - [ ] 大容量ファイル処理時に進行状況が表示されるか
    - [ ] マージ後のデータが正しく統合されているか
  
  - [ ] 5.4 Implement data persistence during development
    - Add Docker volume configuration for data persistence
    - Implement in-memory data storage with container restart handling
    - _Requirements: 4.5_
    
    **ユーザー確認ポイント:**
    - [ ] コンテナ再起動後もアップロードしたデータが保持されるか（`docker compose restart` で確認）
    - [ ] Docker volumeが正しく設定されているか（`docker compose config` で確認）
    - [ ] データの永続化が期待通りに動作するか
  
  - [ ] 5.5 Add comprehensive error handling and loading states
    - Implement Next.js error.tsx for global error boundary
    - Add loading.tsx for page-level loading states
    - Add loading spinners and progress indicators in client components
    - Create user-friendly error messages and validation feedback
    - _Requirements: 1.3_
    
    **ユーザー確認ポイント:**
    - [ ] エラー発生時に適切なエラーメッセージが表示されるか（ブラウザで確認）
    - [ ] ローディング状態が視覚的に分かりやすく表示されるか
    - [ ] ネットワークエラー時の処理が適切に動作するか（バックエンドを停止して確認）
    - [ ] ユーザーフレンドリーなエラー表示になっているか

- [ ] 6. Finalize Docker configuration and comprehensive testing
  - [ ] 6.1 Complete docker-compose configuration for Next.js and Rust backend
    - Ensure both frontend (Next.js with Bun) and backend (Rust) containers build and run correctly
    - Configure Next.js container with proper build optimization
    - Configure Rust container with proper build optimization
    - Verify hot reload functionality works for both services
    - Test volume mounting and data persistence
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_
    
    **ユーザー確認ポイント:**
    - [ ] `docker compose up --build` で全サービスが正常に起動するか
    - [ ] Next.jsコンテナのビルド時間が適切に最適化されているか（`docker compose exec view bun run build`で確認）
    - [ ] Rustコンテナのビルド時間が適切に最適化されているか（`docker compose exec api cargo build --release`で確認）
    - [ ] フロントエンド・バックエンド両方でホットリロードが動作するか（ファイル編集して確認）
    - [ ] 本番用ビルド設定が正しく構成されているか
  
  - [ ] 6.2 Create comprehensive development documentation
    - Write README with setup and usage instructions (including Bun and Docker commands)
    - Document all API endpoints and data formats
    - Add performance benchmarking results and guidelines
    - Add troubleshooting guide for common Docker, Bun, and Rust issues
    - Include all `docker compose exec` command examples
    - _Requirements: 4.3_
    
    **ユーザー確認ポイント:**
    - [ ] READMEの手順に従って新規環境でセットアップできるか（`docker compose up --build`から開始）
    - [ ] API仕様書が実装と一致しているか
    - [ ] パフォーマンスベンチマーク結果が記載されているか（`docker compose exec api cargo bench`の結果）
    - [ ] トラブルシューティングガイドが実用的か
  
  - [ ] 6.3 Add end-to-end testing and performance validation
    - Configure Playwright for E2E testing (`docker compose exec view bun add -d @playwright/test`)
    - Write tests for complete user workflow (upload → visualize → filter)
    - Test Docker environment functionality
    - Validate performance benchmarks meet requirements
    - Test large file upload scenarios (up to 100MB)
    - _Requirements: 1.1, 2.1, 3.1, 8.1, 8.2, 8.3_
    
    **ユーザー確認ポイント:**
    - [ ] E2Eテストが全て成功するか（`docker compose exec view bun run test:e2e`で確認）
    - [ ] ユーザーワークフロー全体が正常に動作するか（ブラウザで手動確認）
    - [ ] 100MBファイルのアップロードテストが成功するか
    - [ ] パフォーマンス要件（応答時間、スループット）を満たしているか（`docker compose exec api cargo bench`で確認）
  
  - [ ] 6.4 Performance optimization and monitoring
    - Implement API response time monitoring
    - Optimize CSV parsing for large files
    - Add memory usage optimization for statistical calculations
    - Implement concurrent request handling validation
    - _Requirements: 8.4, 8.5, 8.6_
    
    **ユーザー確認ポイント:**
    - [ ] API応答時間が要件を満たしているか（`docker compose exec api cargo bench`で確認）
    - [ ] 大容量ファイル処理時のメモリ使用量が適切か（`docker stats`で確認）
    - [ ] 同時リクエスト処理が正常に動作するか（負荷テストツールで確認）
    - [ ] パフォーマンス監視ダッシュボードが機能するか