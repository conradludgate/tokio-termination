#![feature(try_trait_v2)]
#![feature(into_future)]
#![feature(async_iterator)]
use std::{future::Future, process::Termination};

pub struct Tokio<F>(F);

impl<F: Future> From<F> for Tokio<F> {
    fn from(f: F) -> Self {
        Self(f)
    }
}

impl<F: Future> Termination for Tokio<F>
where
    F::Output: Termination,
{
    fn report(self) -> std::process::ExitCode {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(self.0)
            .report()
    }
}
