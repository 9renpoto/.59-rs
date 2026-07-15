# DDD Cloudflare sample

A minimal Rust DDD sample built with Cloudflare Workers, D1, and Leptos CSR. Cloudflare-specific code stays in adapters, while domain rules and use cases remain ordinary, testable Rust crates.

```text
apps/worker ─┐
             ├── crates/application ── crates/domain
apps/web ────┘
```

- `crates/domain`: `GreetingName` validation and Greeting rules
- `crates/application`: the `CreateGreeting` use case and transport-neutral data types
- `apps/worker`: HTTP, D1, and Cloudflare Workers adapter
- `apps/web`: Leptos CSR adapter

## Setup

```bash
mise install
mise run setup
```

Before deploying to Cloudflare, create a D1 database and replace `database_id` in `wrangler.toml` with the returned ID.

```bash
npx wrangler d1 create ddd-cloudflare-sample
```

## Development

```bash
npm run dev
```

| Endpoint | Purpose |
| --- | --- |
| `GET /health` | A readiness check that runs `SELECT 1` against D1. Returns `200 {"status":"ok"}` on success and `503` on failure. |
| `POST /api/greetings` | A sample API that creates a greeting from `{"name":"Ada"}`. |
| Other paths | Serve the Leptos SPA. |

## Commands

```bash
npm run build
npm run check
npm run deploy
```

## License

MIT
