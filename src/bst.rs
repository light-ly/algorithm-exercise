struct BstNode<T> {
    data: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>
}

impl <T: PartialOrd> BstNode<T> {
    fn new(data: T) -> Self {
        BstNode { data, left: None, right: None }
    }

    fn insert(&mut self, data: T) {
        if data < self.data {
            match &mut self.left {
                Some(left) => left.insert(data),
                None => self.left = Some(Box::new(Self::new(data)))
            }
        } else {
            match &mut self.right {
                Some(left) => left.insert(data),
                None => self.left = Some(Box::new(Self::new(data)))
            }
        }
    }
}

pub struct Bst<T> {
    root: Option<BstNode<T>>
}

impl<T: PartialOrd> Bst<T> {
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
}
