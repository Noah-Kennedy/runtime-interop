use futures::Stream;
use std::future::Future;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};

/// A highly bikeshedable name for a highly bikeshedable thing.
///
/// Only the bare minimum things responsible to spark a discussion are here.
///
/// # Design Notes
/// This intentionally does not expose a constructor, as it is up to the end user to construct
/// this and pass it to a library for use. Thus, the constructor can be runtime-specific, and may
/// vary.
///
/// Tokio's AsyncRead/Write implementations were chosen for the sake of picking something. Like
/// everything else, this is subject to bikeshedding. `std` will have this someday anyways.
///
/// `Pin<Box<dyn Future<Output=T>>>` is used here everywhere for lack of GATs, although someday
/// this can change.
pub trait RuntimeBikeshed {
    type TcpStream: AsyncRead + AsyncWrite;
    type TcpListener: Stream<Item = io::Result<(Self::TcpStream, SocketAddr)>>;

    fn connect_tcp<A: ToSocketAddrs>(
        &self,
        addr: A,
    ) -> Pin<Box<dyn Future<Output = io::Result<TcpStream>>>>;

    fn bind_tcp<A: ToSocketAddrs>(
        &self,
        addr: A,
    ) -> Pin<Box<dyn Future<Output = io::Result<TcpListener>>>>;

    /// no gats, weak joinhandle 😔
    fn spawn<T>(
        &self,
        f: impl Future<Output = T>,
    ) -> Pin<Box<dyn Future<Output = Result<T, JoinError>>>>;
}

pub enum JoinError {
    Terminated,
    Panicked,
}
