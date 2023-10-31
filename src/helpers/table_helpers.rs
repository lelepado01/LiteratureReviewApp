
use dioxus::prelude::*;

pub fn handle_table_show_modal_hook(index : usize, index_hook : &UseState<std::option::Option<usize>>) -> &UseState<std::option::Option<usize>> {
    if let Some(picker) = index_hook.get() {
        if *picker == index {
            index_hook.set(None);
        } else {
            index_hook.set(Some(index));
        }
    } else {
        index_hook.set(Some(index));
    }

    index_hook
}

pub fn table_show_modal_hook_is_visible(index : usize, index_hook : &UseState<std::option::Option<usize>>) -> bool {
    if let Some(picker) = index_hook.get() {
        return  *picker == index 
    }

    false
}
