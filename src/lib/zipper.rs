use error::{Error,Result};

use std::env;
use std::str;
use std::path::{Path,PathBuf};
use std::io::Write;
use std::process::Command;
use std::fs::{self, File,DirBuilder};
use uuid;
use std::ops::Drop;

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
            Err(Error::Render("could not create temporary directory "))
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
                Err(Error::Render("could not write to temporary file"))
            }
        } else {
            Err(Error::Render("could not create temporary file"))
        }
    }

    /// run command and copy file name to current dir
    pub fn run_command(&mut self, mut command: Command, file: &str) -> Result<String> {
        let dir = try!(env::current_dir().map_err(|_| Error::Render("could not get current directory")));
        try!(env::set_current_dir(&self.path).map_err(|_| Error::Render("could not change current directory")));

        let output = command.args(&self.args)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        try!(env::set_current_dir(dir).map_err(|_| Error::Render("could not change back to old directory")));
        try!(fs::copy(self.path.join(file), file).map_err(|_| {
            println!("{}", str::from_utf8(&output.stdout).unwrap());
            Error::Render("could not copy file")
        }));
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// generate a pdf file into given file name
    pub fn generate_pdf(&mut self, tex_file: &str, pdf_file: &str) -> Result<String> {
        let mut command = Command::new("pdflatex");
        command.arg(tex_file);
        self.run_command(command, pdf_file)
    }
    
    /// generate an epub into given file name
    pub fn generate_epub(&mut self, file: &str) -> Result<String> {
        let mut command = Command::new("zip");
        command.arg("-X");
        command.arg(file);
        self.run_command(command, file)
    }
}

impl Drop for Zipper {
    fn drop(&mut self) {
        if !fs::remove_dir_all(&self.path).is_ok() {
            println!("Error in zipper: could not delete temporary directory");
        }
    }
}
