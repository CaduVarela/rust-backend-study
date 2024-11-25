use sqlx::{Row, Column, SqlitePool};
use serde_json::Value;

pub async fn execute_query(pool: &SqlitePool, query: &str) -> Result<Value, sqlx::Error> {
    let rows = sqlx::query(query).fetch_all(pool).await?;

    let result: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            for column in row.columns() {
                let key = column.name().to_string();

                let value: Value = match row.try_get::<String, _>(column.ordinal()) {
                    Ok(val) => Value::String(val),
                    Err(_) => match row.try_get::<i64, _>(column.ordinal()) {
                        Ok(val) => Value::Number(val.into()),
                        Err(_) => match row.try_get::<f64, _>(column.ordinal()) {
                            Ok(val) => {
                                if let Some(num) = serde_json::Number::from_f64(val) {
                                    Value::Number(num)
                                } else {
                                    Value::Null
                                }
                            },
                            Err(_) => Value::Null, 
                        },
                    },
                };
                map.insert(key, value);
            }
            Value::Object(map)
        })
        .collect();

    Ok(Value::Array(result))
}