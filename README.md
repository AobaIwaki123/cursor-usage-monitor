# Cursor Usage Dashboard

CursorのAPI使用状況を可視化する高性能Webダッシュボードアプリケーション

## 概要

Cursor Usage Dashboardは、CursorのAPIやモデル使用データをCSVファイルから読み込み、直感的でインタラクティブなダッシュボードで可視化するWebアプリケーションです。トークン使用量、コスト分析、使用パターンの時系列分析など、包括的な統計情報を提供します。

## 主要機能

### データ可視化
- **時系列分析**: 日毎・時間ごとの切り替え可能な使用量グラフ
- **コスト分析**: モデル別コスト内訳とトレンド分析
- **モデル統計**: 個別モデル・全体表示の切り替え機能
- **包括的統計**: ピーク使用時間、効率性メトリクス、使用分布

### データ管理
- **CSVファイルアップロード**: ドラッグ&ドロップ対応
- **データマージ**: 新しいCSVファイルの追加・統合機能
- **リアルタイム更新**: アップロード後の即座な可視化更新
- **大容量ファイル対応**: 最大100MBのCSVファイル処理

### 高度な分析機能
- **ピーク使用分析**: 最も使用量の多い時間帯・日付の特定
- **コスト効率性**: トークンあたり・リクエストあたりのコスト計算
- **使用傾向**: 成長率と使用パターンの分析
- **キャッシュ効率**: キャッシュヒット率とコスト削減効果
- **パーセンタイル分析**: 中央値、95パーセンタイル使用量

## 技術スタック

### フロントエンド
- **React 18** + **TypeScript**: モダンなUI開発
- **Vite**: 高速開発サーバーとHMR
- **Recharts**: データ可視化ライブラリ
- **Tailwind CSS**: ユーティリティファーストCSS
- **Axios**: HTTP通信

### バックエンド
- **Rust** + **Axum**: 高性能Webフレームワーク
- **Tokio**: 非同期ランタイム
- **Serde**: シリアライゼーション
- **CSV crate**: 高速CSV処理
- **Tower-HTTP**: CORS・ミドルウェア

### 開発環境
- **Docker** + **Docker Compose**: 環境分離
- **ホットリロード**: 開発効率向上
- **Volume mounting**: データ永続化

## Rust + Axumを採用する理由

### パフォーマンス比較

| 技術スタック | CSV処理速度 | メモリ使用量 | 同時接続数 | 開発効率 |
|-------------|------------|------------|-----------|---------|
| **Rust + Axum** | 最高 | 最高 | 最高 | 良好 |
| Node.js + Express | 良好 | 良好 | 良好 | 最高 |
| Python + FastAPI | 普通 | 普通 | 良好 | 優秀 |
| Go + Gin | 優秀 | 優秀 | 優秀 | 優秀 |

### Rustの優位性

#### **圧倒的なパフォーマンス**
- **ゼロコスト抽象化**: ランタイムオーバーヘッドなし
- **メモリ安全性**: ガベージコレクションなしで高速動作
- **並行処理**: Tokioによる効率的な非同期処理

#### **大容量データ処理**
- **100MB CSVファイル**: 高速解析対応
- **10,000+レコード**: 効率的な統計計算
- **低メモリフットプリント**: メモリ効率に優れた設計

#### **信頼性**
- **コンパイル時エラー検出**: 実行時エラーの大幅削減
- **型安全性**: バグの事前防止
- **メモリリーク防止**: 自動メモリ管理

### Axumフレームワークの特徴

#### **高性能**
```rust
// 非同期処理による高いスループット
async fn upload_csv(
    multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    // 並行CSV処理
}
```

#### **開発者体験**
- **型安全なルーティング**: コンパイル時の検証
- **ミドルウェア統合**: tower エコシステム活用
- **テスト容易性**: 組み込みテストサポート

#### **スケーラビリティ**
- **軽量スレッド**: 数万の同時接続対応
- **効率的リソース使用**: CPUとメモリの最適化

## 技術選択の理由

### Rustの特徴
- **メモリ安全性**: ガベージコレクションなしでメモリリークを防止
- **ゼロコスト抽象化**: 高レベルな機能を低レベルなパフォーマンスで実現
- **並行処理**: Tokioによる効率的な非同期処理
- **型安全性**: コンパイル時エラー検出によるバグ防止

### 他言語との比較観点
- **Node.js**: 豊富なエコシステム vs Rustの型安全性・パフォーマンス
- **Python**: 開発速度 vs Rustの実行速度・メモリ効率
- **Go**: シンプルさ vs Rustのより厳密な型システム・メモリ安全性

