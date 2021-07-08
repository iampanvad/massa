// Copyright (c) 2021 MASSA LABS <info@massa.net>

use super::tools::*;
use crate::start_storage;
use serial_test::serial;
use std::collections::HashMap;

#[tokio::test]
#[serial]
async fn test_add() {
    let cfg = get_test_config();
    let (command_sender, manager) = start_storage(cfg).unwrap();
    assert_eq!(0, command_sender.len().await.unwrap());
    let hash = get_test_block_id();
    let block = get_test_block();
    command_sender.add_block(hash, block).await.unwrap();
    assert!(command_sender.contains(hash).await.unwrap());
    assert_eq!(1, command_sender.len().await.unwrap());
    manager.stop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_find_operation() {
    let cfg = get_test_config();
    let (command_sender, manager) = start_storage(cfg).unwrap();
    assert_eq!(0, command_sender.len().await.unwrap());
    let (block, id, op) = get_block_with_op();
    command_sender.add_block(id, block).await.unwrap();
    let (out_idx, out_final) = command_sender
        .get_operations(vec![op].into_iter().collect())
        .await
        .unwrap()[&op]
        .in_blocks[&id];
    assert_eq!((out_idx, out_final), (0, true));
    let mut op2 = create_operation();
    op2.content.fee = 42;
    let id2 = op2.get_operation_id().unwrap();
    assert!(!command_sender
        .get_operations(vec![id2].into_iter().collect())
        .await
        .unwrap()
        .contains_key(&id2));
    manager.stop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_add_multiple() {
    let cfg = get_test_config();
    let (command_sender, manager) = start_storage(cfg).unwrap();
    let hash = get_test_block_id();
    let block = get_test_block();
    let mut map = HashMap::new();
    map.insert(hash, block);
    command_sender.add_block_batch(map).await.unwrap();
    assert!(command_sender.contains(hash).await.unwrap());
    manager.stop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_get() {
    // stderrlog::new()
    //     .verbosity(2)
    //     .timestamp(stderrlog::Timestamp::Millisecond)
    //     .init()
    //     .unwrap();
    let cfg = get_test_config();
    let (command_sender, manager) = start_storage(cfg).unwrap();
    assert_eq!(0, command_sender.len().await.unwrap());
    let hash = get_test_block_id();
    let block = get_test_block();
    command_sender.add_block(hash, block.clone()).await.unwrap();
    let retrieved = command_sender.get_block(hash).await.unwrap().unwrap();

    assert_eq!(
        retrieved.header.content.compute_hash().unwrap(),
        block.header.content.compute_hash().unwrap()
    );

    assert!(command_sender
        .get_block(get_another_test_block_id())
        .await
        .unwrap()
        .is_none());
    //command_sender.clear().await.unwrap();
    manager.stop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_contains() {
    let cfg = get_test_config();
    let (command_sender, manager) = start_storage(cfg).unwrap();
    //test in an empty db that the contains return false.
    assert!(!command_sender
        .contains(get_another_test_block_id())
        .await
        .unwrap());

    assert_eq!(0, command_sender.len().await.unwrap());
    let hash = get_test_block_id();
    let block = get_test_block();
    command_sender.add_block(hash, block.clone()).await.unwrap();

    //test the block is present in db
    assert!(command_sender.contains(hash).await.unwrap());

    //test that another block isn't present
    assert!(!command_sender
        .contains(get_another_test_block_id())
        .await
        .unwrap());

    manager.stop().await.unwrap();
}
