use app_store_access::{client::SuggestionClient, country::Country, language::Language};
use app_store_access_apple::{archive::Data, model::lookup::LookupResult, request::Request};
use cli_helpers::prelude::*;
use std::io::Write;
use std::path::PathBuf;

const LOOKUP_PAGE_SIZE: usize = 50;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("Client error")]
    Client(#[from] app_store_access_apple::client::Error),
    #[error("Client suggestion error")]
    SuggestClient(#[from] app_store_access_apple::client::suggest::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
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
        Command::Api { archive, command } => {
            let client = app_store_access_apple::client::Client::new(Some(archive));

            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(std::io::stdout());

            match command {
                ApiCommand::App { id, country } => match client.app(id, country).await? {
                    Some(page) => {
                        let app = page
                            .store_platform_data
                            .product_dv
                            .results
                            .get(&id)
                            .unwrap();

                        writer.write_record([
                            id.to_string(),
                            app.common
                                .bundle_id
                                .as_ref()
                                .map(|id| id.to_string())
                                .unwrap_or_default(),
                            app.common.name.to_string(),
                        ])?;
                    }
                    None => {
                        log::warn!("App not found: {}", id);
                    }
                },
                ApiCommand::Search {
                    query,
                    country,
                    lang,
                } => {
                    let page = client.search(&query, country, lang).await?;

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

                    log::info!("Downloading {} search results", ids.len());

                    for id_chunk in ids.chunks(LOOKUP_PAGE_SIZE) {
                        let result = client.lookup_ids(id_chunk, country, lang).await?;

                        for result in result.results {
                            match &result {
                                LookupResult::Software(software) => {
                                    writer.write_record([
                                        software.track_id.to_string(),
                                        software.bundle_id.to_string(),
                                        software.artist_id.to_string(),
                                        software.track_name.to_string(),
                                        software.artist_name.to_string(),
                                    ])?;
                                }
                                LookupResult::Artist(artist) => {
                                    log::info!("Unexpected artist result: {}", artist.artist_id,);
                                }
                            }
                        }
                    }
                }
                ApiCommand::LookupIds { id, country, lang } => {
                    for id_chunk in id.chunks(LOOKUP_PAGE_SIZE) {
                        let result = client.lookup_ids(id_chunk, country, lang).await?;

                        for result in result.results {
                            match &result {
                                LookupResult::Software(software) => {
                                    writer.write_record([
                                        "software".to_string(),
                                        software.track_id.to_string(),
                                        software.bundle_id.to_string(),
                                        software.artist_id.to_string(),
                                        software.track_name.to_string(),
                                        software.artist_name.to_string(),
                                    ])?;
                                }
                                LookupResult::Artist(artist) => {
                                    writer.write_record([
                                        "artist".to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        artist.artist_id.to_string(),
                                        "".to_string(),
                                        artist.artist_name.to_string(),
                                    ])?;
                                }
                            }
                        }
                    }
                }
                ApiCommand::LookupBundleIds { id, country, lang } => {
                    let ids = id.iter().map(|id| id.as_str()).collect::<Vec<_>>();

                    for id_chunk in ids.chunks(LOOKUP_PAGE_SIZE) {
                        let result = client.lookup_bundle_ids(id_chunk, country, lang).await?;

                        for result in result.results {
                            match &result {
                                LookupResult::Software(software) => {
                                    writer.write_record([
                                        software.track_id.to_string(),
                                        software.bundle_id.to_string(),
                                        software.artist_id.to_string(),
                                        software.track_name.to_string(),
                                        software.artist_name.to_string(),
                                    ])?;
                                }
                                LookupResult::Artist(artist) => {
                                    log::info!("Unexpected artist result: {}", artist.artist_id,);
                                }
                            }
                        }
                    }
                }
                ApiCommand::Reviews {
                    id,
                    country,
                    sort_by_helpful,
                    page,
                } => {
                    let sort = if sort_by_helpful {
                        app_store_access_apple::request::params::review::SortOrder::Helpful
                    } else {
                        app_store_access_apple::request::params::review::SortOrder::Recent
                    };

                    let pages = client.reviews(id, country, sort, page).await?;

                    for page in pages {
                        for entry in page.feed.entries() {
                            writer.write_record([
                                entry.id.label.to_string(),
                                entry.link.attributes.href.to_string(),
                                entry.author.name.label.to_string(),
                            ])?;
                        }
                    }
                }
                ApiCommand::Ratings { id, country } => {
                    let content = client.ratings_html(id, country).await?;

                    writeln!(std::io::stdout(), "{}", content)?;
                }
                ApiCommand::Suggest {
                    query,
                    country,
                    lang,
                } => {
                    let countries = country.map(|country| vec![country]).unwrap_or_else(|| {
                        app_store_access_apple::request::markets::MarketCode::known_countries()
                    });

                    for country in countries {
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
                    for (path, entry) in store.entries::<Data<'static>>(most_recent_first)? {
                        let entry = entry.map_err(|error| {
                            Error::from_scraper_store_error(path.clone(), error)
                        })?;

                        if let Data::App(page) = entry.exchange.response.data {
                            if let Request::App { id, .. } = entry.request_params {
                                if let Some(app) =
                                    page.store_platform_data.product_dv.results.get(&id)
                                {
                                    writer.write_record([
                                        id.to_string(),
                                        app.common
                                            .bundle_id
                                            .as_ref()
                                            .map(|id| id.to_string())
                                            .unwrap_or_default(),
                                        app.common.name.to_string(),
                                    ])?;
                                } else {
                                    log::warn!(
                                        "Empty app result: {}",
                                        path.as_os_str().to_string_lossy()
                                    );
                                }
                            } else {
                                log::warn!(
                                    "Unexpected request: {}",
                                    path.as_os_str().to_string_lossy()
                                );
                            }
                        }
                    }
                }
                StoreCommand::Search => {}

                #[cfg(not(feature = "strict"))]
                StoreCommand::FixStrict => {
                    use app_store_access_apple::model::strict_fix::Fix;
                    use bounded_static::ToBoundedStatic;

                    let mut fixes = vec![];

                    for (path, entry) in store.entries::<Data<'static>>(most_recent_first)? {
                        let entry = entry.map_err(|error| {
                            Error::from_scraper_store_error(path.clone(), error)
                        })?;

                        match entry.exchange.response.data {
                            Data::App(app) => {
                                fixes.extend(
                                    app_store_access_apple::model::strict_fix::app_fixes(&app)
                                        .iter()
                                        .map(|fix| fix.to_static()),
                                );
                            }
                            Data::Lookup(list) => {
                                fixes.extend(
                                    app_store_access_apple::model::strict_fix::lookup_fixes(&list)
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
                            Fix::UnknownContentRatingAdvisory(value) => {
                                writer.write_record(["content_rating_advisory", &value, "", ""])?;
                            }
                            Fix::InvalidGenreName {
                                name,
                                found_id,
                                expected_id,
                            } => {
                                writer.write_record([
                                    "genre_name",
                                    &name,
                                    expected_id.to_string().as_str(),
                                    found_id.to_string().as_str(),
                                ])?;
                            }
                            Fix::UnknownGenreName { name, expected_id } => {
                                writer.write_record([
                                    "genre_name",
                                    &name,
                                    expected_id.to_string().as_str(),
                                    "",
                                ])?;
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
#[clap(name = "apple-scraper", version, author)]
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
        id: u64,
        #[clap(long, default_value = "us")]
        country: Country,
    },
    /// Perform a search for a given query string
    Search {
        #[clap(long)]
        query: String,
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long, default_value = "en")]
        lang: Language,
    },
    /// Look up apps or developers by ID (option can be provided multiple times)
    LookupIds {
        #[clap(long)]
        id: Vec<u64>,
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long, default_value = "en")]
        lang: Language,
    },
    /// Look up apps by bundle ID (option can be provided multiple times)
    LookupBundleIds {
        #[clap(long)]
        id: Vec<String>,
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long, default_value = "en")]
        lang: Language,
    },
    /// Look up reviews for an app by ID
    Reviews {
        #[clap(long)]
        id: u64,
        #[clap(long, default_value = "us")]
        country: Country,
        #[clap(long)]
        sort_by_helpful: bool,
        #[clap(long, default_value = "1")]
        page: usize,
    },
    /// Look up ratings by app ID (warning: raw HTML)
    Ratings {
        #[clap(long)]
        id: u64,
        #[clap(long, default_value = "us")]
        country: Country,
    },
    /// Look up autocomplete suggestions for a given query string
    Suggest {
        #[clap(long)]
        query: String,
        /// If no country is provided, the API will be queried for all markets
        #[clap(long)]
        country: Option<Country>,
        #[clap(long, default_value = "en")]
        lang: Language,
    },
}

#[derive(Debug, Parser)]
enum StoreCommand {
    Apps,
    Search,
    #[cfg(not(feature = "strict"))]
    FixStrict,
}
