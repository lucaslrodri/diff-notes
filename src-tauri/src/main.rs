// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diffmatchpatch::prelude::{diff_semantic, diff_word_mode, Diff};
use std::sync::{Arc, Mutex};

// Highlights of texts differences

#[derive(serde::Deserialize, serde::Serialize)]
struct TextChunk {
    op: char,
    content: String,
    start: usize,
    end: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct TextsChangesHightlights {
    left: Vec<TextChunk>,
    right: Vec<TextChunk>,
    raw: Vec<(char, String)>,
}

enum Side {
    Left,
    Right,
}

#[tauri::command(async)]
fn get_highlights_of_text_changes(
    text_left: &str,
    text_right: &str,
    mode: char,
) -> TextsChangesHightlights {
    let diff = if mode == 'w' {
        diff_word_mode(text_left, text_right)
    } else {
        diff_semantic(text_left, text_right)
    };

    let diff_arr: Vec<(char, String)> = diff
        .into_iter()
        .map(|chunk| match chunk {
            Diff::Delete(value) => ('R', value),
            Diff::Insert(value) => ('A', value),
            Diff::Equal(value) => ('U', value),
        })
        .collect();

    let mut text1: Vec<TextChunk> = Vec::new();
    let mut text2: Vec<TextChunk> = Vec::new();

    let mut iter = diff_arr.iter().peekable();

    let mut cursor_text1: usize = 0;
    let mut cursor_text2: usize = 0;

    let mut push_text = |content: &String, side: Side, op: char| {
        let text_len: usize = if op != 'R' {
            content.chars().count()
        } else {
            0
        };
        match side {
            Side::Left => {
                text1.push(TextChunk {
                    op,
                    content: content.clone(),
                    start: cursor_text1,
                    end: cursor_text1 + text_len,
                });
                cursor_text1 += text_len;
            }
            Side::Right => {
                text2.push(TextChunk {
                    op,
                    content: content.clone(),
                    start: cursor_text2,
                    end: cursor_text2 + text_len,
                });
                cursor_text2 += text_len;
            }
        }
    };

    while let Some(&(op, ref content)) = iter.next() {
        match op {
            'U' => {
                push_text(content, Side::Left, 'U');
                push_text(content, Side::Right, 'U');
            }
            'R' => {
                if let Some(&(next_op, ref next_content)) = iter.peek() {
                    if *next_op == 'A' {
                        // RA
                        push_text(content, Side::Left, 'M');
                        push_text(next_content, Side::Right, 'M');
                        iter.next(); // Skip the next element as it's already processed
                    } else {
                        //only R
                        push_text(content, Side::Left, 'A');
                        push_text(&String::new(), Side::Right, 'R');
                    }
                } else {
                    // Last line: -
                    push_text(content, Side::Left, 'A');
                    push_text(&String::new(), Side::Right, 'R');
                }
            }
            'A' => {
                if let Some(&(next_op, ref next_content)) = iter.peek() {
                    if *next_op == 'R' {
                        //AR
                        // text2: A | text1: R
                        push_text(content, Side::Right, 'M');
                        push_text(next_content, Side::Left, 'M');
                        iter.next();
                    } else {
                        // A
                        push_text(content, Side::Right, 'A');
                        push_text(&String::new(), Side::Left, 'R');
                    }
                } else {
                    // Last line: A
                    push_text(content, Side::Right, 'A');
                    push_text(&String::new(), Side::Left, 'R');
                }
            }
            _ => {
                push_text(content, Side::Left, 'U');
                push_text(content, Side::Right, 'U');
            }
        }
    }
    return if mode == 'w' {
        TextsChangesHightlights {
            left: text1,
            right: text2,
            raw: diff_arr,
        }
    } else {
        TextsChangesHightlights {
            left: text2,
            right: text1,
            raw: diff_arr,
        }
    };
}

// Undo/redo stack
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
struct StackState {
    left: String,
    right: String,
    side: char, // 'l' or 'r'
    caret: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
struct UndoRedoState {
    state: StackState,
    first: bool,
    last: bool,
}

struct HistoryStack {
    stack: Vec<StackState>,
    pivot: usize
}

impl HistoryStack {
    fn new(initial_state: StackState) -> Self {
        Self {
            stack: vec![initial_state],
            pivot: 1
        }
    }

    fn first(&self) -> bool{
        self.pivot == 1
    }
    
    fn last(&self) -> bool{
        self.pivot == self.stack.len()
    }

    fn push(&mut self, value: StackState) {
        if value.left == self.stack[self.pivot - 1].left
            && value.right == self.stack[self.pivot - 1].right {
            return;
        }
        if self.pivot < self.stack.len() {
            self.stack.truncate(self.pivot);
        }
        if self.stack.len() == 200 {
            self.stack.remove(0);
        } else {
            self.pivot += 1;
        }
        self.stack.push(value);
        // for i in 0..self.stack.len() {
        //     println!("{}: {} | {}", i, self.stack[i].left, self.stack[i].right);
        // }
    }

    fn undo(&mut self) -> StackState {
        if self.pivot > 1 {
            self.pivot -= 1;
        }
        self.stack[self.pivot - 1].clone()
        // self.stack.get(self.pivot - 1)
    }

    fn redo(&mut self) -> StackState {
        if self.pivot < self.stack.len() {
            self.pivot += 1;
        }
        self.stack[self.pivot - 1].clone()
        // self.stack.get(self.pivot - 1)
    }

    fn reset(&mut self, value: StackState) {
        self.stack.clear();
        self.stack.push(value);
        self.pivot = 1;
        // for i in 0..self.stack.len() {
        //     println!("{}: {} | {}", i, self.stack[i].left, self.stack[i].right);
        // }
    }
}

#[tauri::command]
fn push(history_stack: tauri::State<'_, Arc<Mutex<HistoryStack>>>, state: StackState) {
    history_stack.lock().unwrap().push(state);
}

#[tauri::command]
fn undo(
    history_stack: tauri::State<'_, Arc<Mutex<HistoryStack>>>,
) -> UndoRedoState {
    let mut stack = history_stack.lock().unwrap();
    let state = stack.undo(); // Assuming undo() now returns the current state
    let first = stack.first();
    let last = stack.last();
    // println!("undo: {} | {} | {}", state.left, state.right, state.caret);
    UndoRedoState {
        state,
        first,
        last,
    }
}

#[tauri::command]
fn redo(
    history_stack: tauri::State<'_, Arc<Mutex<HistoryStack>>>,
) -> UndoRedoState {
    let mut stack = history_stack.lock().unwrap();
    let state = stack.redo(); // Assuming redo() now returns the current state
    let first = stack.first();
    let last = stack.last();
    // println!("redo: {} | {} | {}", state.left, state.right, state.caret);
    UndoRedoState {
        state,
        first,
        last,
    }
}

#[tauri::command]
fn reset(history_stack: tauri::State<'_, Arc<Mutex<HistoryStack>>>, state: StackState) {
    history_stack.lock().unwrap().reset(state);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(Arc::new(Mutex::new(HistoryStack::new(StackState {
            left: "".into(),
            right: "".into(),
            side: 'l',
            caret: 0,
        }))))
        .invoke_handler(tauri::generate_handler![
            get_highlights_of_text_changes,
            push,
            undo,
            redo,
            reset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
