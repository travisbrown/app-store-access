use serde_json::Value;
use swc_ecma_ast::{
    ArrayLit, Expr, Lit, Number, ObjectLit, Prop, PropName, PropOrSpread, SpreadElement,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid object key")]
    InvalidObjectKey(PropName),
    #[error("Invalid number")]
    InvalidNumber(Number),
    #[error("Invalid literal")]
    InvalidLiteral(Lit),
    #[error("Invalid prop")]
    InvalidProp(Prop),
    #[error("Unexpected spread")]
    UnexpectedSpread(SpreadElement),
    #[error("Unexpected expression")]
    UnexpectedExpr(Expr),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
}

pub fn expr_to_json(expr: &Expr) -> Result<Value, Error> {
    match expr {
        Expr::Lit(lit) => match lit {
            Lit::Str(str) => Ok(Value::String(
                str.value
                    .as_str()
                    .ok_or_else(|| Error::InvalidLiteral(lit.clone()))?
                    .to_string(),
            )),
            Lit::Num(num) => Ok(Value::Number(
                number_to_json_number(num).ok_or_else(|| Error::InvalidNumber(num.clone()))?,
            )),
            Lit::BigInt(num) => num
                .raw
                .as_ref()
                .ok_or_else(|| Error::InvalidLiteral(lit.clone()))
                .and_then(|str| Ok(serde_json::from_str(str.as_str())?)),
            Lit::Null(_) => Ok(Value::Null),
            _ => Err(Error::InvalidLiteral(lit.clone())),
        },
        Expr::Object(ObjectLit { props, .. }) => {
            let fields = props
                .iter()
                .map(|prop_or_spread| match prop_or_spread {
                    PropOrSpread::Prop(prop) => match &**prop {
                        Prop::KeyValue(kvp) => {
                            let key = prop_name_to_str(&kvp.key)
                                .ok_or_else(|| Error::InvalidObjectKey(kvp.key.clone()))?;
                            let value = expr_to_json(&kvp.value)?;

                            Ok((key.to_string(), value))
                        }
                        _ => Err(Error::InvalidProp(*prop.clone())),
                    },
                    PropOrSpread::Spread(spread) => Err(Error::UnexpectedSpread(spread.clone())),
                })
                .collect::<Result<_, Error>>()?;

            Ok(Value::Object(fields))
        }
        Expr::Array(ArrayLit { elems, .. }) => {
            let elems = elems
                .iter()
                .map(|elem| {
                    elem.as_ref()
                        .ok_or_else(|| {
                            // TODO: Figure out what `None` means here.
                            Error::UnexpectedExpr(expr.clone())
                        })
                        .and_then(|elem| expr_to_json(&elem.expr))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Value::Array(elems))
        }
        _ => Err(Error::UnexpectedExpr(expr.clone())),
    }
}

fn prop_name_to_str(prop_name: &PropName) -> Option<&str> {
    prop_name
        .as_str()
        .and_then(|str| str.value.as_str())
        .or_else(|| prop_name.as_ident().map(|ident| ident.sym.as_str()))
}

fn number_to_json_number(num: &Number) -> Option<serde_json::Number> {
    if let Some(str) = num.raw.as_ref().map(swc_atoms::Atom::as_str)
        && !str.contains('.')
    {
        serde_json::Number::from_i128(num.value as i128)
    } else {
        serde_json::Number::from_f64(num.value)
    }
}