## Rust Webフレームワーク比較

### なぜAxumを選択したか

| フレームワーク | パフォーマンス | 学習コスト | エコシステム | 非同期サポート | 型安全性 |
|---------------|---------------|-----------|-------------|---------------|---------|
| **Axum** | 最高 | 良好 | 最高 | 最高 | 最高 |
| Actix-web | 最高 | 普通 | 優秀 | 優秀 | 優秀 |
| Warp | 優秀 | 普通 | 良好 | 最高 | 最高 |
| Rocket | 良好 | 最高 | 良好 | 良好 | 優秀 |
| Tide | 良好 | 優秀 | 普通 | 優秀 | 良好 |

### 詳細比較

#### **Axum vs Actix-web**
```rust
// Axum - シンプルで型安全
async fn upload_csv(
    multipart: Multipart,
) -> Result<Json<Response>, AppError> {
    // 型安全なハンドラー
}

// Actix-web - より複雑な設定
async fn upload_csv(
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    // より多くのボイラープレート
}
```

**Axumの優位性:**
- **Tokio統合**: ネイティブなTokioサポート
- **型安全性**: コンパイル時のルート検証
- **シンプルAPI**: 学習コストの低減
- **Tower統合**: 豊富なミドルウェア

**Actix-webの特徴:**
- **最高性能**: ベンチマークでトップクラス
- **成熟度**: 長期間の実績
- **機能豊富**: 多くの組み込み機能

#### **Axum vs Warp**
```rust
// Axum - 直感的なルーティング
let app = Router::new()
    .route("/upload", post(upload_csv))
    .route("/stats", get(get_stats));

// Warp - 関数型アプローチ
let upload = warp::path("upload")
    .and(warp::post())
    .and(warp::multipart::form())
    .and_then(upload_csv);
```

**Axumの優位性:**
- **直感的API**: 従来のWebフレームワークに近い
- **エラーハンドリング**: より簡潔なエラー処理
- **開発体験**: IDEサポートが優秀

**Warpの特徴:**
- **関数型**: 関数合成によるルート定義
- **型レベル**: 高度な型システム活用
- **軽量**: 最小限の依存関係

#### **Axum vs Rocket**
```rust
// Axum - 手動設定
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(upload_csv));
    
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Rocket - 自動設定
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![upload_csv])
        .launch()
        .await
}
```

**Axumの優位性:**
- **非同期ファースト**: Tokioネイティブ
- **パフォーマンス**: より高速な処理
- **柔軟性**: 細かい制御が可能

**Rocketの特徴:**
- **開発効率**: 自動設定とマクロ
- **学習容易**: Rails風の規約
- **機能統合**: ORM、テンプレート統合

### 本プロジェクトでのAxum選択理由

#### **CSV処理に最適**
```rust
// 大容量ファイル処理
async fn process_large_csv(
    mut multipart: Multipart,
) -> Result<Json<ProcessResult>, AppError> {
    while let Some(field) = multipart.next_field().await? {
        // ストリーミング処理で メモリ効率最大化
        let stream = field.chunk().await?;
        // 非同期でCSV解析
    }
}
```

#### **統計計算パフォーマンス**
```rust
// 並列統計計算
async fn calculate_comprehensive_stats(
    data: Vec<UsageData>
) -> ComprehensiveStats {
    let (peak_stats, cost_stats, trend_stats) = tokio::join!(
        calculate_peak_usage(&data),
        calculate_cost_efficiency(&data),
        calculate_usage_trends(&data)
    );
    // 3つの計算を並列実行
}
```

#### **リアルタイム更新**
```rust
// WebSocket統合（将来拡張）
async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| async {
        // リアルタイムデータ更新
    })
}
```

### パフォーマンス評価

#### **ベンチマーク計画**
パフォーマンステスト実装後に以下の指標を測定予定:

```
CSV解析性能 (10MB ファイル):
- [実装後に測定結果を記載]

API応答時間 (1000並行リクエスト):
- [実装後に測定結果を記載]

メモリ使用量 (アイドル時):
- [実装後に測定結果を記載]
```

#### **開発効率指標**
```
コンパイル時間:
- [実装後に測定結果を記載]

学習コスト:
- [実装後に評価結果を記載]
```

### 結論

**Axumを選択した決定的理由:**

