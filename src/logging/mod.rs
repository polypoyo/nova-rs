//! Nova's logging abstraction
//!
//! _Or maybe this will be Nova's implementation of a popular logging abstraction_
//!
//! Nova logs need to go _somewhere_. While Nova itself could maintain a log file, we decided instead that Nova should
//! use the client application's logger. When Nova is initialized by the client, the client passes in callbacks for
//! logging. All of Nova's logs go through these callbacks
