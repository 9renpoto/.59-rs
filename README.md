# DDD Cloudflare sample

Cloudflare Workers、D1、Leptos CSR を使った最小の Rust DDD サンプルです。Cloudflare 固有のコードを adapter に閉じ込め、ドメインルールとユースケースは通常の Rust crate としてテストできます。

```text
apps/worker ─┐
             ├── crates/application ── crates/domain
apps/web ────┘
```

- `crates/domain`: `GreetingName` の入力検証と Greeting のルール
- `crates/application`: `CreateGreeting` ユースケースと API の入出力型
- `apps/worker`: HTTP、D1、Cloudflare Workers adapter
- `apps/web`: Leptos CSR adapter

## Setup

```bash
mise install
mise run setup
```

Cloudflare にデプロイする前に D1 を作成し、`wrangler.toml` の `database_id` を作成結果の ID に置き換えます。

```bash
npx wrangler d1 create ddd-cloudflare-sample
```

## Development

```bash
npm run dev
```

| Endpoint | Purpose |
| --- | --- |
| `GET /health` | D1 に `SELECT 1` を実行する readiness check。成功時は `200 {"status":"ok"}`、失敗時は `503`。 |
| `POST /api/greetings` | `{"name":"Ada"}` から Greeting を作成するサンプル API。 |
| Other paths | Leptos の SPA を返す。 |

## Commands

```bash
npm run build
npm run check
npm run deploy
```

## License

MIT
