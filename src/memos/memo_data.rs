
use std::fmt::Debug;

use dioxus::prelude::*;
use serde::{Serialize, Deserialize};

use crate::data::loader::load_memos;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Memo {
    pub content: String,
    pub open: bool,
    pub done: bool,
    pub children: Vec<Memo>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct MemoData<'a> {
    pub search_query : &'a UseState<String>,
    pub add_memo_modal_form_hidden : &'a UseState<bool>,
    pub modal_form_memo : &'a UseState<Memo>,
    pub memo_parent : &'a UseState<String>,
    pub all_memos : &'a UseRef<Vec<Memo>>,
}

impl<'a> MemoData<'a> {
    pub fn new(cx: Scope<'a>) -> Self {
        Self {
            search_query: use_state(cx, || "".to_string()),
            add_memo_modal_form_hidden: use_state(cx, || true),
            modal_form_memo: use_state(cx, || Memo { content: "".to_string(), open:false, done: false, children: vec![] }),
            all_memos: use_ref(cx, ||{load_memos()}),
            memo_parent: use_state(cx, || "".to_string()),
        }
    }
}

impl<'a> MemoData<'a> {

    pub fn add_memo(&self, memo : Memo) {
        self.all_memos.with_mut(|memos| memos.push(memo));
    }

    pub fn add_memo_to_memo(&self, memo_parent : String, memo : Memo) {
        self.all_memos.with_mut(|memos| {
            for m in memos.iter_mut() {
                m.add_memo(memo_parent.clone(), memo.clone());
            }
        });
    }

    pub fn toggle_memo(&self, memo_content : String) {
        self.all_memos.with_mut(|memos| {
            for memo in memos.iter_mut() {
                if memo.content == memo_content {
                    memo.open = !memo.open;
                    return;
                }
                memo.toggle_memo(memo_content.clone());
            }
        });
    }

    pub fn check_memo(&self, memo_content : String){

        self.all_memos.with_mut(|memos| {
            for memo in memos.iter_mut() {
                if memo.content == memo_content {
                    memo.done = !memo.done;
                    return; 
                }
                memo.check_memo(memo_content.clone());
            }
        });
    }

    pub fn remove_memo(&self, memo_content : String) {
        self.all_memos.with_mut(|memos| {
            for memo in memos.iter_mut() {
                if memo.content == memo_content {
                    memos.retain(|memo| memo.content != memo_content);
                    return;
                }
                memo.remove_memo(memo_content.clone());
            }
        });
    }
}

impl Memo {
    pub fn add_memo(&mut self, memo_parent : String, memo : Memo) {
        if self.content == memo_parent {
            self.children.push(memo);
        } else {
            for m in self.children.iter_mut() {
                m.add_memo(memo_parent.clone(), memo.clone());
            }
        }
    }

    pub fn toggle_memo(&mut self, memo_content : String) {
        for memo in self.children.iter_mut() {
            if memo.content == memo_content {
                memo.open = !memo.open;
            } else {
                memo.toggle_memo(memo_content.clone());
            }
        }
    }

    pub fn check_memo(&mut self, memo_content : String) {
        for memo in self.children.iter_mut() {
            if memo.content == memo_content {
                memo.done = !memo.done;
            }
        }
    }   

    pub fn remove_memo(&mut self, memo_content : String) {
        self.children.retain(|memo| memo.content != memo_content);
    }
}