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
    let file = File::open("input4.txt")?;
    let lines = BufReader::new(file).lines();

    let client_options = ClientOptions::parse("mongodb://localhost").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("aoc2022");

    let input4 = database.collection::<Document>("input4");

    input4.drop(None).await?;

    input4
        .insert_many(
            lines
                .map(|l| {
                    l.map_err(Error::new).map(|items| {
                        doc! { "pairs": items }
                    })
                })
                .collect::<Result<Vec<Document>>>()?,
            None,
        )
        .await?;

    let mut cursor = input4
        .aggregate(
            [doc! {
                "$project": doc! {
                    "s1": doc! {
                        "$toInt": doc! {
                            "$arrayElemAt": [ doc! {
                                "$split": [ doc! {
                                    "$arrayElemAt": [ doc! {
                                        "$split": [ "$pairs", "," ],
                                    }, 0],
                                }, "-" ],
                            }, 0],
                        },
                    },
                    "e1": doc! {
                        "$toInt": doc! {
                            "$arrayElemAt": [ doc! {
                                "$split": [ doc! {
                                    "$arrayElemAt": [ doc! {
                                        "$split": [ "$pairs", "," ],
                                    }, 0],
                                }, "-" ],
                            }, 1],
                        },
                    },
                    "s2": doc! {
                        "$toInt": doc! {
                            "$arrayElemAt": [ doc! {
                                "$split": [ doc! {
                                    "$arrayElemAt": [ doc! {
                                        "$split": [ "$pairs", "," ],
                                    }, 1],
                                }, "-" ],
                            }, 0],
                        },
                    },
                    "e2": doc! {
                        "$toInt": doc! {
                            "$arrayElemAt": [ doc! {
                                "$split": [ doc! {
                                    "$arrayElemAt": [ doc! {
                                        "$split": [ "$pairs", "," ],
                                    }, 1],
                                }, "-" ],
                            }, 1],
                        },
                    },
                },
            }, doc! {
                "$group": doc! {
                    "_id": Bson::Null,
                    "numContained": doc! {
                        "$sum": doc! {
                            "$toInt": doc! {
                                "$or": [
                                    doc! {
                                        "$and": [
                                            doc! {
                                                "$gte": [ "$s2", "$s1" ],
                                            },
                                            doc! {
                                                "$lte": [ "$e2", "$e1" ],
                                            },
                                        ],
                                    },
                                    doc! {
                                        "$and": [
                                            doc! {
                                                "$gte": [ "$s1", "$s2" ],
                                            },
                                            doc! {
                                                "$lte": [ "$e1", "$e2" ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        },
                    },
                    "numOverlapping": doc! {
                        "$sum": doc! {
                            "$toInt": doc! {
                                "$and": [
                                    doc! {
                                        "$lte": [ "$s1", "$e2" ],
                                    },
                                    doc! {
                                        "$lte": [ "$s2", "$e1" ],
                                    },
                                ],
                            },
                        },
                    },
                },
            }],
            None,
        )
        .await?;
    while let Some(result) = cursor.try_next().await? {
        println!("{}", result);
    }

    Ok(())
}
