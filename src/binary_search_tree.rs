use std::{borrow::Borrow, cmp::Ordering};

type Son<T> = Box<BinarySearchTree<T>>;

/// # Binary Search Tree
///
/// A (mostly) recursive, chained implementation of BSTs.
///
/// See [this page on Wikipedia](https://en.wikipedia.org/wiki/Binary_search_tree) for further details.
///
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    value: T,
    left: Option<Son<T>>,
    right: Option<Son<T>>,
}

// TODO adjust implementation to remove the `Copy` bound
impl<T: Ord + Eq + Copy> BinarySearchTree<T> {
    /// Creates a new tree with `value` as root.
    pub fn new(value: T) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }

    /// Like `new`, but inside of a [Box].
    pub fn boxed(value: T) -> Box<Self> {
        Box::new(Self::new(value))
    }

    /// Inserts a new element in the tree. It does nothing if the element is already there.
    pub fn insert(&mut self, new: T) {
        match new.cmp(&self.value) {
            Ordering::Equal => (),
            Ordering::Less => match self.left {
                Some(ref mut tree) => tree.insert(new),
                None => self.left = Some(Self::boxed(new)),
            },
            Ordering::Greater => match self.right {
                Some(ref mut tree) => tree.insert(new),
                None => self.right = Some(Self::boxed(new)),
            },
        }
    }

    /// Goes through the tree looking for `value`. If found, it returns its tree wrapped in `Some`. Otherwise, it returns `None`.
    pub fn find<G: Borrow<T>>(&self, value: G) -> Option<&Self> {
        match value.borrow().cmp(&self.value) {
            Ordering::Equal => Some(&self),
            Ordering::Less => self.left.as_ref().and_then(|tree| tree.find(value)),
            Ordering::Greater => self.right.as_ref().and_then(|tree| tree.find(value)),
        }
    }

    /// Removes `value` from the tree.
    ///
    /// The return value is:
    ///
    /// * `Some(_)` if the value removed is not the last element in the tree.
    /// * `None` if the tree got empty.
    ///
    pub fn remove<G: Borrow<T>>(mut self, value: G) -> Option<Self> {
        match value.borrow().cmp(&self.value) {
            Ordering::Equal => match (self.left.is_some(), self.right.is_some()) {
                (false, false) => return None,
                (true, true) => {
                    let biggest_from_left = {
                        let mut current = self.left.as_ref().unwrap();
                        while let Some(ref right) = current.right {
                            current = right;
                        }
                        current.value
                    };
                    self.left = replace(self.left, |tree| tree.remove(biggest_from_left));
                    self.value = biggest_from_left;
                }
                _ => {
                    if self.left.is_some() {
                        return self.left.map(|it| *it);
                    } else {
                        return self.right.map(|it| *it);
                    }
                }
            },
            Ordering::Less => self.left = replace(self.left, |tree| tree.remove(value)),
            Ordering::Greater => self.right = replace(self.right, |tree| tree.remove(value)),
        }
        Some(self)
    }
}

fn replace<T>(
    son: Option<Son<T>>,
    change: impl FnOnce(BinarySearchTree<T>) -> Option<BinarySearchTree<T>>,
) -> Option<Son<T>> {
    let mut container = son?;
    let new = change(*container)?;
    *container = new;
    Some(container)
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn create() {
        let tree = BinarySearchTree::new(5);

        assert_eq!(tree.value, 5);
        assert!(tree.left.is_none());
        assert!(tree.right.is_none());

        let boxed = BinarySearchTree::boxed(10);

        assert_eq!(boxed.value, 10);
        assert!(boxed.left.is_none());
        assert!(boxed.right.is_none());
    }

    #[test]
    fn insert() {
        let mut tree = BinarySearchTree::new(20);
        tree.insert(10);

        assert!(tree.left.is_some());
        assert!(tree.right.is_none());

        assert!(tree.left.as_ref().unwrap().left.is_none());

        tree.insert(2);
        assert!(tree.left.as_ref().unwrap().left.is_some());

        tree.insert(15);
        assert!(tree.left.as_ref().unwrap().right.is_some());
        assert!(tree.right.is_none());
    }
}
