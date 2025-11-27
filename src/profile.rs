use anyhow::Result;
use chrono::{DateTime, Utc};
use image::ImageReader;
use owo_colors::OwoColorize;
use serde_json::Value;
use std::{collections::HashMap, fmt, io::Cursor, num::NonZeroU32};

#[derive(Debug, Default)]
pub struct Options {
    pub token: String,
    pub username: String,
    pub birth: Option<DateTime<Utc>>,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
    pub os: Option<String>,
}

#[derive(Debug)]
pub struct Profile {
    pub avatar: String,
    pub username: String,
    pub os: Option<String>,
    pub name: String,
    pub bio: Option<String>,
    pub birth: Option<DateTime<Utc>>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub linkedin: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub public_repos: u32,
    pub public_gists: u32,
    pub total_stars: u32,
    pub total_forks: u32,
    pub followers: u32,
    pub following: u32,
    pub languages: Vec<(String, u32)>,
}

impl Profile {
    pub fn fetch(options: Options) -> Result<Self> {
        let Options {
            token,
            username,
            birth,
            phone,
            linkedin,
            os,
        } = options;

        // Get user info
        let user_url = format!("https://api.github.com/users/{}", username);
        let user_data: Value = ureq::get(&user_url)
            .header("Authorization", &format!("token {}", token))
            .header("User-Agent", "fetchme-rust")
            .call()?
            .body_mut()
            .read_json()?;

        let avatar_url = user_data["avatar_url"].as_str().unwrap_or("");
        let mut avatar_bytes = ureq::get(avatar_url)
            .header("User-Agent", "fetchme-rust")
            .call()?
            .body_mut()
            .read_to_vec()?;
        let avatar = ImageReader::new(Cursor::new(&mut avatar_bytes))
            .with_guessed_format()?
            .decode()?;
        let avatar = artem::convert(
            avatar,
            &artem::ConfigBuilder::new()
                .target_size(NonZeroU32::new(50).unwrap())
                .build(),
        );

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
        let mut languages = HashMap::new();

        if let Some(repos) = repos_data.as_array() {
            for repo in repos {
                if let Some(stars) = repo["stargazers_count"].as_u64() {
                    total_stars += stars as u32;
                }
                if let Some(forks) = repo["forks_count"].as_u64() {
                    total_forks += forks as u32;
                }

                if repo["fork"].as_bool().is_some_and(|b| b) {
                    continue;
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
                                        *languages.entry(lang.clone()).or_insert(0) +=
                                            byte_count as u32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut languages = languages.into_iter().collect::<Vec<_>>();
        languages.sort_by_key(|el| el.1);
        languages.reverse();

        let user = Self {
            avatar,
            username: user_data["login"].as_str().unwrap_or("").to_string(),
            os,
            name: user_data["name"].as_str().unwrap_or("").to_string(),
            bio: user_data["bio"].as_str().map(|s| s.to_string()),
            birth,
            email: user_data["email"].as_str().map(|s| s.to_string()),
            phone,
            location: user_data["location"].as_str().map(|s| s.to_string()),
            company: user_data["company"].as_str().map(|s| s.to_string()),
            blog: user_data["blog"].as_str().map(|s| s.to_string()),
            linkedin,
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
            total_stars,
            total_forks,
            followers: user_data["followers"].as_u64().unwrap_or(0) as u32,
            following: user_data["following"].as_u64().unwrap_or(0) as u32,
            languages,
        };

        Ok(user)
    }

    /// Returns list of lines
    pub fn render(&self, mut w: impl fmt::Write) -> fmt::Result {
        let header = format!("{}@{}", self.username.bold(), "github".bright_blue());
        let separator = "─".repeat(header.chars().count().max(20));

        let uptime = self.birth.map(|birth| {
            let age = Utc::now() - birth;
            let num_days = age.num_days();
            let years = num_days / 365;
            let days = num_days % 365;
            format!(
                "{}: {} years, {} days",
                "Uptime".bright_green(),
                years.bold(),
                days.bold()
            )
        });

        // Build all possible lines in order
        let mut lines = vec![
            Some(header),
            Some(separator.bright_black().to_string()),
            self.os
                .as_ref()
                .map(|os| format!("{}: {}", "OS".bright_cyan(), os)),
            Some(format!("{}: {}", "Host".bright_cyan(), self.name)),
            self.bio
                .as_ref()
                .map(|bio| format!("{}: {}", "Commit Message".bright_cyan(), bio)),
            uptime,
            self.email
                .as_ref()
                .map(|email| format!("{}: {}", "Email".bright_cyan(), email)),
            self.phone
                .as_ref()
                .map(|phone| format!("{}: {}", "TTY".bright_cyan(), phone)),
            self.location
                .as_ref()
                .map(|location| format!("{}: {}", "Locale".bright_green(), location)),
            self.company
                .as_ref()
                .map(|company| format!("{}: {}", "Company".bright_green(), company)),
            self.blog
                .as_ref()
                .filter(|blog| !blog.is_empty())
                .map(|blog| format!("{}: {}", "Upstream".bright_green(), blog)),
            self.linkedin
                .as_ref()
                .map(|linkedin| format!("{}: {}", "LinkedIn".bright_green(), linkedin)),
            Some(format!(
                "{}: {}",
                "Public Repos".bright_yellow(),
                format_number(self.public_repos).bold()
            )),
            Some(format!(
                "{}: {}",
                "Public Gists".bright_yellow(),
                format_number(self.public_gists).bold()
            )),
            Some(format!(
                "{}: {}",
                "Total Stars".bright_yellow(),
                format_number(self.total_stars).bold()
            )),
            Some(format!(
                "{}: {}",
                "Total Forks".bright_yellow(),
                format_number(self.total_forks).bold()
            )),
            Some(format!(
                "{}: {}",
                "Followers".bright_magenta(),
                format_number(self.followers).bold()
            )),
            Some(format!(
                "{}: {}",
                "Following".bright_magenta(),
                format_number(self.following).bold()
            )),
        ];

        // Add top languages section if we have any
        if !self.languages.is_empty() {
            lines.push(Some("".to_string())); // Empty line separator
            lines.push(Some(format!("{}", "Top Languages".bright_red())));

            let total_bytes: u32 = self.languages.iter().map(|(_, bytes)| *bytes).sum();
            for (i, (lang, bytes)) in self.languages.iter().enumerate().take(5) {
                let percentage = if total_bytes > 0 {
                    (*bytes as f64 / total_bytes as f64 * 100.0) as u32
                } else {
                    0
                };

                let colored_lang = match i {
                    0 => lang.bright_red().into_styled(),
                    1 => lang.bright_green().into_styled(),
                    2 => lang.bright_yellow().into_styled(),
                    3 => lang.bright_blue().into_styled(),
                    _ => lang.bright_magenta().into_styled(),
                };

                lines.push(Some(format!(
                    "  {}: {}%",
                    colored_lang,
                    percentage.to_string().bold()
                )));
            }
        }

        writeln!(w, "{} fetchme", "$".bright_green())?;

        let stats_lines = lines.into_iter().flatten().collect::<Vec<String>>();
        let avatar_lines = self.avatar.lines().collect::<Vec<&str>>();

        // Print side by side
        let max_lines = avatar_lines.len().max(stats_lines.len());
        for i in 0..max_lines {
            let avatar_line = avatar_lines.get(i).unwrap_or(&"").to_string();
            let stats_line = stats_lines.get(i).unwrap_or(&String::new()).clone();

            // Pad logo to consistent width (around 50 chars)
            let padded_avatar = format!("{:<50}", avatar_line);
            writeln!(w, "{}    {}", padded_avatar, stats_line)?;
        }

        Ok(())
    }
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
