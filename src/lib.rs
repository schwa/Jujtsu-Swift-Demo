use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::sync::Arc;

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct Repo {
    path: PathBuf,
}

#[uniffi::export]
fn repo_from_path(path: String) -> Repo {
    Repo::new(PathBuf::from(path))
}

#[uniffi::export]
impl Repo {
    #[uniffi::constructor]
    pub fn new_from_string(path: &str) -> Self {
        let path = PathBuf::from(path);
        Repo::new(path)
    }
}

#[uniffi::export]
impl Repo {
    #[uniffi::method(name = "log")]
    pub fn log_api(&self) -> Vec<Arc<Change>> {
        // self.log().unwrap()
        let logs = self.log().unwrap();
        logs.into_iter().map(Arc::new).collect()
    }
}

impl Repo {
    pub fn new(path: PathBuf) -> Self {
        Repo { path }
    }

    pub fn log(&self) -> Result<Vec<Change>> {
        // let template =
        // r#"
        // surround('[', ']', self.description())
        // "#;
        // let template = r#"
        //     '{'
        //     ++ change_id++ '|' ++ commit_id ++ '|' ++ divergent ++ '|' ++ immutable ++ '}' ++ "\n"
        // "#;
        let template = r#""\n"
        ++ "change_id.shortest: " ++ change_id.shortest() ++ "\n"
        ++ "change_id: " ++ change_id ++ "\n"
        ++ "commit_id: " ++ commit_id ++ "\n"
        ++ "divergent: " ++ divergent ++ "\n"
        ++ "immutable: " ++ immutable ++ "\n"
        ++ "author.email: " ++ author.email() ++ "\n"
        ++ "author.name: " ++ author.name() ++ "\n"
        ++ "description: " ++ description ++ "\n"
        ++ "\0"
        "#;

        let output = Command::new("/opt/homebrew/bin/jj")
            .arg("--repository")
            .arg(&self.path)
            .arg("--ignore-working-copy")
            .arg("--color")
            .arg("never")
            .arg("--quiet")
            .arg("--no-pager")
            .arg("log")
            .arg("--no-graph")
            .arg("--revisions")
            .arg("::")
            .arg("--template")
            .arg(template)
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            let output = str::from_utf8(&output.stdout)?
                .split('\0')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .map(|s| Change::from_log_str(&s))
                .collect::<Vec<Change>>();
            Ok(output)
        } else {
            Err(anyhow::anyhow!("Error"))
        }
    }
}

#[derive(Debug, uniffi::Object)]
pub struct Change {
    pub change_id: ChangeId,
    pub commit_id: String,
    pub divergent: bool,
    pub immutable: bool,
    pub author: Author,
    pub description: String,
}

#[uniffi::export]
impl Change {
    pub fn change_id(&self) -> ChangeId {
        self.change_id.clone()
    }
    pub fn commit_id(&self) -> String {
        self.commit_id.clone()
    }
    pub fn divergent(&self) -> bool {
        self.divergent
    }
    pub fn immutable(&self) -> bool {
        self.immutable
    }
    pub fn author(&self) -> Author {
        self.author.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl Change {
    fn from_log_str(slice: &str) -> Self {
        let mut shortest_change_id = String::new();
        let mut change_id = String::new();
        let mut commit_id = String::new();
        let mut divergent = false;
        let mut immutable = false;
        let mut author_email = String::new();
        let mut author_name = String::new();
        let mut description = String::new();

        for line in slice.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (key, value) = line.split_once(":").unwrap();
            let key = key.trim();
            let value = value.trim();
            match key {
                "change_id.shortest" => shortest_change_id = value.to_string(),
                "change_id" => change_id = value.to_string(),
                "commit_id" => commit_id = value.to_string(),
                "divergent" => divergent = true_or_false(value),
                "immutable" => immutable = true_or_false(value),
                "author.email" => author_email = value.to_string(),
                "author.name" => author_name = value.to_string(),
                "description" => description = value.to_string(),
                _ => {}
            }
        }

        Change {
            change_id: ChangeId {
                id: change_id,
                shortest_id: shortest_change_id,
            },
            commit_id,
            divergent,
            immutable,

            author: Author {
                email: author_email,
                name: author_name,
            },
            description,
        }
    }
}

#[derive(Debug, Clone, uniffi::Object)]
pub struct ChangeId {
    pub id: String,
    pub shortest_id: String,
}

#[uniffi::export]
impl ChangeId {
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn shortest_id(&self) -> String {
        self.shortest_id.clone()
    }
}

#[derive(Debug, Clone, uniffi::Object)]
pub struct Author {
    pub email: String,
    pub name: String,
}

#[uniffi::export]
impl Author {
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn true_or_false(value: &str) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => false,
    }
}
