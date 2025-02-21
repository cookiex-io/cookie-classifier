use pin_project::pin_project;
use redis::RedisFuture;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::spawn;
use tower::BoxError;
use crate::service::rate::Rate;

use super::error::RateLimitExceeded;

type QueryFuture = RedisFuture<'static, (Option<u32>, Option<u32>)>;
type IncrementFuture = RedisFuture<'static, ()>;

#[derive(Debug)]
enum State {
    Querying,
    Rejected,
    Ready,
    Executing,
}

#[pin_project]
pub struct ResponseFuture<R> {
    #[pin]
    query_future: QueryFuture,
    #[pin]
    response_future: R,
    #[pin]
    increment_future: Option<IncrementFuture>,
    rate: Rate,
    previous_window_ratio: f64,
    state: State,
}

impl<R> ResponseFuture<R> {
    pub(crate) fn new(
        query_future: QueryFuture,
        increment_future: IncrementFuture,
        response_future: R,
        rate: Rate,
        previous_window_ratio: f64,
    ) -> Self {
        ResponseFuture {
            query_future,
            response_future,
            increment_future: Some(increment_future),
            rate,
            previous_window_ratio,
            state: State::Querying,
        }
    }
}

impl<R, Response, Error> Future for ResponseFuture<R>
where
    R: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match *this.state {
                State::Querying => match this.query_future.as_mut().poll(cx) {
                    Poll::Ready(Ok((previous, current))) => {
                        let previous = previous.unwrap_or(0);
                        let current = current.unwrap_or(0);
                        let count =
                            ((previous as f64) * *this.previous_window_ratio) as u32 + current;
                        *this.state = if count < this.rate.rate() {
                            State::Ready
                        } else {
                            State::Rejected
                        };
                    }
                    Poll::Ready(Err(_)) => *this.state = State::Rejected,
                    _ => return Poll::Pending,
                },
                State::Rejected => return Poll::Ready(Err(RateLimitExceeded.into())),
                State::Ready => {
                    spawn(this.increment_future.take().unwrap());
                    *this.state = State::Executing;
                }
                State::Executing => {
                    return match this.response_future.poll(cx) {
                        Poll::Ready(result) => Poll::Ready(result.map_err(Into::into)),
                        Poll::Pending => Poll::Pending,
                    }
                }
            }
        }
    }
}