use std::env;

#[derive(Debug, Clone, Copy)]
pub enum Logo {
    Github,
    Linux,
    Rust,
    Cat,
    Octocat,
    Simple,
}

impl Logo {
    pub fn from_env() -> Self {
        match env::var("LOGO")
            .as_deref()
            .unwrap_or("github")
            .to_lowercase()
            .as_str()
        {
            "github" => Self::Github,
            "linux" => Self::Linux,
            "rust" => Self::Rust,
            "cat" => Self::Cat,
            "octocat" => Self::Octocat,
            "simple" => Self::Simple,
            _ => Self::Github, // Default fallback
        }
    }

    pub fn render(self) -> Vec<&'static str> {
        match self {
            Self::Github => r#"    ┌─────────────────────────────────────────────────┐
    │  ██████╗ ██╗████████╗██╗  ██╗██╗   ██╗██████╗   │
    │ ██╔════╝ ██║╚══██╔══╝██║  ██║██║   ██║██╔══██╗  │
    │ ██║  ███╗██║   ██║   ███████║██║   ██║██████╔╝  │
    │ ██║   ██║██║   ██║   ██╔══██║██║   ██║██╔══██╗  │
    │ ╚██████╔╝██║   ██║   ██║  ██║╚██████╔╝██████╔╝  │
    │  ╚═════╝ ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═════╝   │
    └─────────────────────────────────────────────────┘"#
                .lines()
                .collect(),
            Self::Linux => r#"                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooo+/::
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/"#
                .lines()
                .collect(),
            Self::Rust => r#"             _~^~^~_
         \) /  o o  \ (/
           '_   -   _'
           / '-----' \
          (  .~"""~.  )
         ( /         \ )
        ( (  ,~~|~~.  ) )
        ( (  |o(_)o|  ) )
         ( _',~"""~,'_ )
          (_/       \_)
           '-.     .-'
              '"""'"#
                .lines()
                .collect(),
            Self::Cat => r#"     /\_/\
    ( o.o )
     > ^ <

   ┌─┬─┬─┐
   │ │ │ │
   └─┴─┴─┘"#
                .lines()
                .collect(),
            Self::Octocat => r#"               MMM.           .MMM
               MMMMMMMMMMMMMMMMMMM
               MMMMMMMMMMMMMMMMMMM      _____
              MMMMMMMMMMMMMMMMMMMMM    |     |
             MMMMMMMMMMMMMMMMMMMMMMM   |_____|
            MMMMMMMMMMMMMMMMMMMMMMMM    O   O
           MMMMMMMMMMMMMMMMMMMMMMMMMM     \_/
          MMMMMMMMMMMMMMMMMMMMMMMMMMM
         MMMMMMMMMMMMMMMMMMMMMMMMMMMM
        MMMMMMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMMMM
      MMMMMMMMMMMMMMMMMMMMMMMMMMMM
     MMMMMMMMMMMMMMMMMMMMMMMMMMMM
    MMMMMMMMMMMMMMMMMMMMMMMMMMMM"#
                .lines()
                .collect(),
            Self::Simple => r#"    [  GitHub Stats  ]

         ^     ^
        ( o   o )
         \  ~  /
          '---'"#
                .lines()
                .collect(),
        }
    }
}
