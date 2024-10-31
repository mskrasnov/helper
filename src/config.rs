use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    /// Если установлен как `true`, в строках документации будет использовано
    /// 4 пробела вместо одного символа табуляции.
    pub use_tabs: bool,
}

impl Config {
    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let content = fs::read_to_string(&pth)?;
        let data = toml::from_str(&content)?;

        Ok(data)
    }

    pub fn write<P: AsRef<Path>>(&self, pth: P) -> Result<()> {
        let content = toml::to_string_pretty(&self)?;
        fs::write(
            &pth,
            format!(
                "# Главный конфигурационный файл Helper\n\
             # (C) 2024 Михаил Краснов <michail383krasnov@mail.ru>\n\n{}",
                content,
            ),
        )?;

        Ok(())
    }
}
