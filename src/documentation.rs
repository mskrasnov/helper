use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml;

/// Для удобства переименовал `String` в `CategoryName`, чтобы было
/// понятно, зачем в качестве ключа используется `String`.
pub type CategoryName = String;

/// Содержит информацию о тех файлах, которые необходимо добавить в меню программы
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Documentation {
    /// Список доступных категорий документации в упорядоченном виде
    pub categories: Vec<Category>,

    /// Список файлов документации, доступной для чтения
    pub docs: HashMap<CategoryName, Vec<Doc>>,
}

/// Список доступных категорий документации в упорядоченном виде
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Category {
    /// Идентификатор, используемый в `Documentation.docs` в качестве ключа
    pub id: String,

    /// Имя, которое будет отображено в главном меню программы
    pub name: String,
}

/// Описывает каждый файл документации, доступный для чтения пользователем
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Doc {
    /// Идентификатор, используемый в качестве имени файла для чтения
    pub id: String,

    /// Имя, которое будет отображено в главном меню программы
    pub name: String,
}

impl Documentation {
    /// Читает конфигурационный файл и выполняет его десериализацию
    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let content = fs::read_to_string(&pth)?;
        let data = toml::from_str(&content)?;

        Ok(data)
    }

    /// Сериализует данные структуры `Documentation` и пишет их в указанный файл
    pub fn write<P: AsRef<Path>>(&self, pth: P) -> Result<()> {
        let contents = toml::to_string_pretty(&self)?;
        fs::write(
            &pth,
            format!(
                "# Конфигурационный файл со списком доступной для чтения документации\n\
                 # (C) 2024 Михаил Краснов <michail383krasnov@mail.ru>\n\n{}",
                contents
            ),
        )?;

        Ok(())
    }
}

impl Default for Documentation {
    fn default() -> Self {
        let mut docs = HashMap::new();
        docs.insert(
            "intro".to_string(),
            vec![
                Doc {
                    id: "about".to_string(),
                    name: "Что такое Linux?".to_string(),
                },
                Doc {
                    id: "free-software".to_string(),
                    name: "Свободное программное обеспечение".to_string(),
                },
                Doc {
                    id: "helper".to_string(),
                    name: "Пару слов про Helper".to_string(),
                },
            ],
        );
        docs.insert(
            "systems".to_string(),
            vec![
                Doc {
                    id: "unix".to_string(),
                    name: "Многообразие Unix-систем".to_string(),
                },
                Doc {
                    id: "linux".to_string(),
                    name: "Зоопарк Linux-систем".to_string(),
                },
                Doc {
                    id: "adw-disadw".to_string(),
                    name: "Достоинства и недостатки Linux".to_string(),
                },
            ],
        );

        Self {
            categories: vec![
                Category {
                    id: "intro".to_string(),
                    name: "Введение".to_string(),
                },
                Category {
                    id: "systems".to_string(),
                    name: "Операционные системы".to_string(),
                },
                Category {
                    id: "package_managers".to_string(),
                    name: "Пакетные менеджеры".to_string(),
                },
                Category {
                    id: "quick_fixes".to_string(),
                    name: "Быстрое решение проблем".to_string(),
                },
            ],
            docs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_def_res_conf() {
        let doc = Documentation::default();
        doc.write("./test/documentation.toml").unwrap();
    }

    #[test]
    fn read_def_res_conf() {
        write_def_res_conf(); // сначала пишем дефолтные данные в тестируемый конфиг ресурсов
        let doc = Documentation::read("./test/documentation.toml").unwrap();
        let def_doc = Documentation::default();

        assert_eq!(doc, def_doc);
    }
}
