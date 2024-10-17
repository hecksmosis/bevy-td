use bevy::prelude::*;

pub struct NotifyPlugin;

impl Plugin for NotifyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NotifyQueue::default())
            .add_systems(Update, notification_handler);
    }
}

#[derive(Debug, Resource, Clone, Default)]
pub struct NotifyQueue {
    _inner: Vec<String>,
}

impl NotifyQueue {
    pub fn next(&mut self) -> Option<String> {
        self._inner.pop()
    }

    pub fn push(&mut self, item: impl Into<String>) {
        self._inner.push(item.into())
    }
}

fn notification_handler(mut notify_queue: ResMut<NotifyQueue>) {
    if let Some(message) = notify_queue.next() {
        info!("{}", message);
    }
}
