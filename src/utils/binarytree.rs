use smithay::desktop::Window;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};
use super::workspace::CompWindow;

#[derive(Clone)]
pub enum BinaryTree {
    Empty,
    Window(Rc<RefCell<CompWindow>>),
    Split {
        split: HorizontalOrVertical,
        ratio: f32,
        counter_ratio: f32,
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HorizontalOrVertical {
    Horizontal,
    Vertical,
}
impl HorizontalOrVertical {
    pub fn reverse(&self) -> Self {
        match self {
            HorizontalOrVertical::Horizontal => HorizontalOrVertical::Vertical,
            HorizontalOrVertical::Vertical => HorizontalOrVertical::Horizontal,
        }
    }
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    pub fn insert(
        &mut self,
        window: Rc<RefCell<CompWindow>>,
        splitnew: HorizontalOrVertical,
        rationew: f32,
    ) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Window(window);
            }
            BinaryTree::Window(w) => {
                let counter_rationew = 1.0f32 - rationew;
                *self = BinaryTree::Split {
                    left: Box::new(BinaryTree::Window(w.clone())),
                    right: Box::new(BinaryTree::Window(window)),
                    split: splitnew,
                    ratio: rationew,
                    counter_ratio: counter_rationew,
                };
            }
            BinaryTree::Split {
                left: _,
                right,
                split: _,
                ratio: _,
                counter_ratio: _,
            } => {
                right.insert(window, splitnew, rationew);
            }
        }
    }

    pub fn remove(&mut self, window: &Window) {
        match self {
            BinaryTree::Empty => {}
            BinaryTree::Window(w) => {
                // Should only happen if this is the root
                if w.borrow().window == *window {
                    *self = BinaryTree::Empty;
                }
            }
            BinaryTree::Split {
                left,
                right,
                split: _, 
                ratio: _,
                counter_ratio: _,
            } => {
                if let BinaryTree::Window(w) = left.as_ref() {
                    if w.borrow().window == *window {
                        *self = *right.clone();
                        return;
                        
                    }
                }
                if let BinaryTree::Window(w) = right.as_ref() {
                    if w.borrow().window == *window {
                        *self = *left.clone();
                        return;
                    }
                }
                left.remove(window);
                right.remove(window);
            }
        }
    }

    pub fn update_after_removal(
        &mut self,
        splitter: HorizontalOrVertical
    ) {
        match self {
            BinaryTree::Split { split, right , .. } => {
                *split = splitter;
                right.update_after_removal(splitter.reverse());
            },
            BinaryTree::Empty => {},
            BinaryTree::Window(_) => {},
        }
    }

    pub fn next_split(&self) -> HorizontalOrVertical {
        match self {
            BinaryTree::Empty => HorizontalOrVertical::Horizontal,
            BinaryTree::Window(_w) => HorizontalOrVertical::Horizontal,
            BinaryTree::Split {
                left: _,
                right,
                split,
                ratio: _,
                counter_ratio: _,
            } => {
                if let BinaryTree::Split {
                    left: _,
                    right: _,
                    split: _,
                    ratio: _,
                    counter_ratio: _,
                } = right.as_ref()
                {
                    right.next_split()
                } else if *split == HorizontalOrVertical::Horizontal {
                    HorizontalOrVertical::Vertical
                } else {
                    HorizontalOrVertical::Horizontal
                }
            }
        }
    }

    
    // If increment is set to `None`, it will completely change the ratio to `new-ratio`
    // If increment is `Some` and set to true, the ratio will be incremented by the `update_interval`
    // If increment is `Some` and set to false, the ratio will be decremented by the `update_interval`
    pub fn update_ratio(&mut self, update_interval: f32, increment: Option<bool>) {
        match self {
            BinaryTree::Empty => {}
            BinaryTree::Window(_) => {}
            BinaryTree::Split {
                split: _,
                ratio,
                counter_ratio,
                left: _,
                right: _,
            } => {
                match increment {
                    Some(increment) => {
                        if increment {
                            *ratio = f32::min(*ratio + update_interval, 0.7);
                        } else {
                            *ratio = f32::max(*ratio - update_interval, 0.3);
                        }
                    }
                    None => {
                        *ratio = 0.5;
                    }
                }
                *counter_ratio = 1.0f32 - *ratio;
            }
        }
    }
}

