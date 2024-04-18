#![no_std]

mod bstack;

struct STree8<T>{
    nodes: [Option<T>;256]
}

impl<T : Copy> STree8<T>{
    fn new() -> STree8<T>{
      STree8{
          nodes: [None; 256]
      }
    }
    fn depth(& self) -> u32{
        let mut result : usize = 0;
        for (i, value) in self.nodes.into_iter().enumerate(){
            if let Some(_) = value{
                if i > result{
                    result = i;
                }
            }
        }
        if result == 0 {
            return 0;
        }
        result.ilog2() + 1
    }
    fn peek(& self, cell: usize) -> Result<Option<T>,TreeError>{
        if cell >= 256{
            return Err(TreeError::TreeOverflowCell)
        }
        Ok(self.nodes[cell])
    }
}

trait Tree<T> {
    fn deep_first_level<S : Iterator>(& self) -> S;
    fn deep_first<S : Iterator>(& self) -> S;
}

trait SortTree<T : Ord>{
    fn insert(& mut self, value: T) -> Result<usize, &'static str>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Address{
    Enter,
    AfterLeft,
    ValueYielded,
    Completed
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Branch{
    Left,
    Right
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TreeError{
    MissingReturnAddress(usize),
    StackError(bstack::BStackError),
    IteratorCompleted,
    TreeOverflowCell
}

impl From<bstack::BStackError> for TreeError{
    fn from(value: bstack::BStackError) -> Self {
        TreeError::StackError(value)
    }
}

struct STree8Iter<'a, T>{
    tree: & 'a STree8<T>,
    stack: bstack::BStack,
    addresses: [Option<Address>; 256]
}

impl<'a, T : Copy> STree8Iter<'a, T>{
    pub fn new(tree : & 'a STree8<T>) -> STree8Iter<'a, T>{
        let mut iterator = STree8Iter::<'a, T>{
            tree,
            stack: bstack::BStack::new(),
            addresses: [None; 256]
        };
        if let Ok(Some(_)) = iterator.tree.peek(1) {
            // ignore errors as iterator is just created
            let _ = iterator.push_branch(Branch::Right, Address::Enter);
        }
        iterator
    }

    fn push_branch(& mut self, branch: Branch, address: Address) -> Result<usize, TreeError>{
        let _ = self.stack.push(branch == Branch::Right)?;
        let cell = self.stack.get_state();
        self.addresses[self.stack.get_state()] = Some(address);
        Ok(cell)
    }

    fn push_cell(& mut self, cell: usize, address: Address) -> Result<usize,TreeError>{
        let _ = self.stack.push(cell & 1 == 1)?;
        let cell = self.stack.get_state();
        self.addresses[self.stack.get_state()] = Some(address);
        Ok(cell)
    }

    fn pop(& mut self) -> Result<(usize, Address), TreeError> {
        let cell = self.stack.get_state();
        let _branch = self.stack.pop()?;
        let address = self.addresses[cell].ok_or(TreeError::MissingReturnAddress(cell))?;
        Ok((cell, address))
    }

    pub fn next_item(& mut self) -> Result<T, TreeError>{
        while self.stack.size() > 0{
            let (cell, address) = self.pop()?;
            match address{
                Address::Enter => {
                    let left_address = cell << 1;
                    match self.tree.peek(left_address)?{
                        None => {
                            self.push_cell(cell, Address::AfterLeft)?;
                        }
                        Some(_) =>{
                            self.push_cell(cell, Address::AfterLeft)?;
                            self.push_branch(Branch::Left, Address::Enter)?;
                        }
                    }
                },
                Address::AfterLeft => {
                    self.push_cell(cell, Address::ValueYielded)?;
                    if let Some(ref result) = self.tree.peek(cell)?{
                        return Ok(*result);
                    }else{
                        return Err(TreeError::IteratorCompleted)
                    }

                },
                Address::ValueYielded => {
                    let right_address = (cell << 1) | 1;
                    match self.tree.peek(right_address)?{
                        None => {
                            self.push_cell(cell, Address::Completed)?;
                        },
                        Some(_) => {
                            self.push_cell(cell, Address::Completed)?;
                            self.push_branch(Branch::Right, Address::Enter)?;
                        }
                    }
                },
                Address::Completed =>{

                }
            }
        }
        Err(TreeError::IteratorCompleted)
    }
}

impl<'a, T : Copy> Iterator for STree8Iter<'a, T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // WARNING
        // this implicitly discard any error
        self.next_item().ok()
    }
}

