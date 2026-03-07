# IronClaw Security Reference (Internal)

Date: 2026-03-07
Source: `nearai/ironclaw` `main` @ `3079043` (fetched 2026-03-07)

## Outcome

Decision: **Compose + Extend**.

We should not copy IronClaw wholesale. Its strongest advantages come from a small set of security primitives that can be transplanted into LoopForge without inheriting its heavier gateway/orchestrator/database shape.

Recommended imports for LoopForge:

1. **Explicit outbound egress policy** for networked tools
2. **Encrypted secret resolution** with a host-only access boundary
3. **Leak guard** before tool output is persisted, displayed, or audited
4. **Threat-model documentation** that matches the real code paths

Do **not** copy as-is:

- PostgreSQL + pgvector as the default baseline
- full gateway/orchestrator/worker architecture
- broad multi-channel scope that dilutes the software-delivery product focus
- NEAR-specific onboarding and auth assumptions

## Evidence from IronClaw

### 1. Security is modeled as a system, not a flag

IronClaw maintains a dedicated network security inventory describing bind addresses, auth methods, trust boundaries, and known assumptions. This gives contributors a concrete review baseline instead of vague “be safe” guidance.

Observed in:
- `src/NETWORK_SECURITY.md`
- `README.md` security section

### 2. Network access is governed by allowlists, not a single private-network switch

IronClaw’s WASM HTTP layer validates:
- scheme
- host
- path prefix
- HTTP method
- URL normalization edge cases such as `userinfo` and encoded separators

This is materially stronger than a binary `allow_private` gate.

Observed in:
- `src/tools/wasm/allowlist.rs`

### 3. Secrets stay on the host side

IronClaw separates secret storage from tool execution. It stores a master key in the system keychain, encrypts secret values, and treats tool access as boundary-mediated injection rather than direct exposure.

Observed in:
- `src/secrets/keychain.rs`
- `src/secrets/crypto.rs`
- `src/secrets/store.rs`

### 4. Output is scanned for leaks

IronClaw scans content for likely secrets before output leaves the secure boundary. It supports `block`, `redact`, and `warn` actions, which is a stronger posture than only relying on operator discipline.

Observed in:
- `src/safety/leak_detector.rs`
- `src/safety/policy.rs`

### 5. Sandbox posture is explicit

IronClaw’s sandbox manager treats containerized execution as an isolated runtime with initialization, image checks, proxy startup, and policy-based execution behavior.

Observed in:
- `src/sandbox/manager.rs`

### 6. Engineering discipline is visible in docs

IronClaw keeps a living parity matrix and architecture guide so contributors can reason about what exists, what is partial, and what is intentionally out of scope.

Observed in:
- `FEATURE_PARITY.md`
- `CLAUDE.md`

## Current LoopForge Gaps

### Strong enough already

- Approval gating for risky tools and private-network access exists in `crates/rexos-runtime/src/approval.rs`
- Daemon auth + rate limiting exists in `crates/rexos-daemon/src/lib.rs`
- `web_fetch` rejects non-http schemes and blocks loopback/private access by default in `crates/rexos-tools/src/ops/web/remote.rs`
- Tool and skill audits already exist in `crates/rexos-runtime/src/records.rs`

### Still weak compared with IronClaw

1. **No encrypted secret store**
   - provider credentials are still environment-variable driven
   - no system-keychain or host-managed secret abstraction exists

2. **Outbound policy is coarse**
   - `allow_private` is too blunt for long-term safety
   - there is no host/path/method allowlist per tool or per workflow

3. **No leak guard in the runtime output path**
   - sensitive output can still be persisted or rendered if a tool/model echoes it

4. **No central trust-boundary document**
   - the real rules are spread across runtime/tests/docs rather than expressed as one internal reference

## Recommended Adoption Order

### P0: security primitives with the best risk reduction / complexity ratio

1. **SecretStore**
   - host-managed secret resolution
   - env fallback for compatibility
   - no raw secret values exposed through tool output or doctor output

2. **EgressPolicy**
   - reusable validator for `web_fetch`, A2A, and browser navigation
   - allowlist by tool + host + path prefix + method
   - preserve `allow_private` only as an escape hatch, not the main model

3. **LeakGuard**
   - scan and optionally redact/block output before user display, ACP event capture, or audit persistence

### P1: observability and operator ergonomics

4. **SecurityDoctor**
   - report whether egress policy is configured
   - report whether provider credentials are env-only or sealed
   - report whether leak guard is enabled

5. **ThreatModel doc**
   - one internal file that maps LoopForge bind/auth/rate-limit/private-network rules to source files

## Non-Goals for the Next Iteration

- replacing the current local-first architecture with a web gateway/orchestrator system
- adopting PostgreSQL or pgvector as a baseline dependency
- copying IronClaw’s channel breadth
- implementing a full WASM runtime before the core security primitives exist

## Recommended Direction

Use IronClaw as a **security architecture reference**, not a product template.

For LoopForge, the right move is:
- keep the software-delivery positioning
- keep the current workspace-first UX
- import the strongest host-boundary and egress-control ideas
- document the trust model so future features cannot silently weaken it
