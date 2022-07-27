# Runtime Interop
A highly bikeshedable name for a highly bikeshedable thing.

Only the bare minimum things responsible to spark a discussion are here.

## Design Notes
This intentionally does not expose a constructor, as it is up to the end user to construct
this and pass it to a library for use. Thus, the constructor can be runtime-specific, and may
vary.

Tokio's AsyncRead/Write implementations were chosen for the sake of picking something. Like
everything else, this is subject to bikeshedding. `std` will have this someday anyways.

`Pin<Box<dyn Future<Output=T>>>` is used here everywhere for lack of GATs, although someday
this can change.