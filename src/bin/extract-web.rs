//! Web content extraction CLI
//!
//! Usage:
//!   extract-web https://example.com

use hanzo_extract::{Extractor, ExtractorConfig, WebExtractor};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.iter().any(|a| a == "--help" || a == "-h") {
        println!("extract-web - Extract clean text from web pages");
        println!();
        println!("USAGE:");
        println!("    extract-web <URL> [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -j, --json     Output as JSON");
        println!("    -h, --help     Print help");
        return;
    }

    let url = &args[1];
    let json_output = args.iter().any(|a| a == "--json" || a == "-j");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let extractor = WebExtractor::new(ExtractorConfig::default());

        match extractor.extract(url).await {
            Ok(result) => {
                if json_output {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "url": url,
                            "title": result.title,
                            "text": result.text,
                            "text_length": result.text_length,
                        }))
                        .unwrap()
                    );
                } else {
                    if let Some(title) = result.title {
                        println!("# {title}\n");
                    }
                    println!("{}", result.text);
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    });
}