impl<'a , T : Copy> IntoIterator for & 'a STree8<T>{
     type Item = T;
     type IntoIter = STree8Iter<'a, T>;
     fn into_iter(self) -> Self::IntoIter {
         STree8Iter::new(self)
     }
}

impl<T : Ord> SortTree<T> for STree8<T>{
    fn insert(& mut self, value: T) -> Result<usize, &'static str>{
        let mut node : usize = 1;
        loop {
            if node > 255{
                return Err("level greater than 8")
            }
            match self.nodes[node]{
                None => {
                    self.nodes[node] = Some(value);
                    return Ok(node);
                }
                Some(ref node_value) => {
                    if value == *node_value{
                        return Ok(node);
                    }
                    node <<= 1;
                    if value > *node_value{
                        node += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn can_insert(){
        let mut tree : STree8<i64> = STree8::new();
        let test_list = [4,5,2,8,6,1];
        let mut count = 0;
        for value in test_list{
            let result = tree.insert(value);
            match result {
                Err(message) => {
                    panic!("failed insertion {}",message);
                },
                Ok(node) => {
                    assert!(node < 256);
                    count += 1;
                }
            }
        }
        assert_eq!(count,test_list.len());
        let result = tree.peek(1);
        assert_eq!(Ok(Some(4)),result);
        let result = tree.peek(2);
        assert_eq!(Ok(Some(2)),result);
        let result = tree.peek(3);
        assert_eq!(Ok(Some(5)),result);
    }

    #[test]
    fn test_depth(){
        let mut tree : STree8<i64> = STree8::new();
        assert_eq!(tree.depth(),0);
        let _ = tree.insert(4);
        assert_eq!(tree.depth(),1);
        let _ = tree.insert(5);
        assert_eq!(tree.depth(),2);
        let _ = tree.insert(2);
        assert_eq!(tree.depth(),2);
        let _ = tree.insert(8);
        assert_eq!(tree.depth(),3);
        let _ = tree.insert(6);
        assert_eq!(tree.depth(),4);
        let _ = tree.insert(1);
        assert_eq!(tree.depth(),4);
    }

    #[test]
    fn can_create_iterator(){
        let mut tree : STree8<i64> = STree8::new();
        let test_list = [4,5,2,8,6,1];
        for value in test_list{
            let _result = tree.insert(value);
        }
        let mut iterator = STree8Iter::new(& tree);
        assert_eq!(iterator.stack.size(),1);
        assert_eq!(iterator.pop(),Ok((1,Address::Enter)));
    }

    #[test]
    fn can_extract_with_next_item(){
        let mut tree : STree8<i64> = STree8::new();
        let test_list = [4,5,2,8,6,1];
        for value in test_list{
            let _result = tree.insert(value);
        }
        let mut iterator = STree8Iter::new(& tree);
        let mut result = iterator.next_item();
        assert_eq!(Ok(1),result);
        result = iterator.next_item();
        assert_eq!(Ok(2),result);
    }

    #[test]
    fn sort_works(){
        let mut tree : STree8<i64> = STree8::new();
        let test_list = [4,5,2,8,6,1];
        for value in test_list{
            let _result = tree.insert(value);
        }
        let expected = [1,2,4,5,6,8];
        let mut count = 0;
        for (i,v) in tree.into_iter().enumerate(){
            assert_eq!(v,expected[i]);
            count += 1;
        }
        assert_eq!(count,test_list.len());
    }
}
