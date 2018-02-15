use std::collections::*;

fn init_map() -> Vec<Vec<usize>> {
    let map = vec![
        vec![0, 1, 1, 0, 0, 0, 0],
        vec![1, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0]
    ];
    map
}

fn bfs(start: usize, goal: usize, m: Vec<Vec<usize>>) {
    let mut que = VecDeque::new();
    let v = vec![start];
    let node_num = m.len();
    que.push_back(v);
    while !que.is_empty() {
        let current_path = match que.pop_front() {
            Some(p) => p,
            None => break
        };
        for i in 0..node_num {
            match current_path.iter().find(|x| **x == i) {
                Some(p) => {
                    continue
                },
                None => {
                    let current_node = match &current_path.last() {
                        &Some(l) => l,
                        &None => continue
                    };
                    if m[*current_node][i] > 0 {
                        let mut new_path = current_path.clone();
                        new_path.push(i);
                        if i == goal {
                            println!("dfs goal {:?}", new_path);
                            break;
                        }
                        que.push_back(new_path);
                    }
                }
            }
        }
    }
}

fn reachable<T>(from: &T, to: &T) -> bool {
    true
}

fn reachable_usize(cn: &usize,next: &usize,map: &Vec<Vec<usize>>) -> bool {
    map[*cn][*next] > 0
}

/* 幅優先探索のジェネリクス版 */
fn bfs_gen<T: Ord + Clone,M,F>(start: T, goal: T, nodes: &Vec<T>, map: &M,reachable: F)
    where F: Fn(&T,&T,&M) -> bool,T: std::fmt::Debug {
    let mut que = VecDeque::new();
    let v = vec![start];
    que.push_back(v);
    while !que.is_empty() {
        let current_path = match que.pop_front() {
            Some(p) => p,
            None => break
        };
        for n in nodes.iter() {
            match current_path.iter().find( |x| **x == *n) {
                Some(p) => continue,
                None => {
                    let current_node = match &current_path.last() {
                        &Some(l) => l,
                        &None => continue
                    };
                    if reachable(current_node, n,map) {
                        let mut new_path = current_path.clone();
                        new_path.push((*n).clone());
                        if *n == goal {
                            println!("goal {:?}",new_path);
                            break;
                        }
                        que.push_back(new_path);
                    }
                }
            }
        }
    }
}


fn dfs(start: usize, goal: usize, current: usize, current_path: Vec<usize>, m: &Vec<Vec<usize>>) {
    let node_num = m.len();

    for i in 0..node_num {
        match current_path.iter().find(|x| **x == i) {
            Some(p) => {
                continue
            },
            None => {
                if m[current][i] > 0 {
                    let mut new_path = current_path.clone();
                    new_path.push(i);
                    if i == goal {
                        println!("bfs goal {:?}", new_path);
                        return;
                    } else {
                        dfs(start, goal, i, new_path, m)
                    }
                }
            }
        }
    }
}

/* 深さ探索のジェネリクス版 */
fn dfs_gen<T: Ord+Clone,M,F>(start: T, goal: T,nodes: &Vec<T>,current: T, current_path: Vec<T>, m: &M,reachable: &F)
    where F: Fn(&T,&T,&M) -> bool,T: std::fmt::Debug {

    for n in nodes.iter() {
        match current_path.iter().find(|x| **x == *n) {
            Some(p) => {
                continue
            },
            None => {
                if reachable(&current,n,m) {
                    let mut new_path = current_path.clone();
                    new_path.push((*n).clone());
                    if *n == goal {
                        println!("bfs goal {:?}", new_path);
                        return;
                    } else {
                        dfs_gen(start.clone(), goal.clone(), nodes,(*n).clone(), new_path, m,reachable)
                    }
                }
            }
        }
    }
}





fn main() {
   bfs_gen(0,6,&vec![1,2,3,4,5,6],&init_map(),reachable_usize);
   dfs_gen(0,6,&vec![1,2,3,4,5,6],0,vec![0],&init_map(),&reachable_usize);
}
