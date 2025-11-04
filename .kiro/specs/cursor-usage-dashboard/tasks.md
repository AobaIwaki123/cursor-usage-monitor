# Implementation Plan

- [x] 1. Set up project structure and Docker environment
  - Create root directory structure with frontend, backend, and shared folders
  - Create docker-compose.yml with frontend (React) and backend (Rust) services
  - Set up volume mounting for hot reload development
  - Configure environment variables and port mapping
  - _Requirements: 4.1, 4.2, 4.3_
  
  **ユーザー確認ポイント:**
  - [x] プロジェクトフォルダ構造が正しく作成されているか
  - [x] `docker-compose up --build` でエラーなく起動するか
  - [x] フロントエンド (http://localhost:3000) とバックエンド (http://localhost:3001) にアクセス可能か
  - [x] ファイル変更時にホットリロードが動作するか

- [ ] 2. Initialize Rust backend API server
  - [ ] 2.1 Create Rust Axum server project
    - Initialize Cargo.toml with required dependencies (axum, tokio, serde, csv, tower-http)
    - Set up basic Axum server with CORS middleware
    - Create project structure with handlers, models, services, and utils modules
    - _Requirements: 4.1, 4.4_
    
    **ユーザー確認ポイント:**
    - [ ] `cargo build` でエラーなくコンパイルできるか
    - [ ] `cargo run` でサーバーが起動するか
    - [ ] `GET /api/health` エンドポイントが応答するか
    - [ ] CORS設定でフロントエンドからのリクエストが通るか
  
  - [ ] 2.2 Implement CSV upload and parsing endpoints
    - Create POST /api/upload endpoint with multipart file handling
    - Create POST /api/upload/append endpoint for merging new data
    - Implement CSV parsing logic with comprehensive validation
    - Add error handling for invalid CSV formats and file size limits
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 6.1, 6.2, 6.3_
    
    **ユーザー確認ポイント:**
    - [ ] example.csvファイルが正常にアップロード・解析されるか
    - [ ] 無効なCSVファイルで適切なエラーメッセージが返されるか
    - [ ] ファイルサイズ制限（100MB）が正しく動作するか
    - [ ] `/api/upload/append` で既存データにマージできるか
  
  - [ ] 2.3 Create data processing and summary calculation services
    - Implement UsageData struct and parsing logic
    - Calculate usage summary statistics (total cost, model breakdown, etc.)
    - Add model statistics calculation with cache efficiency
    - Implement data merging logic for append functionality
    - _Requirements: 2.1, 3.1, 3.4, 5.2, 5.3, 6.4_
    
    **ユーザー確認ポイント:**
    - [ ] アップロード後に正確な統計サマリーが返されるか
    - [ ] 総コスト、総トークン数が正しく計算されているか
    - [ ] モデル別の統計が適切に分類されているか
    - [ ] キャッシュ効率が正しく計算されているか
  
  - [ ] 2.4 Implement comprehensive statistics calculation
    - Create services for peak usage analysis (hours, days)
    - Implement cost efficiency metrics calculation
    - Add usage trend analysis and growth rate calculation
    - Calculate usage percentiles and distribution statistics
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_
    
    **ユーザー確認ポイント:**
    - [ ] `/api/stats/comprehensive` エンドポイントが正常に応答するか
    - [ ] ピーク使用時間・日付が正しく特定されているか
    - [ ] コスト効率メトリクス（トークンあたりコスト等）が計算されているか
    - [ ] 使用量のパーセンタイル（中央値、95%等）が算出されているか
  
  - [ ] 2.5 Write backend API tests and performance benchmarks
    - Create unit tests for CSV parsing functions
    - Write integration tests for upload endpoints
    - Add error handling test cases
    - Implement performance benchmarks using criterion crate
    - Add load testing for concurrent file uploads
    - Create memory usage profiling tests for large datasets
    - _Requirements: 1.3, 8.1, 8.2, 8.3, 8.4, 8.5, 8.6_
    
    **ユーザー確認ポイント:**
    - [ ] `cargo test` で全テストがパスするか
    - [ ] `cargo bench` でベンチマークが実行されるか
    - [ ] 大容量ファイル（100MB）のテストが成功するか
    - [ ] 同時アップロードのテストが正常に動作するか

- [ ] 3. Initialize frontend React application
  - [ ] 3.1 Create React TypeScript project with Vite
    - Initialize package.json with React, TypeScript, Vite, and Tailwind CSS
    - Set up Vite configuration for development server
    - Configure Tailwind CSS and basic styling setup
    - _Requirements: 4.4_
    
    **ユーザー確認ポイント:**
    - [ ] `npm install` でエラーなく依存関係がインストールされるか
    - [ ] `npm run dev` で開発サーバーが起動するか
    - [ ] TypeScriptのコンパイルエラーがないか
    - [ ] Tailwind CSSのスタイルが適用されるか
  
  - [ ] 3.2 Create shared TypeScript interfaces
    - Define UsageData, UsageSummary, and ModelStats interfaces
    - Create API response types and error handling types
    - Set up type definitions for chart data structures
    - _Requirements: 1.4, 2.1, 3.1, 5.1_
    
    **ユーザー確認ポイント:**
    - [ ] TypeScript型定義がRustの構造体と一致しているか
    - [ ] APIレスポンスの型チェックが正常に動作するか
    - [ ] インポート・エクスポートが正しく設定されているか
    - [ ] 型安全性が保たれているか（コンパイルエラーなし）
  
  - [ ] 3.3 Implement file upload component
    - Create FileUpload component with drag-and-drop functionality
    - Add file validation (CSV format, size limits)
    - Implement upload progress and error state handling
    - _Requirements: 1.1, 1.2, 1.3_
    
    **ユーザー確認ポイント:**
    - [ ] ドラッグ&ドロップでファイルアップロードできるか
    - [ ] ファイル選択ボタンが正常に動作するか
    - [ ] CSV以外のファイルで適切なエラーが表示されるか
    - [ ] アップロード進行状況が表示されるか

- [ ] 4. Implement data visualization components with enhanced features
  - [ ] 4.1 Create token usage time-series chart with granularity controls
    - Install and configure Recharts library
    - Implement TokenUsageChart component with line chart
    - Add daily/hourly granularity toggle functionality
    - Add hover tooltips with detailed token information
    - Implement date range filtering functionality
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6_
    
    **ユーザー確認ポイント:**
    - [ ] 時系列チャートが正しく描画されるか
    - [ ] 日毎・時間ごとの切り替えが動作するか
    - [ ] ホバー時に詳細情報が表示されるか
    - [ ] 日付範囲フィルターが正常に機能するか
  
  - [ ] 4.2 Create cost breakdown visualization with model filtering
    - Implement CostBreakdownChart with pie chart for model costs
    - Add toggle between individual model view and aggregated overall view
    - Create daily cost trends line chart
    - Add cost summary cards with total and average calculations
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_
    
    **ユーザー確認ポイント:**
    - [ ] モデル別コスト内訳の円グラフが表示されるか
    - [ ] 個別モデル・全体表示の切り替えが動作するか
    - [ ] 日別コストトレンドが正しく描画されるか
    - [ ] コストサマリーカードに正確な数値が表示されるか
  
  - [ ] 4.3 Create model usage statistics display with advanced filtering
    - Implement ModelStatsTable component with sorting and filtering
    - Add model usage frequency bar chart
    - Implement toggle between individual model and aggregated views
    - Calculate and display cache efficiency metrics
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6_
    
    **ユーザー確認ポイント:**
    - [ ] モデル統計テーブルのソート・フィルター機能が動作するか
    - [ ] モデル使用頻度の棒グラフが表示されるか
    - [ ] 個別・集約表示の切り替えが正常に機能するか
    - [ ] キャッシュ効率メトリクスが正確に表示されるか
  
  - [ ] 4.4 Create comprehensive statistics dashboard
    - Implement peak usage hours and days visualization
    - Create cost efficiency metrics display (cost per token, per request)
    - Add usage trends visualization with growth rate indicators
    - Display cache performance and savings metrics
    - Create model comparison charts for performance analysis
    - Show usage distribution percentiles (median, 95th, 99th)
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_
    
    **ユーザー確認ポイント:**
    - [ ] ピーク使用時間・日付が視覚的に表示されるか
    - [ ] コスト効率メトリクス（トークンあたりコスト等）が表示されるか
    - [ ] 使用傾向と成長率が適切に可視化されるか
    - [ ] パーセンタイル分布が正確に表示されるか

- [ ] 5. Integrate components and implement enhanced dashboard layout
  - [ ] 5.1 Create main Dashboard component with advanced state management
    - Implement responsive grid layout for all visualization components
    - Add state management for uploaded data, filters, and view preferences
    - Connect file upload with data visualization components
    - Implement data merging functionality for new CSV uploads
    - Add view preference persistence (granularity, model filters)
    - _Requirements: 1.4, 2.4, 6.4, 6.5_
    
    **ユーザー確認ポイント:**
    - [ ] レスポンシブレイアウトが異なる画面サイズで正常に表示されるか
    - [ ] ファイルアップロード後に全ての可視化コンポーネントが更新されるか
    - [ ] フィルター設定が全コンポーネントに反映されるか
    - [ ] ページリロード後も設定が保持されるか
  
  - [ ] 5.2 Implement enhanced file upload functionality
    - Add support for appending new CSV data to existing dataset
    - Implement option to replace or merge data on new upload
    - Add progress indicators for large file processing
    - _Requirements: 6.1, 6.2, 6.3_
    
    **ユーザー確認ポイント:**
    - [ ] 新しいCSVファイルを既存データに追加できるか
    - [ ] データ置換・マージの選択オプションが動作するか
    - [ ] 大容量ファイル処理時に進行状況が表示されるか
    - [ ] マージ後のデータが正しく統合されているか
  
  - [ ] 5.3 Implement data persistence during development
    - Add Docker volume configuration for data persistence
    - Implement in-memory data storage with container restart handling
    - _Requirements: 4.5_
    
    **ユーザー確認ポイント:**
    - [ ] コンテナ再起動後もアップロードしたデータが保持されるか
    - [ ] Docker volumeが正しく設定されているか
    - [ ] データの永続化が期待通りに動作するか
  
  - [ ] 5.4 Add comprehensive error handling and loading states
    - Implement global error boundary for React components
    - Add loading spinners and progress indicators
    - Create user-friendly error messages and validation feedback
    - _Requirements: 1.3_
    
    **ユーザー確認ポイント:**
    - [ ] エラー発生時に適切なエラーメッセージが表示されるか
    - [ ] ローディング状態が視覚的に分かりやすく表示されるか
    - [ ] ネットワークエラー時の処理が適切に動作するか
    - [ ] ユーザーフレンドリーなエラー表示になっているか

- [ ] 6. Finalize Docker configuration and comprehensive testing
  - [ ] 6.1 Complete docker-compose configuration for Rust backend
    - Ensure both frontend (React) and backend (Rust) containers build and run correctly
    - Configure Rust container with proper build optimization
    - Verify hot reload functionality works for both services
    - Test volume mounting and data persistence
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_
    
    **ユーザー確認ポイント:**
    - [ ] `docker-compose up --build` で全サービスが正常に起動するか
    - [ ] Rustコンテナのビルド時間が適切に最適化されているか
    - [ ] フロントエンド・バックエンド両方でホットリロードが動作するか
    - [ ] 本番用ビルド設定が正しく構成されているか
  
  - [ ] 6.2 Create comprehensive development documentation
    - Write README with setup and usage instructions
    - Document all API endpoints and data formats
    - Add performance benchmarking results and guidelines
    - Add troubleshooting guide for common Docker and Rust issues
    - _Requirements: 4.3_
    
    **ユーザー確認ポイント:**
    - [ ] READMEの手順に従って新規環境でセットアップできるか
    - [ ] API仕様書が実装と一致しているか
    - [ ] パフォーマンスベンチマーク結果が記載されているか
    - [ ] トラブルシューティングガイドが実用的か
  
  - [ ] 6.3 Add end-to-end testing and performance validation
    - Configure Playwright for E2E testing
    - Write tests for complete user workflow (upload → visualize → filter)
    - Test Docker environment functionality
    - Validate performance benchmarks meet requirements
    - Test large file upload scenarios (up to 100MB)
    - _Requirements: 1.1, 2.1, 3.1, 8.1, 8.2, 8.3_
    
    **ユーザー確認ポイント:**
    - [ ] E2Eテストが全て成功するか
    - [ ] ユーザーワークフロー全体が正常に動作するか
    - [ ] 100MBファイルのアップロードテストが成功するか
    - [ ] パフォーマンス要件（応答時間、スループット）を満たしているか
  
  - [ ] 6.4 Performance optimization and monitoring
    - Implement API response time monitoring
    - Optimize CSV parsing for large files
    - Add memory usage optimization for statistical calculations
    - Implement concurrent request handling validation
    - _Requirements: 8.4, 8.5, 8.6_
    
    **ユーザー確認ポイント:**
    - [ ] API応答時間が要件を満たしているか（目標値以下）
    - [ ] 大容量ファイル処理時のメモリ使用量が適切か
    - [ ] 同時リクエスト処理が正常に動作するか
    - [ ] パフォーマンス監視ダッシュボードが機能するか