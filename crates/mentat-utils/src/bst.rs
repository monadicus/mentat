/// BST is an implementation of a binary search tree.
#[derive(Debug, Clone, Default)]
pub struct Bst {
    root: Option<Node>,
}

impl Bst {
    /// Empty returns a boolean indicating
    /// if there are any nodes in the BST.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Set stores the key and value in the BST
    pub fn set(&mut self, key: usize, value: usize) {
        if let Some(r) = &mut self.root {
            r.set(key, value);
        } else {
            self.root = Some(Node::new(key, value));
        }
    }

    /// Get gets the key in the BST and returns
    /// its representative node (so that modifications
    /// can be done in constant time).
    pub fn get(&self, key: usize) -> Option<&Node> {
        self.root.as_ref().and_then(|r| r.get(key))
    }

    /// Delete removes the key from the BST.
    pub fn delete(&mut self, key: usize) {
        self.root = self.root.take().and_then(|r| r.remove(key));
    }

    /// Min returns the smallest node in the BST.
    pub fn min(&self) -> Option<&Node> {
        self.root.as_ref().map(|r| r.find_min())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub key: usize,
    pub value: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: usize, value: usize) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }

    /// set adds the key and value to the BST
    /// if they key doesn't exist. If it does
    /// exist, it is overwritten.
    pub fn set(&mut self, key: usize, value: usize) {
        if self.key == key {
            self.value = value;
        } else if self.key > key {
            if let Some(l) = &mut self.left {
                l.set(key, value)
            } else {
                self.left = Some(Box::new(Node::new(key, value)))
            }
        } else if let Some(r) = &mut self.right {
            r.set(key, value)
        } else {
            self.right = Some(Box::new(Node::new(key, value)))
        }
    }

    /// get gets the *Node for a key in the BST,
    /// returning nil if it doesn't exist.
    pub fn get(&self, key: usize) -> Option<&Node> {
        match key {
            k if k == self.key => Some(self),
            k if k < self.key => self.left.as_ref().and_then(|n| n.get(k)),
            k => self.right.as_ref().and_then(|n| n.get(k)),
        }
    }

    /// remove deletes the *Node for a key in the BST
    /// and returns the new root.
    pub fn remove(mut self, key: usize) -> Option<Node> {
        if key < self.key {
            self.left = self.left.take().and_then(|n| n.remove(key)).map(Box::new);
            Some(self)
        } else if key > self.key {
            self.right = self.right.take().and_then(|n| n.remove(key)).map(Box::new);
            Some(self)
        } else if self.left.is_none() && self.right.is_none() {
            None
        } else if self.left.is_none() {
            self.right.map(|n| *n)
        } else if self.right.is_none() {
            self.left.map(|n| *n)
        } else {
            let smallest_key_on_right = self.right.as_ref().unwrap().find_min();
            self.key = smallest_key_on_right.key;
            self.right = self
                .right
                .take()
                .and_then(|n| n.remove(self.key))
                .map(Box::new);
            Some(self)
        }
    }

    /// findMin returns the smallest key in
    /// the BST
    pub fn find_min(&self) -> &Node {
        if let Some(l) = &self.left {
            l.find_min()
        } else {
            self
        }
    }
}
