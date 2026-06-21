# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Build the library
cargo build

# Build with Python bindings
cargo build --features python

# Run all tests
cargo test

# Run a single test
cargo test test_login -- --nocapture

# Run tests in a specific module
cargo test account::test

# Run with debug logging
RUST_LOG=debug cargo test -- --nocapture

# Check compilation (no test run)
cargo check
```

## Architecture Overview

This is a Rust async library wrapping the 编程猫 (Codemao) open platform API. The base URL is `https://api.codemao.cn/`, defined as a constant in `src/lib.rs`.

### Core Pattern: Account + Behavior Traits

- **`Account`** (`src/account.rs`): The central struct holding authentication state (`username`, `password`, `token`, `reqwest::Client`). Created via `Account::new(username, password)`, which logs in and stores the auth token.

- **Behavior Traits**: Four traits define API methods, all implemented on `Account`:
  - `UserBehaviors` — user profile, password, followers, works, honors
  - `CommunityBehavior` — level signature, banners, report reasons
  - `ForumBehavior` — boards, posts, replies, comments, search
  - `WorkBehavior` — work info, unpublish, report, work tree

Usage: import the trait, then call methods directly on an `Account` instance.

### Module Layout

```
src/
├── lib.rs                    # Module declarations, BASE_URL, re-exports
├── account.rs                # Account struct, Error enum, login auth
├── account/user_behavior/
│   ├── mod.rs                # UserBehaviors trait + impl on Account
│   ├── dtos.rs               # Shared DTOs (PageWrapper, FieldTypes, VOs)
│   └── user_behavior_test.rs # Unit tests
├── community/
│   ├── mod.rs
│   ├── community_behavior.rs # CommunityBehavior trait + impl
│   ├── dtos.rs               # SimpleWrapper, BannerItem, ReasonItem
│   ├── banner_type.rs        # Banner type constants
│   └── test.rs
├── forum/
│   ├── mod.rs
│   ├── forum_behavior.rs     # ForumBehavior trait + impl
│   ├── dtos.rs               # BoardItem, PostDetailVO, DTOs
│   └── test.rs
└── works/
    ├── mod.rs
    ├── work_behavior.rs      # WorkBehavior trait + impl
    ├── dtos.rs               # WorkInfoVO, WorkTreeVO, ReportWorkDTO
    └── test.rs
```

### API Conventions

- Requests requiring auth pass `authorization` token via the `Cookie` header: `format!("authorization={}", self.token)`
- Some endpoints are unauthenticated (work info, board info, search) — no cookie header
- Error handling uses the `Error` enum in `account.rs` with `Reqwest` and `Json` variants
- Response types use `Wrapper<T>` (code/msg/data pattern) and `SimpleWrapper<T>` (items pattern) from `community/dtos.rs`, and `PageWrapper<T>` (paginated) from `account/user_behavior/dtos.rs`

### Testing

Tests require a `.env` file with `USERNAME` and `PASSWORD` (and optionally `RUST_LOG=debug`). The `.env` file is in `.gitignore`. Tests use `tokio::test` and `dotenvy` for env loading.
