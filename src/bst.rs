use std::{cmp::Ordering, fmt::Debug};

#[derive(Clone, Debug)]
pub struct BstNode<T> {
    data: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
    count: u64,
    size: u64
}

impl<T> BstNode<T>
where
    T: Ord + Debug + Clone,
{
    fn new(data: T) -> Self {
        BstNode {
            data,
            left: None,
            right: None,
            count: 1,
            size: 1
        }
    }

    pub fn data(&self) -> T {
        self.data.clone()
    }

    fn insert(&mut self, data: T) {
        match data.cmp(&self.data) {
            Ordering::Less => {
                match &mut self.left {
                    Some(left) => left.insert(data),
                    None => self.left = Some(Box::new(Self::new(data)))
                }
            }
            Ordering::Greater => {
                match &mut self.right {
                    Some(right) => right.insert(data),
                    None => self.right = Some(Box::new(Self::new(data)))
                }
            }
            Ordering::Equal => self.count = self.count + 1
        }

        self.size = self.calc_size()
    }

    fn calc_size(&self) -> u64 {
        match (&self.left, &self.right) {
            (None, None) => self.count,
            (Some(sub), None) | (None, Some(sub)) => sub.size + self.count,
            (Some(left), Some(right)) => left.size + right.size + self.count
        }
    }

    fn remove(&mut self, data: T) -> Option<Box<BstNode<T>>> {
        match data.cmp(&self.data) {
            Ordering::Less => {
                match &mut self.left.take() {
                    Some(left) => {
                        self.left = left.remove(data);
                        self.size = self.calc_size();
                        Some(Box::new(self.clone()))
                    }
                    None => None
                }
            }
            Ordering::Greater => {
                match &mut self.right.take() {
                    Some(right) => {
                        self.right = right.remove(data);
                        self.size = self.calc_size();
                        Some(Box::new(self.clone()))
                    }
                    None => None
                }
            }
            Ordering::Equal => {
                if self.count > 1  {
                    self.count = self.count - 1;
                    self.size = self.calc_size();
                    Some(Box::new(self.clone()))
                } else {
                    match (self.left.take(), self.right.take()) {
                        (None, None) => None,
                        (Some(sub), None) | (None, Some(sub)) => Some(sub),
                        (Some(left), Some(mut right)) => {
                            *self = right.min();
                            self.left = Some(left);
                            self.right = right.remove(data);
                            self.size = self.calc_size();
                            Some(Box::new(self.clone()))
                        }
                    }
                }
            }
        }
    }

    fn search(&self, data: T) -> Option<Self> {
        match data.cmp(&self.data) {
            Ordering::Less => {
                match &self.left {
                    Some(left) => left.search(data),
                    None => None,
                }
            }
            Ordering::Greater => {
                match &self.right {
                    Some(right) => right.search(data),
                    None => None,
                }
            }
            Ordering::Equal => Some(self.clone())
        }
    }

    fn rank(&self, data: T) -> u64 {
        match data.cmp(&self.data) {
            Ordering::Less => {
                match &self.left {
                    Some(left) => left.rank(data),
                    None => 0
                }
            }
            Ordering::Greater => {
                match &self.right {
                    Some(right) => {
                        let left_size = match &self.left {
                            Some(left) => left.size,
                            None => 0
                        };
                        right.rank(data) + left_size + self.count
                    }
                    None => 0
                }
            }
            Ordering::Equal => match &self.left {
                Some(left) => left.size + 1,
                None => 0
            }
        }
    }

    fn get_with_rank(&self, k: u64) -> Option<Self> {
        match &self.left {
            Some(left) => {
                if left.size >= k {
                    left.get_with_rank(k)
                } else if left.size + self.count >= k {
                    Some(self.clone())
                } else {
                    match &self.right {
                        Some(right) => right.get_with_rank(k - left.size - self.count),
                        None => None
                    }
                }
            }
            None => {
                if k == 1 {
                    Some(self.clone())
                } else {
                    match &self.right {
                        Some(right) => right.get_with_rank(k - self.count),
                        None => None
                    }
                }
            }
        }
    }

    fn min(&self) -> Self {
        match &self.left {
            Some(left) => left.min(),
            None => self.clone()
        }
    }

    fn max(&self) -> Self {
        match &self.right {
            Some(right) => right.max(),
            None => self.clone()
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

#[derive(Clone, Debug)]
pub struct Bst<T> {
    root: Option<Box<BstNode<T>>>
}

impl<T> Bst<T>
where
    T: Ord + Debug + Clone,
{
    pub fn new() -> Self {
        Bst { root: None }
    }

    pub fn insert(&mut self, data: T) {
        if let Some(root) = &mut self.root {
            root.insert(data);
        } else {
            self.root = Some(Box::new(BstNode::<T>::new(data)))
        }
    }

    pub fn remove(&mut self, data: T) {
        if let Some(root) = &mut self.root {
           self.root = root.remove(data)
        }
    }

    pub fn search(&self, data: T) -> Option<BstNode<T>> {
        if let Some(root) = &self.root {
            root.search(data)
        } else {
            None
        }
    }

    pub fn rank(&self, data: T) -> u64 {
        if let Some(root) = &self.root {
            root.rank(data)
        } else {
            0
        }
    }

    pub fn get_with_rank(&self, k: u64) -> Option<BstNode<T>> {
        if let Some(root) = &self.root {
            root.get_with_rank(k)
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<BstNode<T>> {
        if let Some(root) = &self.root {
            Some(root.min())
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<BstNode<T>> {
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
