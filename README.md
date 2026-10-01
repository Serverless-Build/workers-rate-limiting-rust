# Rate Limiting — Rust, Worker and Wrangler

Use Cloudflare's native Workers Rate Limiting binding to protect a Worker route. No KV counter or custom token bucket is involved.

## Pattern At A Glance

| | |
|---|---|
| Difficulty | Beginner |
| Build time | 5 minutes |
| Runtime | Cloudflare Workers |
| Language | Rust |
| Framework | No framework |
| Data store | None |

## What It Implements

- A native `RATE_LIMITER` binding with a five-call, ten-second per-location budget.
- An actor-scoped, route-specific call to `limit()` before running the Worker handler.
- A 429 response after the binding reports a rejection; `/health` remains unthrottled.

## Where It's Applicable

- Limiting expensive application operations after the Worker has started.
- Applying different budgets to specific customers or routes in a Workers API.
- Building application-level protection without writing your own rate limiter.

## How It Works

1. The demo client chooses an actor for an isolated trial; a real application must derive the actor from verified credentials.
2. Worker validates the input and calls `env.rate_limiter("RATE_LIMITER")?.limit(key).await?` for `/limited` only.
3. The native binding returns `success`; rejected requests return 429, allowed requests reach the route.

## Prerequisites

- Node.js 22+, npm, Rust, and the wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`).
- A Cloudflare account for permanent deployment.

## Setup

Run `npm install` in this directory. `namespace_id` is a positive integer string unique to your account; change it when combining this example with another rate-limit binding.

## Run Locally

Run `npm run dev`. The local Worker is available at `http://localhost:8787`.

## Deploy Remotely

Authenticate Wrangler and run `npm run deploy`. The Worker will receive a `workers.dev` URL.

## Test Locally

Request `http://localhost:8787/` to inspect the routes. Call `http://localhost:8787/limited?actor=try-1` repeatedly to see allowed responses followed by HTTP 429; wait for the window, then retry.

## Test Remotely

Use the same paths at your deployed URL, or use the API console on the pattern page. Requests may land in different Cloudflare locations; allow the native limiter a moment to propagate within a location.

## Watch Out For

- The actor query parameter is **demo-only**. An untrusted caller can change it and bypass the demonstration. Use a verified user ID, tenant ID, or API key in your own application; avoid IP addresses as a primary identifier.
- Enforcement is per Cloudflare location and intentionally permissive/eventually consistent; do not use it for billing or exact global accounting.
- `limit()` returns only `success`. It does not expose remaining tokens or an exact reset time; this example does not fabricate them.

## Production Fit

Cloudflare recommends the native binding for application-level throttling. Change the demo identity to a trusted authenticated actor, select budgets appropriate to the cost of your operation, and use WAF Rate Limiting Rules when protection must run before the Worker.

## Pattern and live demo

- [Pattern page](https://serverless.build/patterns/rate-limiting)
- [Live deployment](https://workers-rate-limiting-rust.dwarven.workers.dev)
