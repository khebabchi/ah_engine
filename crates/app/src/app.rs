use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Instant;
use bevy_ecs::prelude::World;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time::FormatTime;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Icon, Window, WindowAttributes};
use crate::app_manager::AppManager;
use crate::time::Time;
use crate::window::events_manager::{EventManager };
use crate::window::window_handler::WindowHolder;
use  renderer::Renderer;

/// T is the worlds_hash_map value (the ket is bevy_ecs::World)

pub struct App<UserEvent,MLM >
where
    UserEvent: 'static + Debug + Clone + Default + Eq,
    MLM: AppManager<UserEvent>,
{
    pub(crate) window_holder: WindowHolder,
    pub(crate) event_manager:EventManager<UserEvent>,
    pub(crate) event_proxy:Option<EventLoopProxy<UserEvent>>,
    pub(super) renderer:Option<Renderer<'static>>,
    pub(super) time:Time,
    pub(super) main_loop_manager:MLM,
}

impl<UE,MLM> App<UE,MLM>
where
    UE: 'static + Clone + Debug + Default + Eq,
    MLM: AppManager<UE>,
{
    pub fn new(main_loop_manager:MLM,config:WindowAttributes ) -> Self {

        App{
            window_holder: WindowHolder{
                handle: None,
                config
            },
            event_manager: Default::default(),
            event_proxy:None,
            renderer:None,
            time :Time::new(),
            main_loop_manager
        }
    }




    pub fn run(&mut self) {
        tracing_subscriber::fmt()
            .with_thread_names(true) // Optional: Show thread names
            .with_file(true) // Add file name
            .with_line_number(true) // Add line number
            .with_span_events(FmtSpan::CLOSE) // Show when spans close
            .init();
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        self.event_proxy=Some(event_loop.create_proxy());
        event_loop.run_app(self).unwrap();
    }
}