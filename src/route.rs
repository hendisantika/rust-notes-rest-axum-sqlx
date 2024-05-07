use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
        health_check_handler, note_list_handler,
    },
    AppState,
};