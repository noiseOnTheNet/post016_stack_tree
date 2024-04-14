#![no_std]

use bstack::BStackError;

mod bstack;

// struct STree2<T>{
//     nodes: [Option<T>;4]
// }

// struct STree3<T>{
//     nodes: [Option<T>;8]
// }

// struct STree4<T>{
//     nodes: [Option<T>;16]
// }

// struct STree5<T>{
//     nodes: [Option<T>;32]
// }

// struct STree6<T>{
//     nodes: [Option<T>;64]
// }

// struct STree7<T>{
//     nodes: [Option<T>;128]
// }

struct STree8<T>{
    nodes: [Option<T>;256]
}

impl<T : Copy> STree8<T>{
    fn new() -> STree8<T>{
      STree8{
          nodes: [None; 256]
      }
    }
    fn depth(& self) -> usize{
        todo!("implement with .log2().floor() of the latest busy node")
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

#[derive(Debug, Clone, Copy)]
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

struct TreeStackFrame{
    address: Address,
    cell: usize
}

struct STree8Iter<'a, T>{
    tree: & 'a STree8<T>,
    stack: bstack::BStack,
    addresses: [Option<Address>; 256]
}

impl<'a, T : Copy> STree8Iter<'a, T>{
    pub fn new(tree : & 'a STree8<T>) -> STree8Iter<'a, T>{
        STree8Iter::<'a, T>{
            tree,
            stack: bstack::BStack::new(),
            addresses: [None; 256]
        }
    }

    fn push_branch(& mut self, branch: Branch, address: Address) -> Result<usize, TreeError>{
        let _ = self.stack.push(branch == Branch::Left)?;
        let cell = self.stack.get_state();
        self.addresses[self.stack.get_state()] = Some(address);
        Ok(cell)
    }

    fn push_cell(& mut self, cell: usize, address: Address) -> Result<usize,TreeError>{
        todo!("complete implementation");
    }

    fn pop(& mut self) -> Result<(usize, Address), TreeError> {
        let cell = self.stack.get_state();
        let _branch = self.stack.pop()?;
        let address = self.addresses[cell].ok_or(TreeError::MissingReturnAddress(cell))?;
        Ok((cell, address))
    }

    pub fn next_item(& mut self) -> Result<& 'a T, TreeError>{
        while self.stack.size() > 0{
            let (cell, address) = self.pop()?;
            match address{
                Address::Enter => {
                    let left_address = cell << 1;
                    match self.tree.peek(left_address)?{
                        None => {
                            self.push_cell(cell, Address::AfterLeft);
                        }
                        Some(_) =>{
                            self.push_cell(cell, Address::AfterLeft);
                            self.push_branch(Branch::Left, Address::Enter);
                        }
                    }
                },
                Address::AfterLeft => {
                    self.push_cell(cell, Address::ValueYielded);
                    if let Some(result) = self.tree.peek(cell)?{
                        //return Ok(&result);
                        todo!("complete implementation")
                    }else{
                        return Err(TreeError::IteratorCompleted)
                    }

                },
                Address::ValueYielded => {
                    let right_address = (cell << 1) | 1;
                    match self.tree.peek(right_address)?{
                        None => {
                            self.push_cell(cell, Address::Completed);
                        },
                        Some(_) => {
                            self.push_cell(cell, Address::Completed);
                            self.push_branch(Branch::Right, Address::Enter);
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

impl<T : Copy> Tree<T> for STree8<T>{
    fn deep_first_level<S : Iterator>(& self) -> S {
        todo!("complete implementation")
    }
    fn deep_first<S : Iterator>(& self) -> S {
        let iter = STree8Iter::new(self);
        todo!("complete implementation")
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
        for value in [4,5,2,8,6,1]{
            let result = tree.insert(value);
            match result {
                Err(message) => {
                    panic!("failed insertion {}",message);
                },
                Ok(node) => {
                    //println!("{} inserted in node {}", value, node);
                }
            }
        }
    }
}
