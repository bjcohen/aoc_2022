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
    let file = File::open("input3.txt")?;
    let lines = BufReader::new(file).lines();

    let client_options = ClientOptions::parse("mongodb://localhost").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("aoc2022");

    let input3 = database.collection::<Document>("input3");

    input3.drop(None).await?;

    input3
        .insert_many(
            lines
                .map(|l| {
                    l.map_err(Error::new).map(|items| {
                        doc! { "items": items }
                    })
                })
                .collect::<Result<Vec<Document>>>()?,
            None,
        )
        .await?;

    let mut cursor = input3
        .aggregate(
            [doc! {
                "$project": doc! {
                    "c1": doc! {
                        "$substrBytes": [ "$items", 0, doc! {
                            "$divide": [ doc! { "$strLenBytes": "$items" }, 2 ],
                        } ],
                    },
                    "c2": doc! {
                        "$substrBytes": [ "$items", doc! {
                            "$divide": [ doc! { "$strLenBytes": "$items" }, 2 ],
                        }, -1 ],
                    },
                },
            }, doc! {
                "$project": doc! {
                    "c1": doc! {
                        "$map": doc! {
                            "input": doc! {
                                "$range": [0, doc! { "$strLenBytes": "$c1" }],
                            },
                            "in": doc! {
                                "$substrBytes": [ "$c1", "$$this", 1 ],
                            },
                        },
                    },
                    "c2": doc! {
                        "$map": doc! {
                            "input": doc! {
                                "$range": [0, doc! { "$strLenBytes": "$c2" }],
                            },
                            "in": doc! {
                                "$substrBytes": [ "$c2", "$$this", 1 ],
                            },
                        },
                    },
                },
            }, doc! {
                "$group": doc! {
                    "_id": Bson::Null,
                    "prioritySum": doc! {
                        "$sum": doc! {
                            "$function": doc! {
                                "body": "function(c) {
  return (c.toUpperCase() === c) ? c.charCodeAt(0) - 'A'.charCodeAt(0) + 27 : c.charCodeAt(0) - 'a'.charCodeAt(0) + 1
}",
                                "args": [ doc! {
                                    "$first": doc! {
                                        "$setIntersection": [ "$c1", "$c2" ],
                                    },
                                } ],
                                "lang": "js",
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

    let mut cursor = input3
        .aggregate(
            [
                doc! {
                    "$setWindowFields": doc! {
                        "sortBy": doc! {
                            "_id": 1,
                        },
                        "output": doc! {
                            "i": doc! {
                                "$count": doc! {},
                                "window": doc! {
                                    "documents": ["unbounded", -1],
                                }
                            },
                        },
                    },
                },
                doc! {
                    "$group": {
                        "_id": doc! {
                            "$divide": [
                                doc! {
                                    "$subtract": [
                                        "$i",
                                        doc! {
                                            "$mod": ["$i", 3],
                                        },
                                    ],
                                },
                                3,
                            ],
                        },
                        "groupItems": doc! {
                            "$push": "$items",
                        },
                    },
                },
                doc! {
                    "$project": {
                        "groupBadge": doc! {
                            "$first": doc! {
                                "$setIntersection": [
                                    doc! {
                                        "$map": doc! {
                                            "input": doc! {
                                                "$range": [0, doc! { "$strLenBytes": doc! {
                                                    "$arrayElemAt": ["$groupItems", 0],
                                                }}],
                                            },
                                            "in": doc! {
                                                "$substrBytes": [ doc! {
                                                    "$arrayElemAt": ["$groupItems", 0],
                                                }, "$$this", 1 ],
                                            },
                                        },
                                    },
                                    doc! {
                                        "$map": doc! {
                                            "input": doc! {
                                                "$range": [0, doc! { "$strLenBytes": doc! {
                                                    "$arrayElemAt": ["$groupItems", 1],
                                                }}],
                                            },
                                            "in": doc! {
                                                "$substrBytes": [ doc! {
                                                    "$arrayElemAt": ["$groupItems", 1],
                                                }, "$$this", 1 ],
                                            },
                                        },
                                    },
                                    doc! {
                                        "$map": doc! {
                                            "input": doc! {
                                                "$range": [0, doc! { "$strLenBytes": doc! {
                                                    "$arrayElemAt": ["$groupItems", 2],
                                                }}],
                                            },
                                            "in": doc! {
                                                "$substrBytes": [ doc! {
                                                    "$arrayElemAt": ["$groupItems", 2],
                                                }, "$$this", 1 ],
                                            },
                                        },
                                    },
                                ],
                            },
                        },
                    },
                },
                doc! {
                    "$group": doc! {
                        "_id": Bson::Null,
                        "prioritySum": doc! {
                            "$sum": doc! {
                                "$function": doc! {
                                    "body": "function(c) {
  return (c.toUpperCase() === c) ? c.charCodeAt(0) - 'A'.charCodeAt(0) + 27 : c.charCodeAt(0) - 'a'.charCodeAt(0) + 1
}",
                                    "args": [ "$groupBadge" ],
                                    "lang": "js",
                                },
                            },
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
