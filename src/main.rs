use std::io;
fn main() {
    struct Node {
        value: i32,
        parent: usize,
        child1: usize,
        child2: usize,
    }
    let mut nodes = vec![
        // Node {
        //     value: 6,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 3,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 4,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 8,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 7,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 1,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 2,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
        // Node {
        //     value: 10,
        //     parent: 0,
        //     child1: 0,
        //     child2: 0,
        // },
    ];
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter an integer.");
            return;
        }
    };
    for i in 0..number {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number2: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                return;
            }
        };
        nodes.push(Node {
            value: number2,
            parent: 0,
            child1: 0,
            child2: 0,
        })
    }
    for i in 0..nodes.len() {
        if i % 2 == 1 {
            nodes[i].parent = (i + 1) / 2;
            // let child_start = 2i + 1
        } else {
            nodes[i].parent = i / 2
            // let child_start =
        }

        let child_start = 2 * i + 1;
        if nodes.len() >= child_start + 2 {
            nodes[i].child1 = child_start;
            nodes[i].child2 = child_start + 1
        } else if nodes.len() == child_start + 1 {
            nodes[i].child1 = child_start;
            nodes[i].child2 = 0
        } else {
            nodes[i].child1 = 0;
            nodes[i].child2 = 0
        }
        if i % 2 == 1 {
            nodes[i].parent = (i + 1) / 2;
        } else {
            nodes[i].parent = i / 2
        }
    }
    fn hepify(mut vec_of_nodes: Vec<Node>, i: usize) -> Vec<Node> {
        let mut largest: usize = i;
        if vec_of_nodes[i].value < vec_of_nodes[vec_of_nodes[i].child1].value
            && vec_of_nodes[i].child1 != 0
        {
            largest = vec_of_nodes[i].child1
        }
        if vec_of_nodes[largest].value < vec_of_nodes[vec_of_nodes[i].child2].value
            && vec_of_nodes[i].child2 != 0
        {
            largest = vec_of_nodes[i].child2
        }
        if largest != i {
            let placeholder = vec_of_nodes[i].value;
            vec_of_nodes[i].value = vec_of_nodes[largest].value;
            vec_of_nodes[largest].value = placeholder;
            // vec_of_nodes[i].value, vec_of_nodes[largest].value = vec_of_nodes[largest].value, vec_of_nodes[i].value;
            vec_of_nodes = hepify(vec_of_nodes, largest)
        }
        return vec_of_nodes;
    }
    fn max_heap(mut vec_of_nodes: Vec<Node>) -> Vec<Node> {
        let last_subtree: f32 = (vec_of_nodes.len() / 2) as f32;
        let last_subtree_int = last_subtree.floor() as i32;
        for i in (0..last_subtree_int).rev() {
            let ii = i as usize;
            vec_of_nodes = hepify(vec_of_nodes, ii)
        }
        return vec_of_nodes;
    }
    // for Node in &nodes {
    //     println!(
    //         "parant = {}, child1 = {}, child 2 = {}, value = {}",
    //         Node.parent, Node.child1, Node.child2, Node.value
    //     );
    // }
    let new_nodes = max_heap(nodes);

    for Node in &new_nodes {
        println!(
            "parant = {}, child1 = {}, child 2 = {}, value = {}",
            Node.parent, Node.child1, Node.child2, Node.value
        );
    }
}
