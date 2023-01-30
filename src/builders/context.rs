use super::{
    Builder, BuilderWithGlobalAndThreadContext, BuilderWithGlobalContext, BuilderWithThreadContext,
};

impl<T> Builder<T>
where
    T: Send,
{
    pub fn with_global_context<GC>(self, global_context: &GC) -> BuilderWithGlobalContext<T, GC>
    where
        GC: Sync,
    {
        BuilderWithGlobalContext {
            data: self.data,
            global_context,
        }
    }

    pub fn with_thread_context<TC, CreateTC>(
        self,
        create_thread_context: CreateTC,
    ) -> BuilderWithThreadContext<T, CreateTC>
    where
        CreateTC: Fn(usize) -> TC + Sync,
    {
        BuilderWithThreadContext {
            data: self.data,
            create_thread_context,
        }
    }
}

impl<'a, T, GC> BuilderWithGlobalContext<'a, T, GC>
where
    T: Send,
    GC: Sync,
{
    pub fn with_thread_context<TC, CreateTC>(
        self,
        create_thread_context: CreateTC,
    ) -> BuilderWithGlobalAndThreadContext<'a, T, GC, CreateTC>
    where
        CreateTC: Fn(&GC, usize) -> TC + Sync,
    {
        BuilderWithGlobalAndThreadContext {
            data: self.data,
            global_context: self.global_context,
            create_thread_context,
        }
    }
}
