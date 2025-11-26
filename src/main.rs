use anyhow::Result;
use chrono::{DateTime, Utc};
use colored::*;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct GitHubStats {
    username: String,
    name: String,
    bio: Option<String>,
    company: Option<String>,
    location: Option<String>,
    blog: Option<String>,
    email: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
    total_stars: u32,
    total_forks: u32,
    languages: Vec<(String, u32)>,
}

fn main() -> Result<()> {
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN environment variable not set");
    let username = env::var("USER_NAME").expect("USER_NAME environment variable not set");

    println!();
    print_github_logo();

    let stats = fetch_github_stats(&token, &username)?;
    print_stats(&stats);

    println!();
    Ok(())
}

fn print_github_logo() {
    let logo = r#"
    ┌─────────────────────────────────────────────────┐
    │  ██████╗ ██╗████████╗██╗  ██╗██╗   ██╗██████╗   │
    │ ██╔════╝ ██║╚══██╔══╝██║  ██║██║   ██║██╔══██╗  │
    │ ██║  ███╗██║   ██║   ███████║██║   ██║██████╔╝  │
    │ ██║   ██║██║   ██║   ██╔══██║██║   ██║██╔══██╗  │
    │ ╚██████╔╝██║   ██║   ██║  ██║╚██████╔╝██████╔╝  │
    │  ╚═════╝ ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═════╝   │
    └─────────────────────────────────────────────────┘"#;

    println!("{}", logo.bright_blue());
}

fn print_stats(stats: &GitHubStats) {
    let left_padding = "    ";

    // User info section
    println!(
        "{}{} {}",
        left_padding,
        "Username:".bright_cyan(),
        stats.username.white().bold()
    );
    println!(
        "{}{} {}",
        left_padding,
        "Name:".bright_cyan(),
        stats.name.white()
    );

    if let Some(bio) = &stats.bio {
        println!("{}{} {}", left_padding, "Bio:".bright_cyan(), bio.white());
    }

    if let Some(company) = &stats.company {
        println!(
            "{}{} {}",
            left_padding,
            "Company:".bright_cyan(),
            company.white()
        );
    }

    if let Some(location) = &stats.location {
        println!(
            "{}{} {}",
            left_padding,
            "Location:".bright_cyan(),
            location.white()
        );
    }

    if let Some(blog) = &stats.blog {
        if !blog.is_empty() {
            println!("{}{} {}", left_padding, "Blog:".bright_cyan(), blog.white());
        }
    }

    println!();

    // Account stats
    println!(
        "{}{} {}",
        left_padding,
        "Account Created:".bright_green(),
        stats.created_at.format("%Y-%m-%d").to_string().white()
    );

    let account_age = Utc::now() - stats.created_at;
    let days = account_age.num_days();
    let years = days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30;
    let days_left = remaining_days % 30;

    println!(
        "{}{} {} years, {} months, {} days",
        left_padding,
        "Account Age:".bright_green(),
        years.to_string().white().bold(),
        months.to_string().white().bold(),
        days_left.to_string().white().bold()
    );

    println!();

    // Repository stats
    println!(
        "{}{} {}",
        left_padding,
        "Public Repos:".bright_yellow(),
        format_number(stats.public_repos).white().bold()
    );
    println!(
        "{}{} {}",
        left_padding,
        "Public Gists:".bright_yellow(),
        format_number(stats.public_gists).white().bold()
    );
    println!(
        "{}{} {} ⭐",
        left_padding,
        "Total Stars:".bright_yellow(),
        format_number(stats.total_stars).white().bold()
    );
    println!(
        "{}{} {}",
        left_padding,
        "Total Forks:".bright_yellow(),
        format_number(stats.total_forks).white().bold()
    );

    println!();

    // Social stats
    println!(
        "{}{} {}",
        left_padding,
        "Followers:".bright_magenta(),
        format_number(stats.followers).white().bold()
    );
    println!(
        "{}{} {}",
        left_padding,
        "Following:".bright_magenta(),
        format_number(stats.following).white().bold()
    );

    println!();

    // Top languages
    if !stats.languages.is_empty() {
        println!("{}{}", left_padding, "Top Languages:".bright_red());
        let mut lang_display = Vec::new();
        let total_bytes: u32 = stats.languages.iter().map(|(_, bytes)| *bytes).sum();

        for (i, (lang, bytes)) in stats.languages.iter().enumerate().take(5) {
            let percentage = if total_bytes > 0 {
                (*bytes as f64 / total_bytes as f64 * 100.0) as u32
            } else {
                0
            };

            let color = match i {
                0 => "bright_red",
                1 => "bright_green",
                2 => "bright_yellow",
                3 => "bright_blue",
                _ => "bright_magenta",
            };

            lang_display.push(format!(
                "{} ({}%)",
                match color {
                    "bright_red" => lang.bright_red(),
                    "bright_green" => lang.bright_green(),
                    "bright_yellow" => lang.bright_yellow(),
                    "bright_blue" => lang.bright_blue(),
                    _ => lang.bright_magenta(),
                },
                percentage.to_string().white().bold()
            ));
        }

        for (i, lang_str) in lang_display.iter().enumerate() {
            println!("{}      {}: {}", left_padding, (i + 1), lang_str);
        }
    }

    println!();

    // Footer
    let footer = format!("Last updated: {}", Utc::now().format("%Y-%m-%d %H:%M UTC"));
    println!("{}{}", left_padding, footer.bright_black());
}

