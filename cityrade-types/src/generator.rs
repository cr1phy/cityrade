use async_trait::async_trait;

#[async_trait]
pub trait Generator {
    type Output;

    async fn generate(&self) -> Self::Output;
}
