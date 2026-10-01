use worker::*;
use serde_json::json;

const MARKER: &str = "SERVERLESS_BUILD_RATE_LIMITING_RUST_V1";

#[event(fetch)]
async fn fetch(request: Request, env: Env, _context: Context) -> Result<Response> {
    let url = request.url()?;
    if request.method() == Method::Get && url.path() == "/" {
        return Response::from_json(&json!({"pattern": "Workers Rate Limiting binding", "marker": MARKER,
            "endpoints": ["GET /limited?actor=...", "GET /health"],
            "limit": "5 requests per 10 seconds per actor, per Cloudflare location"}));
    }
    if request.method() == Method::Get && url.path() == "/health" {
        return Response::from_json(&json!({"ok": true, "marker": MARKER}));
    }
    if request.method() != Method::Get || url.path() != "/limited" {
        return Ok(Response::from_json(&json!({"error": "Not found"}))?.with_status(404));
    }
    // DEMO identity only: use a verified user/tenant ID in your application.
    let actor = url.query_pairs().find(|(key, _)| key == "actor").map(|(_, value)| value.into_owned()).unwrap_or_default();
    if actor.is_empty() || actor.len() > 40 || !actor.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_') {
        return Ok(Response::from_json(&json!({"error": "Provide an actor of 1–40 letters, numbers, hyphens, or underscores."}))?.with_status(400));
    }
    let outcome = env.rate_limiter("RATE_LIMITER")?.limit(format!("demo:{actor}:/limited")).await?;
    let data = if outcome.success { json!({"allowed": true, "actor": actor, "marker": MARKER}) }
        else { json!({"error": "Rate limit exceeded", "actor": actor, "marker": MARKER}) };
    let headers = Headers::new();
    headers.set("content-type", "application/json")?;
    headers.set("cache-control", "no-store")?;
    Ok(Response::from_json(&data)?.with_status(if outcome.success {200} else {429}).with_headers(headers))
}
