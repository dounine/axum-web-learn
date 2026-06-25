mod api;
mod app;
mod auth;
mod common;
mod config;
mod entity;
mod error;
mod handler;
mod logger;
mod middleware;
mod server;
mod util;

pub struct FastZip<F> {
    _marker: std::marker::PhantomData<F>,
    // data: T,
}
pub trait BinRead: Sized {
    type Args<'a>: Send
    where
        Self: 'a;

    fn read<'a>(args: Self::Args<'a>) -> Result<Self, Box<dyn std::error::Error>>;
}
impl<F, Fut> BinRead for FastZip<F>
where
    F: FnMut(u64) -> Fut + Send,
    Fut: Future<Output = ()> + Send,
{
    type Args<'a>
        = (&'a u32, &'a mut F)
    where
        F: 'a;

    fn read<'a>(args: Self::Args<'a>) -> Result<Self, Box<dyn std::error::Error>> {
        let (age, callback) = args;
        callback(0);
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run().await?;

    Ok(())
}
