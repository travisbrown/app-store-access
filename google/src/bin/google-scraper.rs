use app_store_access::{client::SuggestionClient, country::Country, language::Language};
use app_store_access_google::{
    archive::Data,
    request::{
        RequestData,
        params::{developer::DeveloperId, review::SortOrder, search::PriceFilter},
    },
};
use cli_helpers::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("Client error")]
    Client(#[from] app_store_access_google::client::Error),
    #[error("Client suggestion error")]
    SuggestClient(#[from] app_store_access_google::client::suggest::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("Google parsing error")]
    GoogleParse(#[from] app_store_access_google::parse::Error),
    #[error("JavaScript error")]
    Js(#[from] app_store_access::js::Error),
    #[error("JSON file error")]
    JsonFile(PathBuf, serde_json::Error),
}

impl Error {
    fn from_scraper_store_error(
        path: PathBuf,
        error: scraper_trail::archive::store::Error,
    ) -> Self {
        match error {
            scraper_trail::archive::store::Error::Io(error) => Self::from(error),
            scraper_trail::archive::store::Error::Json(error) => Self::JsonFile(path, error),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::Api {
            country,
            lang,
            archive,
            command,
        } => {
            let client =
                app_store_access_google::client::Client::new(Some(&archive), Default::default())?;

            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(std::io::stdout());

            match command {
                ApiCommand::App { id } => {
                    let app = client.app(&id, lang, country).await?;

                    match app {
                        Some(app) => {
                            writer.write_record([
                                app.app_id.clone(),
                                app.developer.id.to_string(),
                                app.title,
                                app.developer.name,
                            ])?;
                        }
                        None => {
                            ::log::warn!("App not found: {}", id);
                        }
                    }
                }
                ApiCommand::Search { query, full, delay } => {
                    let results = client
                        .search(&query, lang, country, PriceFilter::Paid, 100)
                        .await?;

                    if full {
                        ::log::info!("Downloading full information for {} apps", results.len());
                    }

                    for app in results {
                        writer.write_record([
                            app.id.clone(),
                            app.developer_id()
                                .map(|developer_id| developer_id.to_string())
                                .unwrap_or_default(),
                            app.title,
                            app.developer.name,
                        ])?;

                        if full {
                            client.app(&app.id, lang, country).await?;
                            writer.flush()?;
                            tokio::time::sleep(Duration::from_millis(delay)).await;
                        }
                    }
                }
                ApiCommand::Developer { id } => {
                    let apps = client.developer(&id, lang, country, 100).await?;

                    match apps {
                        Some(apps) => {
                            for app in apps {
                                writer.write_record([app.app_id, app.developer_id.to_string()])?;
                            }
                        }
                        None => {
                            ::log::warn!("Developer not found: {}", id);
                        }
                    }
                }
                ApiCommand::Reviews { id } => {
                    let reviews = client
                        .reviews(&id.to_string(), lang, country, SortOrder::Newest, 150)
                        .await?;

                    for review in reviews {
                        writer.write_record([
                            review.id.to_string(),
                            review.user.id.map(|id| id.to_string()).unwrap_or_default(),
                            review.user.display_name,
                        ])?;
                    }
                }
                ApiCommand::Suggest { query } => {
                    for suggestion in client.lookup_suggestions(&query, country, lang).await? {
                        writer.write_record([
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
        Command::Store {
            archive,
            most_recent_first,
            command,
        } => {
            let store = scraper_trail::archive::store::Store::new(archive);

            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(std::io::stdout());

            match command {
                StoreCommand::Apps => {
                    for (path, entry) in store.entries::<Data>(most_recent_first)? {
                        let entry =
                            entry.map_err(|error| Error::from_scraper_store_error(path, error))?;

                        if let Data::Detail(app) = entry.exchange.response.data {
                            writer.write_record([app.app_id, app.developer.id.to_string()])?;
                        }
                    }
                }
                StoreCommand::Search => {
                    for (path, entry) in store.entries::<Data>(most_recent_first)? {
                        let entry =
                            entry.map_err(|error| Error::from_scraper_store_error(path, error))?;

                        if let Data::Search(page) = entry.exchange.response.data {
                            let request = match &entry.request_params.data {
                                RequestData::Search { query, price } => {
                                    Some((query, price.as_ref()))
                                }
                                _ => None,
                            };

                            let request_token = match &entry.request_params.data {
                                RequestData::Pagination { token, .. } => Some(token),
                                _ => None,
                            };

                            for app in page.apps.unwrap_or_default() {
                                writer.write_record([
                                    request
                                        .map(|(query, _)| query.to_string())
                                        .unwrap_or_default(),
                                    request
                                        .and_then(|(_, price)| price)
                                        .map(|price| price.to_string())
                                        .unwrap_or_default(),
                                    app.id,
                                    app.developer.id,
                                    request_token
                                        .map(|token| token.to_string())
                                        .unwrap_or_default(),
                                    page.token.clone().unwrap_or_default(),
                                ])?;
                            }
                        }
                    }
                }

                #[cfg(not(feature = "strict"))]
                StoreCommand::FixStrict => {
                    use app_store_access_google::model::strict_fix::Fix;
                    use bounded_static::ToBoundedStatic;

                    let mut fixes = vec![];

                    for (path, entry) in store.entries::<Data>(most_recent_first)? {
                        let entry = entry.map_err(|error| {
                            Error::from_scraper_store_error(path.clone(), error)
                        })?;

                        match entry.exchange.response.data {
                            Data::Reviews(page) => {
                                fixes.extend(
                                    app_store_access_google::model::strict_fix::review_page_fixes(
                                        &page,
                                    )
                                    .iter()
                                    .map(|fix| fix.to_static()),
                                );
                            }
                            _ => {}
                        }
                    }

                    fixes.sort();
                    fixes.dedup();

                    for fix in fixes {
                        match fix {
                            Fix::UnknownReviewCriterionType(value) => {
                                writer.write_record(["review_criterion_type", &value])?;
                            }
                        }
                    }
                }
            }

            writer.flush()?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "app-store-access-google", version, author)]
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
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long, default_value = "en")]
        lang: Language,
        #[clap(long, default_value = "data/google/")]
        archive: PathBuf,
        #[clap(subcommand)]
        command: ApiCommand,
    },
    /// Read data from the local store
    Store {
        #[clap(long)]
        archive: PathBuf,
        #[clap(long)]
        most_recent_first: bool,
        #[clap(subcommand)]
        command: StoreCommand,
    },
}

#[derive(Debug, Parser)]
enum ApiCommand {
    /// Request details for an app by ID
    App {
        #[clap(long)]
        id: String,
    },
    /// Perform a search for a given query string
    Search {
        #[clap(long)]
        query: String,
        /// Download full app information for results
        #[clap(long)]
        full: bool,
        /// Time to wait between image requests in milliseconds
        #[clap(long, default_value = "500")]
        delay: u64,
    },
    /// Request a list of apps for a developer by ID (may be an integer or a string)
    Developer {
        #[clap(long)]
        id: DeveloperId,
    },
    /// Look up reviews for an app by ID
    Reviews {
        #[clap(long)]
        id: String,
    },
    /// Look up autocomplete suggestions for a given query string
    Suggest {
        #[clap(long)]
        query: String,
    },
}

#[derive(Debug, Parser)]
enum StoreCommand {
    Apps,
    Search,
    #[cfg(not(feature = "strict"))]
    FixStrict,
}