fn fetch_github_stats(token: &str, username: &str) -> Result<GitHubStats> {
    // Get user info
    let user_url = format!("https://api.github.com/users/{}", username);
    let user_data: Value = ureq::get(&user_url)
        .header("Authorization", &format!("token {}", token))
        .header("User-Agent", "fetchme-rust")
        .call()?
        .body_mut()
        .read_json()?;

    // Get repositories for star/fork count and languages
    let repos_url = format!(
        "https://api.github.com/users/{}/repos?per_page=100&type=owner",
        username
    );
    let repos_data: Value = ureq::get(&repos_url)
        .header("Authorization", &format!("token {}", token))
        .header("User-Agent", "fetchme-rust")
        .call()?
        .body_mut()
        .read_json()?;

    let mut total_stars = 0;
    let mut total_forks = 0;
    let mut language_stats: HashMap<String, u32> = HashMap::new();

    if let Some(repos) = repos_data.as_array() {
        for repo in repos {
            if let Some(stars) = repo["stargazers_count"].as_u64() {
                total_stars += stars as u32;
            }
            if let Some(forks) = repo["forks_count"].as_u64() {
                total_forks += forks as u32;
            }

            // Get language data for each repo
            if let Some(languages_url) = repo["languages_url"].as_str() {
                if let Ok(mut lang_response) = ureq::get(languages_url)
                    .header("Authorization", &format!("token {}", token))
                    .header("User-Agent", "fetchme-rust")
                    .call()
                {
                    if let Ok(lang_data) = lang_response.body_mut().read_json::<Value>() {
                        if let Some(lang_obj) = lang_data.as_object() {
                            for (lang, bytes) in lang_obj {
                                if let Some(byte_count) = bytes.as_u64() {
                                    *language_stats.entry(lang.clone()).or_insert(0) +=
                                        byte_count as u32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort languages by usage
    let mut languages: Vec<(String, u32)> = language_stats.into_iter().collect();
    languages.sort_by(|a, b| b.1.cmp(&a.1));

    let stats = GitHubStats {
        username: user_data["login"].as_str().unwrap_or("").to_string(),
        name: user_data["name"].as_str().unwrap_or("").to_string(),
        bio: user_data["bio"].as_str().map(|s| s.to_string()),
        company: user_data["company"].as_str().map(|s| s.to_string()),
        location: user_data["location"].as_str().map(|s| s.to_string()),
        blog: user_data["blog"].as_str().map(|s| s.to_string()),
        email: user_data["email"].as_str().map(|s| s.to_string()),
        created_at: user_data["created_at"]
            .as_str()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now()),
        updated_at: user_data["updated_at"]
            .as_str()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now()),
        public_repos: user_data["public_repos"].as_u64().unwrap_or(0) as u32,
        public_gists: user_data["public_gists"].as_u64().unwrap_or(0) as u32,
        followers: user_data["followers"].as_u64().unwrap_or(0) as u32,
        following: user_data["following"].as_u64().unwrap_or(0) as u32,
        total_stars,
        total_forks,
        languages,
    };

    Ok(stats)
}

fn format_number(num: u32) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f32 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f32 / 1_000.0)
    } else {
        num.to_string()
    }
}
