use crate::defs::{
    ApplyPatchArgs, FileListArgs, FileReadArgs, FileWriteArgs, FsReadArgs, FsWriteArgs,
};
use crate::Toolset;

impl Toolset {
    pub(super) fn call_fs_tool(&self, name: &str, arguments_json: &str) -> anyhow::Result<String> {
        match name {
            "fs_read" => {
                let args: FsReadArgs = super::parse_args(arguments_json, "fs_read")?;
                self.fs_read(&args.path)
            }
            "file_read" => {
                let args: FileReadArgs = super::parse_args(arguments_json, "file_read")?;
                self.fs_read(&args.path)
            }
            "fs_write" => {
                let args: FsWriteArgs = super::parse_args(arguments_json, "fs_write")?;
                self.fs_write(&args.path, &args.content)
            }
            "file_write" => {
                let args: FileWriteArgs = super::parse_args(arguments_json, "file_write")?;
                self.fs_write(&args.path, &args.content)
            }
            "file_list" => {
                let args: FileListArgs = super::parse_args(arguments_json, "file_list")?;
                self.file_list(&args.path)
            }
            "apply_patch" => {
                let args: ApplyPatchArgs = super::parse_args(arguments_json, "apply_patch")?;
                self.apply_patch(&args.patch)
            }
            _ => unreachable!("unexpected fs tool: {name}"),
        }
    }
}
