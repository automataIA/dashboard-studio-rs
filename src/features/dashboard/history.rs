//! Undo/redo history management for dashboard operations
//!
//! Implements a Command Pattern-based undo/redo system for all widget operations.

use super::models::{DataMapping, GridPosition, Layer, Widget};
use crate::features::dashboard::WidgetId;

/// Command represents any undoable operation on the dashboard
#[derive(Clone, Debug)]
pub enum Command {
    /// Add a widget to the canvas (includes layer creation)
    AddWidget {
        widget: Box<Widget>,
        layer: Layer,
    },

    /// Remove a widget from the canvas (includes layer removal)
    RemoveWidget {
        widget: Box<Widget>,
        layer: Layer,
    },

    /// Update widget properties (title, config, etc.)
    UpdateWidget {
        widget_id: WidgetId,
        previous_state: Box<Widget>,
        new_state: Box<Widget>,
    },

    /// Update widget grid position (for drag-drop)
    MoveWidget {
        widget_id: WidgetId,
        previous_position: GridPosition,
        new_position: GridPosition,
    },

    /// Update widget data mapping configuration
    UpdateDataMapping {
        widget_id: WidgetId,
        previous_mapping: Box<DataMapping>,
        new_mapping: Box<DataMapping>,
    },

    /// Batch operations (for future use with drag-drop or bulk edits)
    #[allow(dead_code)]
    Batch {
        commands: Vec<Command>,
        description: String,
    },
}

/// History manager for undo/redo operations
///
/// Maintains two stacks: undo_stack (for operations that can be undone)
/// and redo_stack (for operations that can be redone).
#[derive(Clone, Debug)]
pub struct HistoryManager {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    max_history_size: usize,
}

impl HistoryManager {
    /// Create a new history manager with specified stack size limit
    pub fn new(max_history_size: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_history_size),
            redo_stack: Vec::with_capacity(max_history_size),
            max_history_size,
        }
    }

    /// Execute a command and add to undo stack
    ///
    /// Clears the redo stack (new actions invalidate redo history).
    pub fn execute(&mut self, command: Command) {
        // Clear redo stack when new command executed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(command);

        // Limit stack size (remove oldest if exceeding limit)
        if self.undo_stack.len() > self.max_history_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the most recent command
    ///
    /// Returns the command if successful, None if no command to undo
    pub fn undo(&mut self) -> Option<&Command> {
        let command = self.undo_stack.pop()?;
        self.redo_stack.push(command.clone());
        self.undo_stack.last()
    }

    /// Redo the most recently undone command
    ///
    /// Returns the command if successful, None if no command to redo
    pub fn redo(&mut self) -> Option<&Command> {
        let command = self.redo_stack.pop()?;
        self.undo_stack.push(command.clone());
        self.undo_stack.last()
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history (e.g., after loading saved dashboard)
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the current size of the undo stack
    #[allow(dead_code)]
    pub fn undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the current size of the redo stack
    #[allow(dead_code)]
    pub fn redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_widget(id: &str) -> Widget {
        Widget {
            id: id.to_string(),
            title: "Test Widget".to_string(),
            subtitle: None,
            widget_type: crate::features::dashboard::models::WidgetType::Line,
            chart_config: crate::features::dashboard::models::ChartConfig {
                chart_type: Some(crate::features::dashboard::models::WidgetType::Line),
                data_mapping: DataMapping::default(),
                style_options: "{}".to_string(),
            },
            grid_position: GridPosition::default(),
            editing: false,
        }
    }

    fn create_test_layer(id: &str, widget_id: &str) -> Layer {
        Layer {
            id: id.to_string(),
            widget_id: widget_id.to_string(),
            label: "Test Layer".to_string(),
            icon: "show-chart".to_string(),
            visible: true,
            locked: false,
        }
    }

    #[test]
    fn test_history_manager_initial_state() {
        let history = HistoryManager::new(10);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_stack_size(), 0);
        assert_eq!(history.redo_stack_size(), 0);
    }

    #[test]
    fn test_history_manager_execute() {
        let mut history = HistoryManager::new(10);
        let widget = create_test_widget("widget1");
        let layer = create_test_layer("layer1", "widget1");

        history.execute(Command::AddWidget {
            widget: Box::new(widget.clone()),
            
            layer: layer.clone(),
        });

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_stack_size(), 1);
    }

    #[test]
    fn test_history_manager_undo() {
        let mut history = HistoryManager::new(10);
        let widget = create_test_widget("widget1");
        let layer = create_test_layer("layer1", "widget1");

        history.execute(Command::AddWidget {
            widget: Box::new(widget.clone()),
            
            layer: layer.clone(),
        });

        let command = history.undo();
        assert!(command.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());
        assert_eq!(history.undo_stack_size(), 0);
        assert_eq!(history.redo_stack_size(), 1);
    }

    #[test]
    fn test_history_manager_redo() {
        let mut history = HistoryManager::new(10);
        let widget = create_test_widget("widget1");
        let layer = create_test_layer("layer1", "widget1");

        history.execute(Command::AddWidget {
            widget: Box::new(widget.clone()),
            
            layer: layer.clone(),
        });

        history.undo();
        let command = history.redo();

        assert!(command.is_some());
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_stack_size(), 1);
        assert_eq!(history.redo_stack_size(), 0);
    }

    #[test]
    fn test_history_manager_stack_limit() {
        let mut history = HistoryManager::new(3);

        // Add 5 commands
        for i in 0..5 {
            let widget = create_test_widget(&format!("widget{}", i));
            let layer = create_test_layer(&format!("layer{}", i), &format!("widget{}", i));
            history.execute(Command::AddWidget {
                widget: Box::new(widget),
                layer,
            });
        }

        // Should only have 3 (oldest 2 dropped)
        assert_eq!(history.undo_stack_size(), 3);
        assert_eq!(history.redo_stack_size(), 0);
    }

    #[test]
    fn test_redo_invalidation() {
        let mut history = HistoryManager::new(10);
        let widget1 = create_test_widget("widget1");
        let layer1 = create_test_layer("layer1", "widget1");

        history.execute(Command::AddWidget {
            widget: Box::new(widget1.clone()),
            layer: layer1.clone(),
        });

        history.undo();
        assert!(history.can_redo());

        // New action clears redo stack
        let widget2 = create_test_widget("widget2");
        let layer2 = create_test_layer("layer2", "widget2");
        history.execute(Command::AddWidget {
            widget: Box::new(widget2),
            layer: layer2,
        });

        assert!(!history.can_redo());
    }

    #[test]
    fn test_clear() {
        let mut history = HistoryManager::new(10);
        let widget = create_test_widget("widget1");
        let layer = create_test_layer("layer1", "widget1");

        history.execute(Command::AddWidget {
            widget: Box::new(widget),
            layer,
        });

        history.undo();
        assert!(history.can_redo());

        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
}
