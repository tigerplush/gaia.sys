use bevy::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Resource)]
pub(crate) enum InputObserver {
    Zoom(ZoomObserver),
    Pan(PanObserver),
    Overlay(OverlayObserver),
}

pub(crate) struct ZoomObserver {
    pub(crate) done: Arc<AtomicBool>,
}

impl ZoomObserver {
    pub(crate) fn new() -> Self {
        Self {
            done: Arc::new(false.into()),
        }
    }

    pub(crate) fn set_done(&mut self) {
        self.done.store(true, Ordering::Relaxed);
    }
}

pub(crate) struct PanObserver {
    pub(crate) done: Arc<AtomicBool>,
}

impl PanObserver {
    pub(crate) fn new() -> Self {
        Self {
            done: Arc::new(false.into()),
        }
    }

    pub(crate) fn set_done(&mut self) {
        self.done.store(true, Ordering::Relaxed);
    }
}

pub(crate) struct OverlayObserver {
    pub(crate) done: Arc<AtomicBool>,
}

impl OverlayObserver {
    pub(crate) fn new() -> Self {
        Self {
            done: Arc::new(false.into()),
        }
    }

    pub(crate) fn set_done(&mut self) {
        self.done.store(true, Ordering::Relaxed);
    }
}
