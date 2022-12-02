use anyhow::{anyhow, Error, Result};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open("input2.txt")?;
    let lines = BufReader::new(file).lines();

    let client_options = ClientOptions::parse("mongodb://localhost").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("aoc2022");

    let input2 = database.collection::<Document>("input2");

    input2.drop(None).await?;

    input2
        .insert_many(
            lines
                .map(|l| {
                    l.map_err(Error::new)
                        .and_then(|c| {
                            c.split_once(' ')
                                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                                .ok_or_else(|| anyhow!("Couldn't parse line"))
                        })
                        .map(|(opp, self_)| {
                            doc! { "self": self_, "opp": opp }
                        })
                })
                .collect::<Result<Vec<Document>>>()?,
            None,
        )
        .await?;

    let mut cursor = input2
        .aggregate(
            [
                doc! {
                    "$project": doc! {
                        "score": doc! {
                            "$add": [
                                doc! {
                                    "$indexOfArray": [["X", "Y", "Z"], "$self"]
                                },
                                1,
                                doc! {
                                    "$arrayElemAt": [
                                        [3, 6, 0],
                                        doc! {
                                            "$mod": [
                                                doc! {
                                                    "$add": [
                                                        doc! {
                                                            "$subtract": [
                                                                doc! {
                                                                    "$indexOfArray": [
                                                                        ["X", "Y", "Z"],
                                                                        "$self",
                                                                    ],
                                                                },
                                                                doc! {
                                                                    "$indexOfArray": [
                                                                        ["A", "B", "C"],
                                                                        "$opp",
                                                                    ],
                                                                },
                                                            ],
                                                        },
                                                        3,
                                                    ],
                                                },
                                                3,
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    }
                },
                doc! {
                    "$group": doc! {
                        "_id": 0,
                        "total_score": doc! {
                            "$sum": "$score",
                        },
                    },
                },
            ],
            None,
        )
        .await?;
    while let Some(result) = cursor.try_next().await? {
        println!("{}", result);
    }

    let mut cursor = input2
        .aggregate(
            [
                doc! {
                    "$project": doc! {
                        "score": doc! {
                            "$add": [
                                doc! {
                                    "$arrayElemAt": [
                                        [0, 3, 6],
                                        doc! {
                                            "$indexOfArray": [["X", "Y", "Z"], "$self"]
                                        },
                                    ],
                                },
                                doc! {
                                    "$arrayElemAt": [
                                        [3, 1, 2],
                                        doc! {
                                            "$mod": [
                                                doc! {
                                                    "$add": [
                                                        doc! {
                                                            "$indexOfArray": [
                                                                ["X", "Y", "Z"],
                                                                "$self",
                                                            ],
                                                        },
                                                        doc! {
                                                            "$indexOfArray": [
                                                                ["A", "B", "C"],
                                                                "$opp",
                                                            ],
                                                        },
                                                    ],
                                                },
                                                3,
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    }
                },
                doc! {
                    "$group": doc! {
                        "_id": 0,
                        "total_score": doc! {
                            "$sum": "$score",
                        },
                    },
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
