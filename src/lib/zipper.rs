use error::{Error,Result};

use std::env;
use std::path::{Path,PathBuf};
use std::io::Write;
use std::process::Command;
use std::fs::{self, File,DirBuilder};
use uuid;

/// Struct used to create zip (using filesystem and zip command)
pub struct Zipper {
    args: Vec<String>,
    path: PathBuf,
}

impl Zipper {
    /// creates new zipper
    pub fn new(path: &str) -> Result<Zipper> {
        let uuid = uuid::Uuid::new_v4();
        let zipper = Zipper {
            args:vec!(),
            path: Path::new(path).join(uuid.to_simple_string())
        };

        let res = DirBuilder::new()
            .recursive(true)
            .create(zipper.path.join("META-INF"));

        if !res.is_ok() {
            Err(Error::Render("could not create temporary directory for generating epub"))
        } else {
            Ok(zipper)
        }
    }

    /// writes a content to a temporary file
    pub fn write(&mut self, file: &str, content: &[u8]) -> Result<()> {
        if let Ok(mut f) = File::create(self.path.join(file)) {
            if f.write_all(content).is_ok() {
                self.args.push(String::from(file));
                Ok(())
            } else {
                Err(Error::Render("could not write to temporary file while generating epub"))
            }
        } else {
            Err(Error::Render("could not create temporary file for generating epub"))
        }
    }

    /// generate an epub into given file name
    pub fn generate_epub(&mut self, file: &str) -> Result<String> {
        let dir = try!(env::current_dir().map_err(|_| Error::Render("could not get current directory")));
        try!(env::set_current_dir(&self.path).map_err(|_| Error::Render("could not change current directory")));

        let output = Command::new("zip")
            .arg("-X")
            .arg(file)
            .args(&self.args)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        try!(env::set_current_dir(dir).map_err(|_| Error::Render("could not change back to old directory")));
        try!(fs::copy(self.path.join(file), file).map_err(|_| Error::Render("could not copy epub file")));
        try!(fs::remove_dir_all(&self.path).map_err(|_| Error::Render("could not delete temporary directory")));
        String::from_utf8(output.stdout).map_err(|_| Error::Render("invalid utf-8 code in command output"))
    }
}
