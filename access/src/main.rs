use app_store_access::{client::SuggestionClient, country::Country, language::Language};
use app_store_access_apple::model::lookup::LookupResult;
use app_store_access_google::request::params::search::PriceFilter;
use cli_helpers::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

const LOOKUP_PAGE_SIZE: usize = 50;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("Apple client error")]
    AppleClient(#[from] app_store_access_apple::client::Error),
    #[error("Apple client suggestion error")]
    AppleSuggestClient(#[from] app_store_access_apple::client::suggest::Error),
    #[error("Google client error")]
    GoogleClient(#[from] app_store_access_google::client::Error),
    #[error("Apple client suggestion error")]
    GoogleSuggestClient(#[from] app_store_access_google::client::suggest::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("JSON file error")]
    JsonFile(PathBuf, serde_json::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::Api {
            apple_archive,
            google_archive,
            command,
        } => {
            let apple_client = app_store_access_apple::client::Client::new(Some(&apple_archive));

            let google_client = app_store_access_google::client::Client::new(
                Some(&google_archive),
                Default::default(),
            )?;

            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(std::io::stdout());

            match command {
                ApiCommand::Search {
                    query,
                    country,
                    lang,
                    full,
                    delay,
                } => {
                    let page = apple_client.search(&query, country, lang).await?;

                    let ids = page
                        .bubbles
                        .bubbles()
                        .map(|bubbles| {
                            bubbles
                                .results
                                .iter()
                                .map(|result| result.id)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    if full {
                        log::info!("Downloading full information for {} Apple apps", ids.len());

                        for id in ids {
                            match apple_client.app(id, country).await? {
                                Some(page) => {
                                    let app = page
                                        .store_platform_data
                                        .product_dv
                                        .results
                                        .get(&id)
                                        .unwrap();

                                    writer.write_record([
                                        "apple".to_string(),
                                        id.to_string(),
                                        app.common
                                            .bundle_id
                                            .as_ref()
                                            .map(|id| id.to_string())
                                            .unwrap_or_default(),
                                        app.artist_id.to_string(),
                                        app.common.name.to_string(),
                                        app.common.artist_name.to_string(),
                                    ])?;

                                    writer.flush()?;
                                }
                                None => {
                                    log::warn!("App not found: {}", id);
                                }
                            }

                            tokio::time::sleep(Duration::from_millis(delay)).await;
                        }
                    } else {
                        log::info!("Downloading {} Apple search results", ids.len());

                        for id_chunk in ids.chunks(LOOKUP_PAGE_SIZE) {
                            let result = apple_client.lookup_ids(id_chunk, country, lang).await?;

                            for result in result.results {
                                match &result {
                                    LookupResult::Software(software) => {
                                        writer.write_record([
                                            "apple".to_string(),
                                            software.track_id.to_string(),
                                            software.bundle_id.to_string(),
                                            software.artist_id.to_string(),
                                            software.track_name.to_string(),
                                            software.artist_name.to_string(),
                                        ])?;
                                    }
                                    LookupResult::Artist(artist) => {
                                        log::info!(
                                            "Unexpected artist result: {}",
                                            artist.artist_id,
                                        );
                                    }
                                }
                            }
                        }
                    }

                    let results = google_client
                        .search(&query, lang, country, PriceFilter::default(), 100)
                        .await?;

                    if full {
                        log::info!(
                            "Downloading full information for {} Google apps",
                            results.len()
                        );
                    }

                    for app in results {
                        writer.write_record([
                            "google".to_string(),
                            "".to_string(),
                            app.id.clone(),
                            app.developer_id()
                                .map(|developer_id| developer_id.to_string())
                                .unwrap_or_default(),
                            app.title,
                            app.developer.name,
                        ])?;

                        if full {
                            google_client.app(&app.id, lang, country).await?;
                            writer.flush()?;
                            tokio::time::sleep(Duration::from_millis(delay)).await;
                        }
                    }
                }
                ApiCommand::Suggest {
                    query,
                    country,
                    lang,
                } => {
                    for suggestion in apple_client
                        .lookup_suggestions(&query, country, lang)
                        .await?
                    {
                        writer.write_record([
                            "apple".to_string(),
                            query.to_string(),
                            lang.to_string(),
                            country.to_string(),
                            suggestion,
                        ])?;
                    }

                    for suggestion in google_client
                        .lookup_suggestions(&query, country, lang)
                        .await?
                    {
                        writer.write_record([
                            "google".to_string(),
                            query.to_string(),
                            lang.to_string(),
                            country.to_string(),
                            suggestion,
                        ])?;
                    }
                }
            }

            writer.flush()?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "app-store-access", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    /// Make API requests and save responses
    Api {
        #[clap(long, default_value = "data/apple/")]
        apple_archive: PathBuf,
        #[clap(long, default_value = "data/google/")]
        google_archive: PathBuf,
        #[clap(subcommand)]
        command: ApiCommand,
    },
}

#[derive(Debug, Parser)]
enum ApiCommand {
    /// Perform a search for a given query string
    Search {
        #[clap(long)]
        query: String,
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long, default_value = "en")]
        lang: Language,
        /// Download full app information for results
        #[clap(long)]
        full: bool,
        /// Time to wait between image requests in milliseconds
        #[clap(long, default_value = "500")]
        delay: u64,
    },
    /// Look up autocomplete suggestions for a given query string
    Suggest {
        #[clap(long)]
        query: String,
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long, default_value = "en")]
        lang: Language,
    },
}
