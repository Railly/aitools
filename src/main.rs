mod cli;
mod code_analyzer;
mod config;
mod file_system;
mod output_formatter;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    let _config = config::load_config()?;
    let args = cli::parse_args();
    let file_structure = file_system::get_file_structure(&args.path.to_string_lossy()).await?;
    let selected_files = cli::interactive_file_selection(&args.path)?;

    let contents = file_system::read_selected_files(&selected_files).await?;
    let signatures = code_analyzer::extract_signatures(&contents);

    output_formatter::format_output_for_neovim(
        &file_structure,
        &selected_files,
        &signatures,
        &contents,
    )?;

    Ok(())
}
