mod bstack;

struct STree2<T>{
    nodes: [Option<T>;4]
}

struct STree3<T>{
    nodes: [Option<T>;8]
}

struct STree4<T>{
    nodes: [Option<T>;16]
}

struct STree5<T>{
    nodes: [Option<T>;32]
}

struct STree6<T>{
    nodes: [Option<T>;64]
}

struct STree7<T>{
    nodes: [Option<T>;128]
}

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
}

trait Tree<T> {
    fn deep_first_level<S : Iterator>(& self) -> S;
    fn deep_first<S : Iterator>(& self) -> S;
}

trait SortTree<T : Ord>{
    fn insert(& mut self, value: T) -> Result<usize, &'static str>;
}

struct STree8Iter{
    stack: bstack::BStack
}

impl<T> Tree<T> for STree8<T>{
    fn deep_first_level<S : Iterator>(& self) -> S {
        todo!("complete implementation")
    }
    fn deep_first<S : Iterator>(& self) -> S {
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
                    println!("{} inserted in node {}", value, node);
                }
            }
        }
    }
}