1. **バランス**: パフォーマンスと開発効率の最適バランス
2. **エコシステム**: Tokio/Towerとの完全統合
3. **将来性**: 活発な開発とコミュニティサポート
4. **型安全性**: コンパイル時エラー検出による品質向上
5. **CSV処理**: 大容量ファイル処理に最適化された非同期I/O

このプロジェクトの要件（大容量CSV処理、リアルタイム統計計算、高い同時接続数）において、Axumが最も適切な選択となります。

## クイックスタート

### 前提条件
- Docker & Docker Compose
- Git

### セットアップ

1. **リポジトリクローン**
```bash
git clone <repository-url>
cd cursor-usage-dashboard
```

2. **アプリケーション起動**
```bash
docker-compose up --build
```

3. **アクセス**
- フロントエンド: http://localhost:3000
- バックエンドAPI: http://localhost:3001

### 使用方法

1. **CSVファイル準備**
   - Cursorから使用データをCSV形式でエクスポート
   - 必要な列: Date, Kind, Model, Max Mode, Input tokens, Cache Read, Output Tokens, Total Tokens, Cost

2. **データアップロード**
   - ダッシュボードでCSVファイルをドラッグ&ドロップ
   - または「ファイル選択」ボタンでアップロード

3. **データ分析**
   - 時系列グラフで使用パターンを確認
   - コスト分析でモデル別支出を把握
   - 統計情報で効率性を評価

## パフォーマンス指標

### ベンチマーク結果
パフォーマンステスト実装後に以下の指標を測定予定:

```
CSV解析性能:
- 1MB ファイル: [測定予定]
- 10MB ファイル: [測定予定]
- 100MB ファイル: [測定予定]

API応答時間:
- アップロード: [測定予定]
- 統計計算: [測定予定]
- データ取得: [測定予定]

同時接続:
- 最大同時アップロード: [測定予定]
- API スループット: [測定予定]
```

### メモリ使用量
```
データセットサイズ別:
- 1,000レコード: [測定予定]
- 10,000レコード: [測定予定]
- 100,000レコード: [測定予定]
```

詳細なベンチマーク結果は、パフォーマンステスト実装完了後に更新されます。

## 開発

### ローカル開発
```bash
# フロントエンド開発
cd frontend
npm install
npm run dev

# バックエンド開発
cd backend
cargo run

# テスト実行
cargo test
npm test
```

### パフォーマンステスト
```bash
# ベンチマーク実行
cd backend
cargo bench

# 負荷テスト
cd tests
./load_test.sh
```

パフォーマンステストの詳細については、[パフォーマンステスト仕様書](.kiro/specs/cursor-usage-dashboard/requirements.md#requirement-8)を参照してください。

## API仕様

### エンドポイント

#### `POST /api/upload`
CSVファイルのアップロードと解析
```rust
Request: multipart/form-data
Response: {
  success: bool,
  data: Vec<UsageData>,
  summary: UsageSummary
}
```

#### `POST /api/upload/append`
既存データへの新しいCSVデータ追加
```rust
Request: multipart/form-data
Response: {
  success: bool,
  data: Vec<UsageData>,
  summary: UsageSummary
}
```

#### `GET /api/stats/comprehensive`
包括的統計情報の取得
```rust
Response: {
  peak_usage: PeakUsageStats,
  cost_efficiency: CostEfficiencyStats,
  usage_trends: UsageTrendStats,
  cache_performance: CacheStats,
  model_comparison: Vec<ModelComparisonStats>
}
```

## プロジェクト構造

```
cursor-usage-dashboard/
├── README.md
├── docker-compose.yml
├── .kiro/
│   └── specs/
│       └── cursor-usage-dashboard/
│           ├── requirements.md
│           ├── design.md
│           └── tasks.md
├── frontend/                 # React + TypeScript
│   ├── Dockerfile
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── components/       # UI コンポーネント
│       ├── types/           # TypeScript 型定義
│       ├── utils/           # ユーティリティ関数
│       └── hooks/           # カスタムフック
├── backend/                 # Rust + Axum
│   ├── Dockerfile
│   ├── Cargo.toml
│   └── src/
│       ├── handlers/        # API ハンドラー
│       ├── models/          # データモデル
│       ├── services/        # ビジネスロジック
│       └── utils/           # ユーティリティ
└── shared/                  # 共通型定義
    └── types/
```

## ライセンス

MIT License

## 貢献

プルリクエストやイシューの報告を歓迎します。

## サポート

- Email: [your-email]
- Issues: [GitHub Issues]
- Documentation: [Wiki]