// list with a mutable iterator:
pub mod second;
// persistent list that can be shared:
pub mod third;
// Now with Rc and RefCell we can become... an incredibly verbose
// pervasively mutable garbage collected language that can't collect
// cycles! Y-yaaaaay...
pub mod fourth;

// Brandon's implementation - similar to fourth, using Rc and RefCell:
pub mod brandon;

// unsafe version, which is closest to the official implementation,
// and how linked lists *should* be implemented, but I ran into some
// bugs that I couldn't resolve...
pub mod fifth;

// custom refcell implementation:
pub mod refcell;
// custom refcell implementation, as a single linked list:
pub mod refcell_singly;
