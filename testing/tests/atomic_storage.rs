#![cfg(foo)]
use hotshot::{
    certificate::QuorumCertificate,
    data::LeafType,
    demos::vdemo::{
        random_quorom_certificate, random_transaction, random_validating_leaf, VDemoBlock,
        VDemoState,
    },
    traits::{Block, State, Storage},
};
use hotshot_types::{data::ViewNumber, traits::state::TestableState};
use rand::thread_rng;

type AtomicStorage = hotshot::traits::implementations::AtomicStorage<DEntryState>;

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_happy_path_blocks() {
    // This folder will be destroyed when the last handle to it closes
    let file = tempfile::tempdir().expect("Could not create temp dir");
    let path = file.path();
    println!("Using store in {:?}", path);
    let mut store = AtomicStorage::open(path).expect("Could not open atomic store");

    let block = VDEntryBlock::default();
    let hash = block.hash();
    store
        .update(|mut m| {
            let block = block.clone();
            async move { m.insert_block(hash, block).await }
        })
        .await
        .unwrap();

    // Make sure the data is still there after re-opening
    drop(store);
    store = AtomicStorage::open(path).expect("Could not open atomic store");
    assert_eq!(
        store.get_block(&hash).await.unwrap(),
        Some(DEntryBlock::default())
    );

    // Add some transactions
    let mut rng = thread_rng();
    let state = <DemoState as TestableState<H_256>>::get_starting_state();
    let mut hashes = Vec::new();
    let mut block = block;
    for _ in 0..10 {
        let new = block
            .add_transaction_raw(&random_transaction(&state, &mut rng))
            .expect("Could not add transaction");
        println!("Inserting {:?}: {:?}", new.hash(), new);
        store
            .update(|mut m| {
                let new = new.clone();
                async move { m.insert_block(new.hash(), new.clone()).await }
            })
            .await
            .unwrap();
        hashes.push(new.hash());
        block = new;
    }

    // read them all back 3 times
    // 1st time: normal readback
    // 2nd: after dropping and re-opening the store
    for i in 0..3 {
        if i == 1 {
            drop(store);
            store = AtomicStorage::open(path).expect("Could not open atomic store");
        }

        // read them all back
        for (idx, hash) in hashes.iter().enumerate() {
            match store.get_block(hash).await.expect("Could not read hash") {
                Some(block) => println!("read {:?}", block),
                None => panic!("Could not read hash {} {:?}", idx, hash),
            }
        }
    }
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_happy_path_qcs() {
    // This folder will be destroyed when the last handle to it closes
    let file = tempfile::tempdir().expect("Could not create temp dir");
    let path = file.path();
    println!("Using store in {:?}", path);
    let mut store = AtomicStorage::open(path).expect("Could not open atomic store");

    // Add some certificates
    let mut certs = Vec::<QuorumCertificate<H_256>>::new();
    for i in 0..10 {
        let cert = QuorumCertificate {
            view_number: ViewNumber::new(i),
            ..random_quorom_certificate()
        };
        println!("Inserting {:?}", cert);
        store
            .update(|mut m| {
                let cert = cert.clone();
                async move { m.insert_qc(cert).await }
            })
            .await
            .unwrap();
        certs.push(cert);
    }

    // read them all back 3 times
    // 1st time: normal readback
    // 2nd: after dropping and re-opening the store
    for i in 0..2 {
        if i == 1 {
            drop(store);
            store = AtomicStorage::open(path).expect("Could not open atomic store");
        }

        for cert in &certs {
            match store
                .get_qc_for_view(cert.view_number)
                .await
                .expect("Could not read view_number")
            {
                Some(c) => {
                    println!("read {:?}", c);
                    assert_eq!(&c, cert);
                }
                None => panic!("Could not read {:?}: {:?}", cert.view_number, cert),
            }
            match store
                .get_qc(&cert.block_hash)
                .await
                .expect("Could not read qc by hash")
            {
                Some(c) => {
                    println!("read {:?}", c);
                    assert_eq!(&c, cert);
                }
                None => panic!(
                    "Could not read block_hash {:?}: {:?}",
                    cert.block_hash, cert
                ),
            }
        }
    }
}

#[cfg_attr(
    async_executor_impl = "tokio",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(async_executor_impl = "async-std", async_std::test)]
async fn test_happy_path_leaves() {
    // This folder will be destroyed when the last handle to it closes
    let file = tempfile::tempdir().expect("Could not create temp dir");
    let path = file.path();
    println!("Using store in {:?}", path);
    let mut store = AtomicStorage::open(path).expect("Could not open atomic store");

    // Add some leaves
    let mut leaves = Vec::<Leaf<DEntryBlock, State, H_256>>::new();
    for _ in 0..10 {
        let leaf = random_validating_leaf(DEntryBlock {
            previous_block: StateHash::random(),
            ..Default::default()
        });
        println!("Inserting {:?}", leaf);
        store
            .update(|mut m| {
                let leaf = leaf.clone();
                async move { m.insert_leaf(leaf).await }
            })
            .await
            .unwrap();
        leaves.push(leaf);
    }

    // read them all back 2 times
    // 1st time: normal readback
    // 2nd: after dropping and re-opening the store
    for i in 0..2 {
        if i == 1 {
            drop(store);
            store = AtomicStorage::open(path).expect("Could not open atomic store");
        }

        for leaf in &leaves {
            match store
                .get_leaf(&leaf.hash())
                .await
                .expect("Could not read leaf hash")
            {
                Some(l) => {
                    println!("read {:?}", l);
                    assert_eq!(&l, leaf);
                }
                None => {
                    panic!("Could not read leaf hash {:?}: {:?}", leaf.hash(), leaf)
                }
            }
            let hash = BlockContents::hash(&leaf.deltas);
            match store
                .get_leaf_by_block(&hash)
                .await
                .expect("Could not read leaf by block")
            {
                Some(l) => {
                    println!("read {:?}", l);
                    assert_eq!(&l, leaf);
                }
                None => panic!("Could not read leaf hash {:?}: {:?}", hash, leaf),
            }
        }
    }
}
