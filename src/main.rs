fn main() {
    println!("Hello world!");
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Node {
    value: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: usize) -> Node {
        Node { value: value, left: None, right: None }
    }

    fn insert(&mut self, n: Node, balance: bool) -> bool {
        let mut rebalance = false;
        if n.value > self.value {
            if self.get_balance() == -1 { rebalance = true; }
            if self.right.is_some() {
                let mut node_right = self.right.clone().unwrap();
                let b = node_right.insert(n, balance);
                self.right = Some(node_right);

                if b && rebalance && balance {
                    self.rebalance();
                }

                return b;
            } else {
                self.right = Some(Box::new(n));

                if rebalance && balance {
                    self.rebalance();
                }

                return true;
            }
        } else if n.value < self.value {
            if self.get_balance() == 1 { rebalance = true; }
            if self.left.is_some() {
                let mut node_left = self.left.clone().unwrap();
                let b = node_left.insert(n, balance);
                self.left = Some(node_left);

                if b && rebalance && balance{
                    self.rebalance();
                }

                return b;
            } else {
                self.left = Some(Box::new(n));

                if rebalance && balance {
                    self.rebalance();
                }

                return true;
            }
        } else {
            return false;
        }
    }

    fn rebalance(&mut self) -> () {
        let mut values = self.get_values();
        values.sort();
        let mut values = sort_vec_by_middle(values, 0);
        values.sort();
        self.right = None;
        self.left = None;
        self.value = values[0].1;
        for i in 1..values.len() {
            self.insert(Node::new(values[i].1), false);
        }
    }

    fn get_balance(&self) -> isize {
        let left = self.left_height();
        let right = self.right_height();

        let balance = left as isize - right as isize;
        balance
    }

    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height() + 1)
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height() + 1)
    }

    fn height(&self) -> usize {
        std::cmp::max(self.left_height(), self.right_height())
    }

    fn get_values(&self) -> Vec<usize> {
        let mut right_values: Vec<usize> = vec![];
        if (&self).right.is_some() {
            right_values = (&self).right.clone().unwrap().get_values();
        }
        let mut left_values: Vec<usize> = vec![];
        if (&self).left.is_some() {
            left_values = (&self).left.clone().unwrap().get_values();
        }
        left_values.append(&mut right_values);
        left_values.push(self.value);
        left_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_is_binary() {
        let mut n = Node::new(5);
        n.insert(Node::new(6), true);
        n.insert(Node::new(8), true);
        n.insert(Node::new(2), true);
        n.insert(Node::new(4), true);
        n.insert(Node::new(7), true);

        assert_eq!(n.value, 5);
    }

    #[test]
    fn tree_is_balanced() {
        let mut n = Node::new(5);
        n.insert(Node::new(6), true);
        n.insert(Node::new(8), true);

        let b = n.get_balance();

        assert_eq!(b, 0);
    }

    #[test]
    fn tree_is_really_balanced() {
        let mut n = Node::new(9);
        n.insert(Node::new(2), true);
        n.insert(Node::new(3), true);
        n.insert(Node::new(4), true);
        n.insert(Node::new(1), true);
        n.insert(Node::new(7), true);
        n.insert(Node::new(8), true);
        n.insert(Node::new(6), true);
        n.insert(Node::new(5), true);

        let b = n.get_balance();

        assert_eq!(b, 0);
    }

    #[test]
    fn tree_gets_values() {
        let mut n = Node::new(5);
        n.insert(Node::new(6), true);
        n.insert(Node::new(8), true);
        n.insert(Node::new(2), true);
        n.insert(Node::new(7), true);
        let vf = vec![2, 5, 6, 7, 8];

        let mut v = n.get_values();
        v.sort();

        assert_eq!(v, vf);
    }
}

fn sort_vec_by_middle(list: Vec<usize>, iter: usize) -> Vec<(usize, usize)> {
    if list.len() == 0 {
        return vec![];
    } else {
        let splits = list.split_at((list.len() + 1) / 2);
        let mut left = splits.0.into_iter().map(|x| *x).collect::<Vec<usize>>();
        let mut right = splits.1.into_iter().map(|x| *x).collect::<Vec<usize>>();
        let mut new = vec![(iter, left.pop().unwrap())];
        new.append(&mut sort_vec_by_middle(left, iter + 1));
        new.append(&mut sort_vec_by_middle(right, iter + 1));
        new.sort();
        new
    }
}