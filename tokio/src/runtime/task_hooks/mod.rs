use super::Config;
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct TaskHooks<U> {;
    pub(crate) task_spawn_callback: Option<OnTaskSpawnCallback<U>>,
    pub(crate) task_terminate_callback: Option<OnTaskTerminateCallback<U>>,
    pub(crate) before_poll_callback: Option<BeforeTaskPollCallback<U>>,
    pub(crate) after_poll_callback: Option<AfterTaskPollCallback<U>>,
}

/// Task metadata supplied to user-provided hooks for task events.
///
/// **Note**: This is an [unstable API][unstable]. The public API of this type
/// may break in 1.x releases. See [the documentation on unstable
/// features][unstable] for details.
///
/// [unstable]: crate#unstable-features
#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub(crate) struct TaskContext<'a, U> {
    pub(crate) id: super::task::Id,
    pub(crate) name: Option<&'a str>,
    // todo maybe we can make this properly generic instead of dynamically typed
    pub(crate) user_data: Option<&'a mut U>,
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct OnTaskSpawnContext<'a, U> {
    pub(crate) task: TaskContext<'a, U>,
    pub(crate) child_user_data: &'a mut Option<U>,
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct OnTaskTerminateContext<'a, U> {
    pub(crate) task: TaskContext<'a, U>,
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct BeforeTaskPollContext<'a, U> {
    pub(crate) task: TaskContext<'a, U>,
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct AfterTaskPollContext<'a, U> {
    pub(crate) task: TaskContext<'a, U>,
}

pub(crate) type OnTaskSpawnCallback<U> = Arc<dyn Fn(&mut OnTaskSpawnContext<'_, U>) + Send + Sync>;
pub(crate) type OnTaskTerminateCallback<U> =
    Arc<dyn Fn(&mut OnTaskTerminateContext<'_, U>) + Send + Sync>;
pub(crate) type BeforeTaskPollCallback<U> =
    Arc<dyn Fn(&mut BeforeTaskPollContext<'_, U>) + Send + Sync>;
pub(crate) type AfterTaskPollCallback<U> =
    Arc<dyn Fn(&mut AfterTaskPollContext<'_, U>) + Send + Sync>;

impl<U> TaskHooks<U> {
    pub(crate) fn spawn(
        &self,
        id: super::task::Id,
        name: Option<&str>,
        user_data: Option<&mut (dyn Any + 'static)>,
        child_user_data: &mut Option<U>,
    ) {
        if let Some(f) = self.task_spawn_callback.as_ref() {
            f(meta)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_config(config: &Config) -> Self {
        Self {
            task_spawn_callback: config.before_spawn.clone(),
            task_terminate_callback: config.after_termination.clone(),
            #[cfg(tokio_unstable)]
            before_poll_callback: config.before_poll.clone(),
            #[cfg(tokio_unstable)]
            after_poll_callback: config.after_poll.clone(),
        }
    }

    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn poll_start_callback(&self, id: super::task::Id) {
        todo!()
    }

    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn poll_stop_callback(&self, id: super::task::Id) {
        todo!()
    }
}
