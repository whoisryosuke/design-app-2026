use std::collections::VecDeque;

use crate::{app_state::AppState, command_queue::design_command::DesignCommand};

pub struct CommandQueue {
    queue: VecDeque<DesignCommand>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, cmd: DesignCommand, high_priority: bool) {
        if high_priority {
            self.queue.push_front(cmd);
        } else {
            self.queue.push_back(cmd);
        }
    }

    pub fn drain(&mut self, app_state: &mut AppState) {
        // Loop through commands
        // then remove them from queue
        let commands: Vec<DesignCommand> = self.queue.drain(..).collect();

        for command in commands {
            match command {
                DesignCommand::AddLayer { layer } => app_state.layers_service.add_layer(layer),
                DesignCommand::SelectLayer { layer_id } => {
                    app_state.layers_service.select_layer(layer_id)
                }
                // Assume it's a response command
                // @TODO: Would be nice to have a way to distinguish in types.
                _ => {}
            }
        }
    }
}
