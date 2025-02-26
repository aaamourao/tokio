use crate::future::Future;
use crate::loom::sync::Arc;
use crate::runtime::scheduler::multi_thread::worker;
use crate::runtime::{blocking, driver, task::{self, JoinHandle}, OnChildTaskSpawnContext, OnTopLevelTaskSpawnContext, TaskHookHarness, TaskHookHarnessFactory};
use crate::util::RngSeedGenerator;

use crate::runtime::task::Schedule;
use std::fmt;

mod metrics;

cfg_taskdump! {
    mod taskdump;
}

/// Handle to the multi thread scheduler
pub(crate) struct Handle {
    /// Task spawner
    pub(super) shared: worker::Shared,

    /// Resource driver handles
    pub(crate) driver: driver::Handle,

    /// Blocking pool spawner
    pub(crate) blocking_spawner: blocking::Spawner,

    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,

    /// User-supplied hooks to invoke for things
    #[cfg(tokio_unstable)]
    pub(crate) task_hooks: Option<Arc<dyn TaskHookHarnessFactory + Send + Sync + 'static>>,
}

impl Handle {
    /// Spawns a future onto the thread pool
    pub(crate) fn spawn<F>(
        me: &Arc<Self>,
        future: F,
        id: task::Id,
        parent: Option<&(dyn TaskHookHarnessFactory + Send + Sync + 'static)>,
    ) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::bind_new_task(me, future, id, parent)
    }

    pub(crate) fn shutdown(&self) {
        self.close();
    }

    pub(super) fn bind_new_task<T>(
        me: &Arc<Self>,
        future: T,
        id: task::Id,
        parent: Option<&mut (dyn TaskHookHarness + Send + Sync + 'static)>,
    ) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let hooks = if let Some(parent) = parent {
            parent.on_child_spawn(&mut OnChildTaskSpawnContext {
                id,
                _phantom: Default::default(),
            })
        } else {
            if let Some(hooks) = me.hooks() {
                hooks.on_top_level_spawn(&mut OnTopLevelTaskSpawnContext {
                    id,
                    _phantom: Default::default()
                })
            } else {
                None
            }
        };


        
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id);

        me.schedule_option_task_without_yield(notified);

        handle
    }
}

cfg_unstable! {
    use std::num::NonZeroU64;

    impl Handle {
        pub(crate) fn owned_id(&self) -> NonZeroU64 {
            self.shared.owned.id
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("multi_thread::Handle { ... }").finish()
    }
}
