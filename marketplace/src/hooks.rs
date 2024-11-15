use std::marker::PhantomData;

use async_trait::async_trait;
use hotshot::types::Event;
use hotshot_types::traits::node_implementation::NodeType;

#[async_trait]
pub trait BuilderHooks<Types: NodeType>: Sync + Send + 'static {
    #[inline(always)]
    async fn process_transactions(
        &self,
        transactions: Vec<Types::Transaction>,
    ) -> Vec<Types::Transaction> {
        transactions
    }

    #[inline(always)]
    async fn handle_hotshot_event(&self, _event: &Event<Types>) {}
}

#[async_trait]
impl<T: ?Sized, Types> BuilderHooks<Types> for Box<T>
where
    Types: NodeType,
    T: BuilderHooks<Types>,
{
    #[inline(always)]
    async fn process_transactions(
        &self,
        transactions: Vec<Types::Transaction>,
    ) -> Vec<Types::Transaction> {
        (**self).process_transactions(transactions).await
    }

    #[inline(always)]
    async fn handle_hotshot_event(&self, event: &Event<Types>) {
        (**self).handle_hotshot_event(event).await
    }
}

pub struct NoHooks<Types: NodeType>(pub PhantomData<Types>);

impl<Types: NodeType> BuilderHooks<Types> for NoHooks<Types> {}
