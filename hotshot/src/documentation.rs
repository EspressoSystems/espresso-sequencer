// This is prosaic documentation, we don't need clippy
#![allow(
    clippy::all,
    clippy::pedantic,
    missing_docs,
    clippy::missing_docs_in_private_items,
    non_camel_case_types
)]
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("basic_hotstuff", "../../docs/HotShotDocs/img/basic_hotstuff.svg")),
doc = ::embed_doc_image::embed_image!("chained_hotstuff", "../../docs/HotShotDocs/img/chained_hotstuff.svg"))
]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
#![ doc = include_str!("../../../docs/HotShotDocs/main.md")]
