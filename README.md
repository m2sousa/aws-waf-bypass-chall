# Rust-based AWS WAF Challenge Bypass (PoC)

This repository contains a minimal Rust proof of concept bypassing AWS Web Application Firewall (WAF) challenge usually iniated by `challenge.js`.

## Overview

Modern AWS WAF deployments often rely on browser-based challenges (e.g., JavaScript execution, token validation, or behavioral checks) to distinguish legitimate users from automated traffic.
Based on some reverse-engineering work, we reproduce the challenge flow handled by `challenge.js` and replicate its behaviour in Rust to obtain the aws-waf-token.

## Usage

```rust
mod http;
mod waf;

fn main() {
    // Your domain of interest here. Use only authorized targets.
    let domain: &'static str = "www.example.com";

    if let Ok(token) = waf::AwsChallengeSolver::create_challenge_token(domain) {
        println!("aws-waf-token : {}", token);
    }
}
```

## Build & Run

```bash
cargo run --release
```

## Disclaimer

This project is intended **strictly for educational and security research purposes**.
Do not use this code against systems without **explicit authorization**. Unauthorized use may violate laws and terms of service.
