use data_structures::BinarySearchTree;

/// This test is just for debugging/printing purposes.
#[test]
#[ignore]
fn debug() {
    let mut tree = BinarySearchTree::new(50);
    tree.insert(10);
    tree.insert(2);
    tree.insert(75);
    tree.insert(15);
    tree.insert(100);

    eprintln!("{:#?}", tree);
}

#[test]
fn tree_insertion() {
    let mut tree = BinarySearchTree::new(10);
    tree.insert(50);
    tree.insert(27);

    assert!(tree.find(27).is_some());
    assert!(tree.find(42).is_none());
}

#[test]
fn removal_works_for_leafs() {
    let mut tree = BinarySearchTree::new(30);
    tree.insert(10);
    assert!(tree.find(10).is_some());

    tree = tree.remove(10).unwrap();
    assert!(tree.find(10).is_none());

    assert!(tree.remove(30).is_none());
}

#[test]
fn removal_works_when_value_is_not_a_leaf() {
    let mut tree = BinarySearchTree::new(20);
    tree.insert(10);
    tree.insert(5);

    tree = tree.remove(10).unwrap();
    assert!(tree.find(5).is_some());
}
