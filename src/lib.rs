use zed_extension_api::{self as zed, LanguageServerId, Result};

struct KerboscriptExtension
{
	cached_binary_path: Option<String>,
}

impl zed::Extension for KerboscriptExtension
{
	fn new() -> Self
	{
		Self {
			cached_binary_path: None,
		}
	}

	fn language_server_command(&mut self, _id: &LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command>
	{
		let node = zed::node_binary_path()?;
		Ok(zed::Command {
			command: node,
			args: vec![
				"./vendor/server.js".into(), // adjust to actual entry point
				"--stdio".into(),
			],
			env: Default::default(),
		})
	}
}

zed::register_extension!(KerboscriptExtension);
