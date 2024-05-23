use crate::Figlet;
use std::io;

#[derive(Debug, Clone)]
pub struct Config {
    pub types: Vec<String>,
    pub types_desc: Vec<String>,
    pub figlet_file: Option<String>,
}

impl Config {
    pub fn get_figlet(&self) -> Result<Figlet, io::Error> {
        match self.figlet_file {
            Some(ref figlet_file) => Figlet::from_file(figlet_file),
            None => Ok(Figlet::default()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            types: vec![
                "feat", 
                "fix", 
                "docs", 
                "style", 
                "refactor", 
                "perf", 
                "test", 
                "build", 
                "ci", 
                "chore",
                "revert",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            types_desc: vec![
                "✨ feat:\tA new feature",
                "🐛 fix:\tA bug fix",
                "📚 docs:\tDocumentation only changes",
                "💎 style:\tChanges that do not affect the meaning of the code (white-space, formatting, etc)",
                "📦 refactor:\tA code change that neither fixes a bug nor adds a feature",
                "🚀 perf:\tA code change that improves performance",
                "🚨 test:\tAdding missing tests or correcting existing tests",
                "🛠  build:\tChanges that affect the build system or external dependencies',",
                "🔫 ci:\tChanges to our CI configuration files and scripts'",
                "📎 chore:\tOther changes that don't modify src or test files",
                "🗑  revert:\tReverts a previous commit",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            figlet_file: None,
        }
    }
}
