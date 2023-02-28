pub struct Node {
    pub val: i32,
    pub is_leaf: bool,
    pub top_left: Option<Box<Node>>,
    pub top_right: Option<Box<Node>>,
    pub bottom_left: Option<Box<Node>>,
    pub bottom_right: Option<Box<Node>>,

}

// Definition of a quadtree node
impl Node {
    pub fn new(val: i32, is_leaf: bool,
               top_left: Option<Box<Node>>,
               top_right: Option<Box<Node>>,
               bottom_left: Option<Box<Node>>,
               bottom_right: Option<Box<Node>>
    ) -> Self {
        Node {
            val,
            is_leaf,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}


