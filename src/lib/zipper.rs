// Copyright (C) 2016-2022 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use crate::error::{Error, Result};

use std::fs::{self, DirBuilder, File};
use std::io;
use std::io::Write;
use std::ops::Drop;
use std::path::{Path, PathBuf};
use std::process::Command;
use rust_i18n::t;

/// Struct used to create zip (using filesystem and zip command)
pub struct Zipper {
    args: Vec<String>,
    path: PathBuf,
}

impl Zipper {
    /// Creates new zipper
    ///
    /// # Arguments
    /// * `path`: the path to a temporary directory
    /// (zipper will create a random dir in it and clean it later)
    pub fn new(path: &str) -> Result<Zipper> {
        let uuid = uuid::Uuid::new_v4();
        let zipper_path = Path::new(path).join(uuid.as_simple().to_string());

        DirBuilder::new()
            .recursive(true)
            .create(&zipper_path)
            .map_err(|_| {
                Error::zipper(t!(
                    "zipper.tmp_dir",
                    path = path
                ))
            })?;

        Ok(Zipper {
            args: vec![],
            path: zipper_path,
        })
    }

    /// writes a content to a temporary file
    pub fn write<P: AsRef<Path>>(&mut self, path: P, content: &[u8], add_args: bool) -> Result<()> {
        let path = path.as_ref();
        let file = format!("{}", path.display());
        if path.starts_with("..") || path.is_absolute() {
            return Err(Error::zipper(t!("zipper.verboten",
                file = file
            )));
        }
        let dest_file = self.path.join(path);
        let dest_dir = dest_file.parent().unwrap();
        if fs::metadata(dest_dir).is_err() {
            // dir does not exist, create it
            DirBuilder::new()
                .recursive(true)
                .create(dest_dir)
                .map_err(|_| {
                    Error::zipper(t!(
                        "zipper.tpm_dir",
                        path = dest_dir.display()
                    ))
                })?;
        }

        if let Ok(mut f) = File::create(&dest_file) {
            if f.write_all(content).is_ok() {
                if add_args {
                    self.args.push(file);
                }
                Ok(())
            } else {
                Err(Error::zipper(t!(
                    "zipper.write_error",
                    file = file
                )))
            }
        } else {
            Err(Error::zipper(t!(
                "zipper.create_error",
                file = file
            )))
        }
    }

    /// run command and copy content of file output (supposed to result from the command) to current dir
    pub fn run_command(
        &mut self,
        mut command: Command,
        command_name: &str,
        in_file: &str,
        out: &mut dyn Write,
    ) -> Result<String> {
        let res_output = command.output().map_err(|e| {
            debug!(
                "{}",
                t!("zipper.command_output",
                    name = command_name,
                    error = e
                )
            );
            Error::zipper(t!(
                "zipper.command_error",
                name = command_name
            ))
        });
        let output = res_output?;
        if output.status.success() {
            let mut file = File::open(self.path.join(in_file)).map_err(|_| {
                debug!(
                    "{}",
                    t!("zipper.command_result_error",
                        command = command_name,
                        output = String::from_utf8_lossy(&output.stderr)
                    )
                );
                Error::zipper(t!(
                    "zipper.command_result_err",
                    command = command_name
                ))
            })?;
            io::copy(&mut file, out).map_err(|_| {
                Error::zipper(t!("zipper.copy_error", file = in_file))
            })?;

            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            debug!(
                "{}",
                format!(
                    "{cmd}: {output}",
                    cmd = t!("zipper.command_no_success", command = command_name),
                    output = String::from_utf8_lossy(&output.stderr)
                )
            );
            Err(Error::zipper(t!(
                "zipper.command_no_success",
                command = command_name
            )))
        }
    }

    /// zip all files in zipper's tmp dir to a given file name and write to odt file
    #[cfg(feature = "odt")]
    pub fn generate_odt(&mut self, command_name: &str, odt_file: &mut dyn Write) -> Result<String> {
        let mut command = Command::new(command_name);
        command.current_dir(&self.path);
        command.arg("-r");
        command.arg("result.odt");
        command.arg(".");
        self.run_command(command, command_name, "result.odt", odt_file)
    }

    /// generate a pdf file into given file name
    pub fn generate_pdf(
        &mut self,
        command_name: &str,
        tex_file: &str,
        pdf_file: &mut dyn Write,
    ) -> Result<String> {
        // first pass
        let mut command = Command::new(command_name);
        command.current_dir(&self.path).arg(tex_file);
        let _ = command.output();

        // second pass
        let _ = command.output();

        // third pass
        // let mut command = Command::new(command_name);
        // command.current_dir(&self.path);
        // command.arg(tex_file);
        self.run_command(command, command_name, "result.pdf", pdf_file)
    }
}

impl Drop for Zipper {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.path) {
            println!(
                "Error in zipper: could not delete temporary directory {}, error: {}",
                self.path.to_string_lossy(),
                err
            );
        }
    }
}
