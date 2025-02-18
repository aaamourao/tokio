use super::Config;
use std::any::Any;
use std::sync::Arc;



#[derive(Clone)]
pub(crate) struct TaskHooks<U> {
    pub(crate) task_spawn_callback: Option<OnTaskSpawnCallback<U>>,
    pub(crate) task_terminate_callback: Option<OnTaskTerminateCallback<U>>,
    pub(crate) before_poll_callback: Option<BeforeTaskPollCallback<U>>,
    pub(crate) after_poll_callback: Option<AfterTaskPollCallback<U>>,
}

macro_rules! gen_task_context_methods {
    ($structname: ident) => {
        impl<U> $structname<U> {
            /// Returns the opaque ID of the task.
            pub fn id(&self) -> super::task::Id {
                self.task.id()
            }

            /// Returns a reference to the name of the task, if the task was named.
            pub fn name(&self) -> Option<&str> {
                self.task.name()
            }

            /// Returns a mutable reference to optional user provided data stored in the context.
            ///
            /// This can be added either via a builder method or via a spawn hook.
            pub fn user_data(&mut self) -> Option<&mut U> {
                self.task.user_data()
            }
        }
    };
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
    pub(crate) child_user_data: &'a mut Option<Box<U>>,
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

impl<'a, U> OnTaskSpawnContext<'a, U> {
    pub fn child_user_data(&mut self) -> &'a mut Option<Box<U>> {
        self.child_user_data
    }
}

gen_task_context_methods!(OnTaskSpawnContext);
gen_task_context_methods!(OnTaskTerminateContext);
gen_task_context_methods!(BeforeTaskPollContext);
gen_task_context_methods!(AfterTaskPollContext);

impl<'a, U> OnTaskSpawnContext<U> {
    pub fn child_user_data(&mut self) -> Option<&'a mut U> {
        self.child_user_data.as_mut()
    }
}

pub(crate) type OnTaskSpawnCallback<U> = Arc<dyn Fn(&mut OnTaskSpawnContext<'_, U>) + Send + Sync>;
pub(crate) type OnTaskTerminateCallback<U> =
    Arc<dyn Fn(&mut OnTaskTerminateContext<'_, U>) + Send + Sync>;
pub(crate) type BeforeTaskPollCallback<U> =
    Arc<dyn Fn(&mut BeforeTaskPollContext<'_, U>) + Send + Sync>;
pub(crate) type AfterTaskPollCallback<U> =
    Arc<dyn Fn(&mut AfterTaskPollContext<'_, U>) + Send + Sync>;

impl<U> TaskHooks<U> {
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

    pub(crate) fn dispatch_task_spawn_callback(
        &self,
        id: super::task::Id,
        name: Option<&str>,
        user_data: Option<&mut U>,
        child_user_data: &mut Option<Box<U>>,
    ) {
        if let Some(f) = self.task_spawn_callback.as_ref() {
            let mut ctx = OnTaskSpawnContext {
                task: TaskContext {
                    id,
                    name,
                    user_data,
                },
                child_user_data,
            };

            f(&mut ctx)
        }
    }

    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn dispatch_task_terminate_callback(&self, id: super::task::Id) {
        if let Some(f) = self.task_terminate_callback.as_ref() {
            let mut ctx = OnTaskTerminateContext {
                task: TaskContext {
                    id,
                    name,
                    user_data,
                },
            };

            f(&mut ctx)
        }
    }

    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn dispatch_pre_poll_callback(&self, id: super::task::Id) {
        if let Some(f) = self.before_poll_callback.as_ref() {
            let mut ctx = BeforeTaskPollContext {
                task: TaskContext {
                    id,
                    name,
                    user_data,
                },
            };

            f(&mut ctx)
        }
    }

    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn dispatch_post_poll_callback(&self, id: super::task::Id) {
        if let Some(f) = self.after_poll_callback.as_ref() {
            let mut ctx = AfterTaskPollContext {
                task: TaskContext {
                    id,
                    name,
                    user_data,
                },
            };

            f(&mut ctx)
        }
    }
}

impl<'a, U> TaskContext<'a, U> {
    fn id(&self) -> super::task::Id {
        self.id
    }

    fn name(&self) -> Option<&str> {
        self.name
    }

    fn user_data(&mut self) -> Option<&mut U> {
        self.user_data.as_mut()
    }
}
