extern crate anyhow;
extern crate openssl;
extern crate serde_json;

use anyhow::{Context, Result};
use openssl::sha::sha256;
use serde_json::Value as Json;

use crate::http::{HTTPHeader, HTTPPayload, HTTPRequest};

/// This is a basic helper struct to hold various challenge informations.
struct Challenge {
    raw: Json,
    input: String,
    difficulty: usize,
}

pub struct AwsChallengeSolver;

impl AwsChallengeSolver {
    const SPOOFED_USER_AGENT: &'static str =
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36";

    // TODO: Since the domain is provided to the method that generates the token,
    // it should be possible to fetch the domain, retrieve the source, obtain the script URL,
    // and generate the token from there. This would make the script resilient to any URL changes.
    const AWS_WAF_BASE_URI: &'static str =
        "https://3f38f7f4f368.a20ab67d.eu-south-2.token.awswaf.com/3f38f7f4f368/e1fcfc58118e/";

    pub fn create_challenge_token(domain: &str) -> Result<String> {
        // In fact, one can use whatever checksum to obtain the token..
        let checksum = &format!("{:08X}", 0);

        let challenge: Challenge = Self::fetch_challenge()?;

        let solution = Self::solve_challenge(&challenge, checksum);

        Self::submit_challenge(domain, solution, &challenge, checksum)
    }

    fn solve_challenge(challenge: &Challenge, checksum: &str) -> u64 {
        let input = &challenge.input;
        let difficulty = challenge.difficulty;

        let base = [input.as_bytes(), checksum.as_bytes()].concat();

        let mut buffer = Vec::with_capacity(base.len() + size_of::<u64>());
        buffer.extend_from_slice(&base);

        let mut solution: u64 = 0;

        loop {
            buffer.truncate(base.len());
            buffer.extend_from_slice(&solution.to_be_bytes());

            let hash_hex = Self::sha256_hex(&buffer);

            if Self::is_difficulty_satisfied(&hash_hex, difficulty) {
                return solution;
            }

            solution += 1;
        }
    }

    fn sha256_hex(data: &[u8]) -> String {
        sha256(data).iter().map(|b| format!("{:02X}", b)).collect()
    }

    fn is_difficulty_satisfied(hash: &str, difficulty: usize) -> bool {
        let mut bits_checked = 0;

        for c in hash.chars() {
            let nibble = c.to_digit(16).unwrap_or(0);

            for bit in (0..4).rev() {
                if bits_checked == difficulty {
                    return true;
                }

                if (nibble >> bit) & 1 != 0 {
                    return false;
                }

                bits_checked += 1;
            }
        }

        bits_checked >= difficulty
    }

    fn fetch_challenge() -> Result<Challenge> {
        let endpoint = &format!("{}/inputs?client=browser", Self::AWS_WAF_BASE_URI);

        let mut header = HTTPHeader::new();
        header.add_field("User-Agent", Self::SPOOFED_USER_AGENT);

        let mut json: Json = HTTPRequest::get_json(endpoint, header)?;

        let input = json["challenge"]["input"]
            .as_str()
            .context("Missing the challenge input from the server response.")?
            .to_string();

        let difficulty = json["difficulty"]
            .as_u64()
            .context("Missing challenge difficulty from the server response.")?
            as usize;

        let challenge = json
            .get_mut("challenge")
            .context("Cannot obtain the full challenge object from the server response.")?
            .take();

        Ok(Challenge {
            raw: challenge,
            input,
            difficulty,
        })
    }

    fn submit_challenge(
        domain: &str,
        solution: u64,
        challenge: &Challenge,
        checksum: &str,
    ) -> Result<String> {
        let solution_str = solution.to_string();
        let challenge_str = challenge.raw.to_string();

        let endpoint = &format!("{}/verify", Self::AWS_WAF_BASE_URI);

        let mut header = HTTPHeader::new();
        header.add_field("User-Agent", Self::SPOOFED_USER_AGENT);

        let mut payload = HTTPPayload::new();
        payload.add_field("challenge", &challenge_str);
        payload.add_field("solution", &solution_str);
        payload.add_field("checksum", checksum);
        payload.add_field("existing_token", "");
        payload.add_field("client", "Browser");
        payload.add_field("domain", domain);
        payload.add_field("signals", "[]");
        payload.add_field("metrics", "[]");

        let json = HTTPRequest::post_json(endpoint, header, payload)?;

        let token = json
            .get("token")
            .and_then(|v| v.as_str())
            .context("Missing aws-waf-token in server response.")?
            .to_string();

        Ok(token)
    }
}
