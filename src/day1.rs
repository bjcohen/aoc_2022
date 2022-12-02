use anyhow::{Error, Result};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::ClientOptions,
    Client,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open("input1.txt")?;
    let lines = BufReader::new(file).lines();

    let client_options = ClientOptions::parse("mongodb://localhost").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("aoc2022");

    let input1 = database.collection::<Document>("input1");

    input1.drop(None).await?;

    input1
        .insert_many(
            lines
                .map(|l| {
                    l.map_err(Error::new)
                        .map(|c| c.parse().ok())
                        .map(|o: Option<i32>| {
                            if let Some(n) = o {
                                doc! { "calories": n }
                            } else {
                                doc! {}
                            }
                        })
                })
                .collect::<Result<Vec<Document>>>()?,
            None,
        )
        .await?;

    let mut cursor = input1
        .aggregate(
            [
                doc! {
                    "$setWindowFields": doc! {
                        "partitionBy": 0,
                        "sortBy": doc! {
                            "_id": 1
                        },
                        "output": doc! {
                            "elfNum": doc! {
                                "$sum": doc! {
                                    "$cond": doc! {
                                        "if": doc! {
                                            "$lt": [
                                                "$calories",
                                                Bson::Null
                                            ]
                                        },
                                        "then": 1,
                                        "else": 0
                                    }
                                },
                                "window": doc! {
                                    "documents": [
                                        "unbounded",
                                        "current"
                                    ]
                                }
                            }
                        }
                    }
                },
                doc! {
                    "$match": doc! {
                        "$expr": "$calories"
                    }
                },
                doc! {
                    "$group": doc! {
                        "_id": "$elfNum",
                        "totalCalories": doc! {
                            "$sum": "$calories"
                        }
                    }
                },
                doc! {
                    "$group": doc! {
                        "_id": 0,
                        "topElfCalories": doc! {
                            "$topN": doc! {
                                "output": "$totalCalories",
                                "sortBy": doc! {
                                    "totalCalories": -1,
                                },
                                "n": 1,
                            }
                        },
                        "topThreeElfCalories": doc! {
                            "$topN": doc! {
                                "output": "$totalCalories",
                                "sortBy": doc! {
                                    "totalCalories": -1,
                                },
                                "n": 3,
                            }
                        }
                    }
                },
                doc! {
                    "$project": doc! {
                        "topElfCalories": doc! {
                            "$arrayElemAt": ["$topElfCalories", 0],
                        },
                        "topThreeElfCalories": doc! {
                            "$sum": "$topThreeElfCalories",
                        }
                    }
                },
            ],
            None,
        )
        .await?;
    while let Some(result) = cursor.try_next().await? {
        println!("{}", result);
    }

    Ok(())
}
