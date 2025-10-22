use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use swc_ecma_ast::{CallExpr, EsVersion, Expr};

static DS_SELS: LazyLock<[(usize, Selector); 10]> = LazyLock::new(|| {
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .map(|i| (i, Selector::parse(&format!("script.ds\\:{i}")).unwrap()))
});

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid script")]
    InvalidScript(Expr),
    #[error("JavaScript parse error")]
    JavaScriptParse(#[from] app_store_access::js::Error),
    #[error("JSON conversion error")]
    JsonConversion(#[from] ecmade::error::Error),
    #[error("Multiple scripts")]
    MultipleScripts { ds_index: usize, count: usize },
    #[error("Missing script")]
    MissingScript { ds_index: usize },
}

fn extract_object<T: serde::de::DeserializeOwned>(expr: &Expr) -> Result<T, Error> {
    match expr {
        Expr::Call(CallExpr { args, .. }) if args.len() == 1 => {
            Ok(ecmade::from_expr::<T>(&args[0].expr)?)
        }
        _ => Err(Error::InvalidScript(expr.clone())),
    }
}

pub fn get_ds_values(html: &Html) -> Result<BTreeMap<usize, Value>, Error> {
    DS_SELS
        .iter()
        .filter_map(|(i, ds_sel)| {
            let ds_scripts = html.select(ds_sel).collect::<Vec<_>>();

            match ds_scripts.len() {
                0 => None,
                1 => {
                    let ds_script = ds_scripts[0];

                    Some(
                        app_store_access::js::parse_js(
                            &ds_script.inner_html(),
                            EsVersion::default(),
                        )
                        .map_err(Error::from)
                        .and_then(|expr| extract_object(&expr))
                        .map(|value| (*i, value)),
                    )
                }
                other => Some(Err(Error::MultipleScripts {
                    ds_index: *i,
                    count: other,
                })),
            }
        })
        .collect()
}

pub fn parse_ds_value<T: serde::de::DeserializeOwned>(
    html: &Html,
    index: usize,
) -> Result<T, Error> {
    let scripts = html.select(&DS_SELS[index].1).collect::<Vec<_>>();

    match scripts.len() {
        0 => Err(Error::MissingScript { ds_index: index }),
        1 => {
            let script = scripts[0];

            app_store_access::js::parse_js(&script.inner_html(), EsVersion::default())
                .map_err(Error::from)
                .and_then(|expr| extract_object(&expr))
        }
        other => Err(Error::MultipleScripts {
            ds_index: index,
            count: other,
        }),
    }
}
