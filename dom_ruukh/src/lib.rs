//! The Virtual DOM library which backs the `ruukh` frontend framework.
#![deny(missing_docs)]

use vcomponent::VComponent;
use velement::VElement;
use vlist::VList;
use vtext::VText;

mod component;
mod vcomponent;
mod velement;
mod vlist;
mod vtext;

/// A keyed virtual node in a virtual DOM tree.
#[derive(Debug)]
pub struct KeyedVNodes {
    /// A uniquely identifying key in the list of vnodes.
    pub key: Option<Key>,

    /// A virtual node
    pub node: VNode,
}

/// A virtual node in a virtual DOM tree.
#[derive(Debug)]
pub enum VNode {
    /// A text vnode
    Text(VText),
    /// An element vnode
    Element(VElement),
    /// A list vnode
    List(VList),
    /// A component vnode
    Component(VComponent),
}

/// Keys to identify the VNode in Virtual DOM.
/// Only the basic types are supported.
#[derive(Debug)]
pub enum Key {
    /// An `i8` key
    I8(i8),
    /// An `i16` key
    I16(i16),
    /// An `i32` key
    I32(i32),
    /// An `i64` key
    I64(i64),
    /// An `u8` key
    U8(u8),
    /// An `u16` key
    U16(u16),
    /// An `u32` key
    U32(u32),
    /// An `u64` key
    U64(u64),
    /// An `String` key
    String(String),
}