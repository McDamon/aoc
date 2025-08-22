// https://adventofcode.com/2022/day/7

use std::sync::atomic::{AtomicUsize, Ordering};

use regex::Regex;

use crate::utils::get_lines;

#[derive(Clone, Debug, PartialEq)]
enum FileKind {
    Directory,
    File,
}

#[derive(Clone, Debug)]
pub(crate) struct FileNode {
    id: usize,
    kind: FileKind,
    name: String,
    size: Option<usize>,
}

#[derive(Debug)]
pub(crate) struct DirectoryTreeNode {
    value: FileNode,
    children: Vec<DirectoryTreeNode>,
    parent: Option<FileNode>,
}

pub(crate) fn get_counter() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

impl DirectoryTreeNode {
    pub fn new_dir(name: String, parent: Option<FileNode>) -> DirectoryTreeNode {
        return DirectoryTreeNode {
            value: FileNode {
                id: get_counter(),
                kind: FileKind::Directory,
                name: name,
                size: None,
            },
            children: vec![],
            parent,
        };
    }

    pub fn new_file(name: String, size: usize, parent: Option<FileNode>) -> DirectoryTreeNode {
        return DirectoryTreeNode {
            value: FileNode {
                id: get_counter(),
                kind: FileKind::File,
                name,
                size: Some(size),
            },
            children: vec![],
            parent,
        };
    }

    pub fn add_child(&mut self, child_node: DirectoryTreeNode) {
        for child in &self.children {
            if child.value.id == child_node.value.id {
                return;
            }
        }
        self.children.push(child_node);
    }

    pub fn get_sum_of_directories(&mut self) -> i32 {
        let mut sum: i32 = 0;
        self.dfs(&mut |node| -> () {
            if node.value.kind == FileKind::Directory {
                let size = node.get_size_of_directory();
                if size <= 100000 {
                    sum += size;
                }
            }
        });
        sum
    }

    pub fn get_size_of_directory(&mut self) -> i32 {
        let mut sum: i32 = 0;
        self.dfs(&mut |node| -> () {
            if node.value.kind == FileKind::File {
                sum += node.value.size.unwrap() as i32;
            }
        });
        sum
    }

    fn get_all_directory_sizes(&mut self) -> Vec<i32> {
        let mut sizes: Vec<i32> = vec![];
        self.dfs(&mut |node| -> () {
            if node.value.kind == FileKind::Directory {
                sizes.push(node.get_size_of_directory());
            }
        });
        sizes
    }

    pub fn dfs<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut DirectoryTreeNode),
    {
        self.dfs_helper(f);
    }

    fn dfs_helper<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut DirectoryTreeNode),
    {
        (f)(self);
        self.children.iter_mut().for_each(|child| {
            child.dfs_helper(f);
        });
    }
}

pub(crate) fn parse_input(input_file: &str) -> DirectoryTreeNode {
    lazy_static! {
        static ref RE_CD: Regex = Regex::new(r"\$ cd (?P<path>.+)").unwrap();
        static ref RE_LS: Regex = Regex::new(r"\$ (?P<command>ls)").unwrap();
        static ref RE_DIR: Regex = Regex::new(r"dir (?P<dir>.+)").unwrap();
        static ref RE_FILE: Regex = Regex::new(r"(?P<size>\d+) (?P<file>.+)").unwrap();
    }

    let mut root = DirectoryTreeNode::new_dir("/".to_string(), None);

    let mut current = root.value.clone();

    let lines = get_lines(input_file);

    for line in lines {
        if let Some(caps_cd) = RE_CD.captures(&line) {
            let path = &caps_cd["path"];

            match path {
                // Reset our current node to root
                "/" => {
                    current = root.value.clone();
                }
                // Set our current node to current parent
                ".." => {
                    root.dfs(&mut |node| -> () {
                        if node.value.id == current.id {
                            if let Some(parent) = &node.parent {
                                current = parent.clone();
                            } else {
                                println!("Node {0} has no parent", node.value.name);
                            }
                        }
                    });
                }
                // Otherwise we set our current node to the new directory
                _ => {
                    root.dfs(&mut |node| -> () {
                        if let Some(parent) = &node.parent {
                            if node.value.name == path && parent.id == current.id {
                                current = node.value.clone();
                            }
                        }
                    });
                }
            }
        }
        if let Some(_caps_ls) = RE_LS.captures(&line) {
            // Drop this
        }
        if let Some(caps_dir) = RE_DIR.captures(&line) {
            let dir = &caps_dir["dir"];

            root.dfs(&mut |node| -> () {
                if node.value.id == current.id {
                    node.add_child(DirectoryTreeNode::new_dir(
                        dir.to_string(),
                        Some(current.clone()),
                    ));
                }
            });
        }
        if let Some(caps_file) = RE_FILE.captures(&line) {
            let size: usize = caps_file["size"].parse().unwrap();
            let file = &caps_file["file"];

            root.dfs(&mut |node| -> () {
                if node.value.id == current.id {
                    node.add_child(DirectoryTreeNode::new_file(
                        file.to_string(),
                        size,
                        Some(current.clone()),
                    ));
                }
            });
        }
    }
    root
}

pub fn get_sum_of_directories(input_file: &str) -> i32 {
    let mut root = parse_input(input_file);
    let sum = root.get_sum_of_directories();
    sum
}

pub fn get_all_directory_sizes(input_file: &str) -> Vec<i32> {
    let mut root = parse_input(input_file);
    let sizes = root.get_all_directory_sizes();
    sizes
}

pub fn get_size_deleted_directory(input_file: &str) -> i32 {
    let sizes = get_all_directory_sizes(input_file);
    let root_size = sizes.iter().max().unwrap();
    let unused_space = 70000000 - root_size;
    let required_space = 30000000 - unused_space;
    println!(
        "Root size: {0}, unused space: {1}, required space {2}",
        root_size, unused_space, required_space
    );
    let potential_sizes: Vec<i32> = sizes.into_iter().filter(|x| x >= &required_space).collect();
    let chosen_size = potential_sizes.iter().min().unwrap();
    *chosen_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_of_directories_test01() {
        assert_eq!(95437, get_sum_of_directories("input/2022/day07_test01.txt"));
    }

    #[test]
    fn test_get_sum_of_directories_test02() {
        assert_eq!(773, get_sum_of_directories("input/2022/day07_test02.txt"));
    }

    #[test]
    fn test_get_sum_of_directories_test03() {
        assert_eq!(99999, get_sum_of_directories("input/2022/day07_test03.txt"));
    }

    #[test]
    fn test_get_sum_of_directories() {
        assert_eq!(1667443, get_sum_of_directories("input/2022/day07.txt"));
    }

    #[test]
    fn test_get_all_directory_sizes_test01() {
        assert_eq!(
            24933642,
            get_size_deleted_directory("input/2022/day07_test01.txt")
        );
    }

    #[test]
    fn test_get_all_directory_sizes() {
        assert_eq!(
            8998590,
            get_size_deleted_directory("input/2022/day07.txt")
        );
    }
}
