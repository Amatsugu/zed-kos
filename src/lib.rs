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
		// 1. Already on PATH?
		if let Some(path) = worktree.which("kos-language-server")
		{
			return Ok(zed::Command {
				command: path,
				args: vec!["--stdio".into()],
				env: Default::default(),
			});
		}

		// 2. Already installed by us previously?
		if let Some(path) = &self.cached_binary_path
		{
			if std::fs::metadata(path).is_ok()
			{
				return Ok(zed::Command {
					command: path.clone(),
					args: vec!["--stdio".into()],
					env: Default::default(),
				});
			}
		}

		// 3. Install via npm into the extension's work dir
		zed::npm_install_package("kos-vscode", "latest")?;
		let path = zed::node_binary_path()?; // or locate the installed bin directly
		self.cached_binary_path = Some(path.clone());

		Ok(zed::Command {
			command: path,
			args: vec!["--stdio".into()],
			env: Default::default(),
		})
	}
}

zed::register_extension!(KerboscriptExtension);
