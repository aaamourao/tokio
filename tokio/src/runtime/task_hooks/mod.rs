use super::task;
use std::marker::PhantomData;

/// A factory which produces new [`TaskHookHarness`] objects for tasks which either have been
/// spawned in "detached mode" via the builder, or which were spawned from outside the runtime or
/// from another context where no [`TaskHookHarness`] was present.
pub trait TaskHooksFactory {
    /// Create a new [`TaskHookHarness`] object which the runtime will attach to a given task.
    fn on_top_level_spawn(
        &self,
        ctx: &mut OnTopLevelTaskSpawnContext<'_>,
    ) -> Option<Box<dyn TaskHookHarness + Send + Sync + 'static>>;
}

/// Trait for user-provided "harness" objects which are attached to tasks and provide hook
/// implementations.
pub trait TaskHookHarness {
    /// Pre-poll task hook which runs arbitrary user logic.
    fn before_poll(&mut self, ctx: &mut BeforeTaskPollContext<'_>);

    /// Post-poll task hook which runs arbitrary user logic.
    fn after_poll(&mut self, ctx: &mut BeforeTaskPollContext<'_>);

    /// Task hook which runs when this task spawns a child, unless that child is explicitly spawned
    /// detached from the parent.
    ///
    /// This hook creates a harness for the child, or detaches the child from any instrumentation.
    fn on_child_spawn(
        &mut self,
        ctx: &mut OnChildTaskSpawnContext<'_>,
    ) -> Option<Box<dyn TaskHookHarness>>;

    /// Task hook which runs on task termination.
    fn on_task_terminate(&mut self, ctx: &mut OnTaskTerminateContext<'_>);
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct OnTopLevelTaskSpawnContext<'a> {
    name: Option<&'a str>,
    id: task::Id,
}

impl<'a> OnTopLevelTaskSpawnContext<'a> {
    /// Returns the name of the task, if one was provided via a builder.
    pub fn name(&self) -> Option<&str> {
        self.name
    }

    /// Returns the ID of the task.
    pub fn id(&self) -> task::Id {
        self.id
    }
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct OnChildTaskSpawnContext<'a> {
    name: Option<&'a str>,
    id: task::Id,
}

impl<'a> OnChildTaskSpawnContext<'a> {
    /// Returns the name of the task, if one was provided via a builder.
    pub fn name(&self) -> Option<&str> {
        self.name
    }

    /// Returns the ID of the task.
    pub fn id(&self) -> task::Id {
        self.id
    }
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct OnTaskTerminateContext<'a> {
    _phantom: PhantomData<&'a ()>,
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct BeforeTaskPollContext<'a> {
    _phantom: PhantomData<&'a ()>,
}

#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct AfterTaskPollContext<'a> {
    _phantom: PhantomData<&'a ()>,
}
