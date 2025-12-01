use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::path::Path;

const AOC_URL: &str = "https://adventofcode.com";

pub struct AocClient {
    client: Client,
    year: u32,
    session_token: String,
}

impl AocClient {
    pub fn new(year: u32) -> Result<Self> {
        let session_token = env::var("AOC_SESSION").context(
            "AOC_SESSION environment variable not set. Get it from your browser cookies.",
        )?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::COOKIE,
            format!("session={}", session_token).parse()?,
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            year,
            session_token,
        })
    }

    pub fn get_input(&self, day: u32) -> Result<String> {
        // Check if input already exists locally
        let input_path = format!("inputs/day{:02}.txt", day);
        if Path::new(&input_path).exists() {
            println!("Using cached input from {}", input_path);
            return Ok(fs::read_to_string(&input_path)?);
        }

        // Fetch from AoC
        println!("Fetching input for day {} from Advent of Code...", day);
        let url = format!("{}/{}/day/{}/input", AOC_URL, self.year, day);
        let response = self.client.get(&url).send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch input: HTTP {}", response.status());
        }

        let input = response.text()?;

        // Cache it locally
        fs::create_dir_all("inputs")?;
        fs::write(&input_path, &input)?;
        println!("Cached input to {}", input_path);

        Ok(input)
    }

    pub fn submit_answer(&self, day: u32, part: u32, answer: &str) -> Result<String> {
        println!(
            "Submitting answer for day {}, part {}: {}",
            day, part, answer
        );

        let url = format!("{}/{}/day/{}/answer", AOC_URL, self.year, day);
        let params = [("level", part.to_string()), ("answer", answer.to_string())];

        let response = self.client.post(&url).form(&params).send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to submit answer: HTTP {}", response.status());
        }

        let body = response.text()?;

        // Parse the response to check if correct
        if body.contains("That's the right answer") {
            println!("✓ Correct!");
        } else if body.contains("That's not the right answer") {
            println!("✗ Incorrect");
        } else if body.contains("You gave an answer too recently") {
            println!("⏱ Rate limited - wait before submitting again");
        } else if body.contains("Did you already complete it") {
            println!("Already completed");
        }

        Ok(body)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin aoc <command> [args]");
        eprintln!("Commands:");
        eprintln!("  fetch <day>           - Fetch input for a day");
        eprintln!("  submit <day> <part> <answer> - Submit an answer");
        return Ok(());
    }

    let client = AocClient::new(2025)?;

    match args[1].as_str() {
        "fetch" => {
            let day: u32 = args.get(2).context("Missing day argument")?.parse()?;
            let input = client.get_input(day)?;
            println!("Input length: {} characters", input.len());
        }
        "submit" => {
            let day: u32 = args.get(2).context("Missing day")?.parse()?;
            let part: u32 = args.get(3).context("Missing part")?.parse()?;
            let answer = args.get(4).context("Missing answer")?;
            client.submit_answer(day, part, answer)?;
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }

    Ok(())
}
