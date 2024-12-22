use std::fmt::Debug;

struct BstNode<T> {
    data: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>
}

impl<T> BstNode<T>
where
    T: PartialOrd + Debug + Clone,
{
    fn new(data: T) -> Self {
        BstNode {
            data,
            left: None,
            right: None
        }
    }

    fn insert(&mut self, data: T) {
        if data < self.data {
            match &mut self.left {
                Some(left) => left.insert(data),
                None => self.left = Some(Box::new(Self::new(data)))
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(data),
                None => self.right = Some(Box::new(Self::new(data)))
            }
        }
    }

    fn min(&self) -> T {
        match &self.left {
            Some(left) => left.min(),
            None => self.data.clone()
        }
    }

    fn max(&self) -> T {
        match &self.right {
            Some(right) => right.max(),
            None => self.data.clone()
        }
    }

    fn preorder_traversal(&self) {
        self.show();
        if let Some(left) = &self.left {
            left.preorder_traversal();
        }
        if let Some(right) = &self.right {
            right.preorder_traversal();
        }
    }

    fn inorder_traversal(&self) {
        if let Some(left) = &self.left {
            left.inorder_traversal();
        }
        self.show();
        if let Some(right) = &self.right {
            right.inorder_traversal();
        }
    }

    fn postorder_traversal(&self) {
        if let Some(left) = &self.left {
            left.postorder_traversal();
        }
        if let Some(right) = &self.right {
            right.postorder_traversal();
        }
        self.show();
    }

    fn show(&self) {
        print!("{:?} ", self.data);
    }
}

pub struct Bst<T> {
    root: Option<BstNode<T>>
}

impl<T> Bst<T>
where
    T: PartialOrd + Debug + Clone,
{
    pub fn new() -> Self {
        Bst { root: None }
    }

    pub fn insert(&mut self, data: T) {
        if let Some(root) = &mut self.root {
            root.insert(data);
        } else {
            self.root = Some(BstNode::<T>::new(data))
        }
    }

    pub fn min(&self) -> Option<T> {
        if let Some(root) = &self.root {
            Some(root.min())
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<T> {
        if let Some(root) = &self.root {
            Some(root.max())
        } else {
            None
        }
    }

    pub fn preorder_traversal(&self) {
        if let Some(root) = &self.root {
            root.preorder_traversal();
            println!("");
        } else {
            println!("A Empty Tree")
        }
    }

    pub fn inorder_traversal(&self) {
        if let Some(root) = &self.root {
            root.inorder_traversal();
            println!("");
        } else {
            println!("A Empty Tree")
        }
    }

    pub fn postorder_traversal(&self) {
        if let Some(root) = &self.root {
            root.postorder_traversal();
            println!("");
        } else {
            println!("A Empty Tree")
        }
    }
}
