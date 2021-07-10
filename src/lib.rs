//! # Swanling
//!
//! Have you ever been attacked by a swanling?
//!
//! Swanling is a load testing framework inspired by [Locust](https://locust.io/).
//! User behavior is defined with standard Rust code.
//!
//! Swanling load tests, called Swanling Attacks, are built by creating an application
//! with Cargo, and declaring a dependency on the Swanling library.
//!
//! Swanling uses [`reqwest`](https://docs.rs/reqwest/) to provide a convenient HTTP
//! client.
//!
//! ## Documentation
//!
//! - [README](https://github.com/begleybrothers/swanling/blob/main/README.md)
//! - [Developer documentation](https://docs.rs/swanling/)
//! ## Creating and running a Swanling load test
//!
//! ### Creating a simple Swanling load test
//!
//! First create a new empty cargo application, for example:
//!
//! ```bash
//! $ cargo new loadtest
//!      Created binary (application) `loadtest` package
//! $ cd loadtest/
//! ```
//!
//! Add Swanling as a dependency in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! swanling = "0.12"
//! ```
//!
//! Add the following boilerplate `use` declaration at the top of your `src/main.rs`:
//!
//! ```rust
//! use swanling::prelude::*;
//! ```
//!
//! Using the above prelude will automatically add the following `use` statements
//! necessary for your load test, so you don't need to manually add them:
//!
//! ```rust
//! use swanling::swanling::{
//!     SwanlingTask, SwanlingTaskError, SwanlingTaskFunction, SwanlingTaskResult, SwanlingTaskSet, SwanlingUser,
//! };
//! use swanling::metrics::SwanlingMetrics;
//! use swanling::{
//!     task, taskset, SwanlingAttack, SwanlingDefault, SwanlingDefaultType, SwanlingError, SwanlingScheduler,
//! };
//! ```
//!
//! Below your `main` function (which currently is the default `Hello, world!`), add
//! one or more load test functions. The names of these functions are arbitrary, but it is
//! recommended you use self-documenting names. Load test functions must be async. Each load
//! test function must accept a reference to a [`SwanlingUser`](./swanling/struct.SwanlingUser.html) object
//! and return a [`SwanlingTaskResult`](./swanling/type.SwanlingTaskResult.html). For example:
//!
//! ```rust
//! use swanling::prelude::*;
//!
//! async fn loadtest_foo(user: &SwanlingUser) -> SwanlingTaskResult {
//!   let _swanling = user.get("/path/to/foo").await?;
//!
//!   Ok(())
//! }
//! ```
//!
//! In the above example, we're using the [`SwanlingUser`](./swanling/struct.SwanlingUser.html) helper
//! [`get`](./swanling/struct.SwanlingUser.html#method.get) to load a path on the website we are load
//! testing. This helper creates a
//! [`reqwest::RequestBuilder`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html)
//! object and uses it to build and execute a request for the above path. If you want access
//! to the [`RequestBuilder`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html)
//! object, you can instead use the [`swanling_get`](./swanling/struct.SwanlingUser.html#method.swanling_get)
//! helper, for example to set a timeout on this specific request:
//!
//! ```rust
//! use std::time;
//!
//! use swanling::prelude::*;
//!
//! async fn loadtest_bar(user: &SwanlingUser) -> SwanlingTaskResult {
//!     let request_builder = user.swanling_get("/path/to/bar").await?;
//!     let _swanling = user.swanling_send(request_builder.timeout(time::Duration::from_secs(3)), None).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! We pass the [`RequestBuilder`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html)
//! object to [`swanling_send`](./swanling/struct.SwanlingUser.html#method.swanling_send) which builds and
//! executes it, also collecting useful metrics. The
//! [`.await`](https://doc.rust-lang.org/std/keyword.await.html) at the end is necessary as
//! [`swanling_send`](./swanling/struct.SwanlingUser.html#method.swanling_send) is an async function.
//!
//! Once all our tasks are created, we edit the main function to initialize swanling and register
//! the tasks. In this very simple example we only have two tasks to register, while in a real
//! load test you can have any number of task sets with any number of individual tasks.
//!
//! ```rust
//! use swanling::prelude::*;
//!
//! fn main() -> Result<(), SwanlingError> {
//!     let _swanling_metrics = SwanlingAttack::initialize()?
//!         .register_taskset(taskset!("LoadtestTasks")
//!             // Register the foo task, assigning it a weight of 10.
//!             .register_task(task!(loadtest_foo).set_weight(10)?)
//!             // Register the bar task, assigning it a weight of 2 (so it
//!             // runs 1/5 as often as bar). Apply a task name which shows up
//!             // in metrics.
//!             .register_task(task!(loadtest_bar).set_name("bar").set_weight(2)?)
//!         )
//!         // You could also set a default host here, for example:
//!         .set_default(SwanlingDefault::Host, "http://dev.local/")?
//!         // We set a default run time so this test runs to completion.
//!         .set_default(SwanlingDefault::RunTime, 1)?
//!         .execute()?;
//!
//!     Ok(())
//! }
//!
//! // A task function that loads `/path/to/foo`.
//! async fn loadtest_foo(user: &SwanlingUser) -> SwanlingTaskResult {
//!     let _swanling = user.get("/path/to/foo").await?;
//!
//!     Ok(())
//! }
//!
//! // A task function that loads `/path/to/bar`.
//! async fn loadtest_bar(user: &SwanlingUser) -> SwanlingTaskResult {
//!     let _swanling = user.get("/path/to/bar").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Swanling now spins up a configurable number of users, each simulating a user on your
//! website. Thanks to [`reqwest`](https://docs.rs/reqwest/), each user maintains its own
//! web client state, handling cookies and more so your "users" can log in, fill out forms,
//! and more, as real users on your sites would do.
//!
//! ### Running the Swanling load test
//!
//! Attempts to run our example will result in an error, as we have not yet defined the
//! host against which this load test should be run. We intentionally do not hard code the
//! host in the individual tasks, as this allows us to run the test against different
//! environments, such as local development, staging, and production.
//!
//! ```bash
//! $ cargo run --release
//!    Compiling loadtest v0.1.0 (~/loadtest)
//!     Finished release [optimized] target(s) in 1.52s
//!      Running `target/release/loadtest`
//! Error: InvalidOption { option: "--host", value: "", detail: "A host must be defined via the --host option, the SwanlingAttack.set_default() function, or the SwanlingTaskSet.set_host() function (no host defined for WebsiteUser)." }
//! ```
//! Pass in the `-h` flag to see all available run-time options. For now, we'll use a few
//! options to customize our load test.
//!
//! ```bash
//! $ cargo run --release -- --host http://dev.local -t 30s -v
//! ```
//!
//! The first option we specified is `--host`, and in this case tells Swanling to run the load test
//! against a VM on my local network. The `-t 30s` option tells Swanling to end the load test after 30
//! seconds (for real load tests you'll certainly want to run it longer, you can use `h`, `m`, and
//! `s` to specify hours, minutes and seconds respectively. For example, `-t1h30m` would run the
//! load test for 1 hour 30 minutes). Finally, the `-v` flag tells swanling to display INFO and higher
//! level logs to stdout, giving more insight into what is happening. (Additional `-v` flags will
//! result in considerably more debug output, and are not recommended for running actual load tests;
//! they're only useful if you're trying to debug Swanling itself.)
//!
//! Running the test results in the following output (broken up to explain it as it goes):
//!
//! ```bash
//!    Finished release [optimized] target(s) in 0.05s
//!     Running `target/release/loadtest --host 'http://dev.local' -t 30s -v`
//! 15:42:23 [ INFO] Output verbosity level: INFO
//! 15:42:23 [ INFO] Logfile verbosity level: WARN
//! ```
//!
//! If we set the `--log-file` flag, Swanling will write a log file with WARN and higher level logs
//! as you run the test from (add a `-g` flag to log all INFO and higher level logs).
//!
//! ```bash
//! 15:42:23 [ INFO] concurrent users defaulted to 8 (number of CPUs)
//! 15:42:23 [ INFO] run_time = 30
//! 15:42:23 [ INFO] hatch_rate = 1
//! ```
//!
//! Swanling will default to launching 1 user per available CPU core, and will launch them all in
//! one second. You can change how many users are launched with the `-u` option, and you can
//! change how many users are launched per second with the `-r` option. For example, `-u30 -r2`
//! would launch 30 users over 15 seconds (two users per second).
//!
//! ```bash
//! 15:42:23 [ INFO] global host configured: http://dev.local/
//! 15:42:23 [ INFO] initializing user states...
//! 15:42:23 [ INFO] launching user 1 from LoadtestTasks...
//! 15:42:24 [ INFO] launching user 2 from LoadtestTasks...
//! 15:42:25 [ INFO] launching user 3 from LoadtestTasks...
//! 15:42:26 [ INFO] launching user 4 from LoadtestTasks...
//! 15:42:27 [ INFO] launching user 5 from LoadtestTasks...
//! 15:42:28 [ INFO] launching user 6 from LoadtestTasks...
//! 15:42:29 [ INFO] launching user 7 from LoadtestTasks...
//! 15:42:30 [ INFO] launching user 8 from LoadtestTasks...
//! 15:42:31 [ INFO] launched 8 users...
//! 15:42:31 [ INFO] printing running metrics after 8 seconds...
//! ```
//!
//! Each user is launched in its own thread with its own user state. Swanling is able to make
//! very efficient use of server resources. By default Swanling resets the metrics after all
//! users are launched, but first it outputs the metrics collected while ramping up:
//!
//! ```bash
//! 15:42:31 [ INFO] printing running metrics after 8 seconds...
//!
//!  === PER TASK METRICS ===
//!  ------------------------------------------------------------------------------
//!  Name                     |   # times run |        # fails |   task/s |  fail/s
//!  ------------------------------------------------------------------------------
//!  1: LoadtestTasks         |
//!    1:                     |         2,033 |         0 (0%) |   254.12 |    0.00
//!    2: bar                 |           407 |         0 (0%) |    50.88 |    0.00
//!  -------------------------+---------------+----------------+----------+--------
//!  Aggregated               |         2,440 |         0 (0%) |   305.00 |    0.00
//!  ------------------------------------------------------------------------------
//!  Name                     |    Avg (ms) |        Min |         Max |     Median
//!  ------------------------------------------------------------------------------
//!  1: LoadtestTasks         |
//!    1:                     |       14.23 |          6 |          32 |         14
//!    2: bar                 |       14.13 |          6 |          30 |         14
//!  -------------------------+-------------+------------+-------------+-----------
//!  Aggregated               |       14.21 |          6 |          32 |         14
//!
//!  === PER REQUEST METRICS ===
//!  ------------------------------------------------------------------------------
//!  Name                     |        # reqs |        # fails |    req/s |  fail/s
//!  ------------------------------------------------------------------------------
//!  GET /                    |         2,033 |         0 (0%) |   254.12 |    0.00
//!  GET bar                  |           407 |         0 (0%) |    50.88 |    0.00
//!  -------------------------+---------------+----------------+----------+--------
//!  Aggregated               |         2,440 |         0 (0%) |   305.00 |    0.00
//!  ------------------------------------------------------------------------------
//!  Name                     |    Avg (ms) |        Min |        Max |      Median
//!  ------------------------------------------------------------------------------
//!  GET /                    |       14.18 |          6 |          32 |         14
//!  GET bar                  |       14.08 |          6 |          30 |         14
//!  -------------------------+-------------+------------+-------------+-----------
//!  Aggregated               |       14.16 |          6 |          32 |         14
//!
//! All 8 users hatched, resetting metrics (disable with --no-reset-metrics).
//! ```
//!
//! Swanling can optionally display running metrics if started with `--running-metrics INT`
//! where INT is an integer value in seconds. For example, if Swanling is started with
//! `--running-metrics 15` it will display running values approximately every 15 seconds.
//! Running metrics are broken into several tables. First are the per-task metrics which
//! are further split into two sections. The first section shows how many requests have
//! been made, how many of them failed (non-2xx response), and the corresponding per-second
//! rates.
//!
//! This table shows details for all Task Sets and all Tasks defined by your load test,
//! regardless of if they actually run. This can be useful to ensure that you have set
//! up weighting as intended, and that you are simulating enough users. As our first
//! task wasn't named, it just showed up as "1:". Our second task was named, so it shows
//! up as the name we gave it, "bar".
//!
//! ```bash
//! 15:42:46 [ INFO] printing running metrics after 15 seconds...
//!
//!  === PER TASK METRICS ===
//!  ------------------------------------------------------------------------------
//!  Name                     |   # times run |        # fails |   task/s |  fail/s
//!  ------------------------------------------------------------------------------
//!  1: LoadtestTasks         |
//!    1:                     |         4,618 |         0 (0%) |   307.87 |    0.00
//!    2: bar                 |           924 |         0 (0%) |    61.60 |    0.00
//!  -------------------------+---------------+----------------+----------+--------
//!  Aggregated               |         5,542 |         0 (0%) |   369.47 |    0.00
//!  ------------------------------------------------------------------------------
//!  Name                     |    Avg (ms) |        Min |         Max |     Median
//!  ------------------------------------------------------------------------------
//!  1: LoadtestTasks         |
//!    1:                     |       21.17 |          8 |         151 |         19
//!    2: bar                 |       21.62 |          9 |         156 |         19
//!  -------------------------+-------------+------------+-------------+-----------
//!  Aggregated               |       21.24 |          8 |         156 |         19
//! ```
//!
//! The second table breaks down the same metrics by request instead of by Task. For
//! our simple load test, each Task only makes a single request, so the metrics are
//! the same. There are two main differences. First, metrics are listed by request
//! type and path or name. The first request shows up as `GET /path/to/foo` as the
//! request was not named. The second request shows up as `GET bar` as the request
//! was named. The times to complete each are slightly smaller as this is only the
//! time to make the request, not the time for Swanling to execute the entire task.
//!
//! ```bash
//!  === PER REQUEST METRICS ===
//!  ------------------------------------------------------------------------------
//!  Name                     |        # reqs |        # fails |    req/s |  fail/s
//!  ------------------------------------------------------------------------------
//!  GET /path/to/foo         |         4,618 |         0 (0%) |   307.87 |    0.00
//!  GET bar                  |           924 |         0 (0%) |    61.60 |    0.00
//!  -------------------------+---------------+----------------+----------+--------
//!  Aggregated               |         5,542 |         0 (0%) |   369.47 |    0.00
//!  ------------------------------------------------------------------------------
//!  Name                     |    Avg (ms) |        Min |        Max |      Median
//!  ------------------------------------------------------------------------------
//!  GET /path/to/foo         |       21.13 |          8 |         151 |         19
//!  GET bar                  |       21.58 |          9 |         156 |         19
//!  -------------------------+-------------+------------+-------------+-----------
//!  Aggregated               |       21.20 |          8 |         156 |         19
//! ```
//!
//! Note that Swanling respected the per-task weights we set, and `foo` (with a weight of 10)
//! is being loaded five times as often as `bar` (with a weight of 2). On average
//! each page is returning within `21.2` milliseconds. The quickest page response was
//! for `foo` in `8` milliseconds. The slowest page response was for `bar` in `156`
//! milliseconds.
//!
//! ```bash
//! 15:43:02 [ INFO] stopping after 30 seconds...
//! 15:43:02 [ INFO] waiting for users to exit
//! 15:43:02 [ INFO] exiting user 3 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 4 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 5 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 8 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 2 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 7 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 6 from LoadtestTasks...
//! 15:43:02 [ INFO] exiting user 1 from LoadtestTasks...
//! 15:43:02 [ INFO] printing metrics after 30 seconds...
//! ```
//!
//! Our example only runs for 30 seconds, so we only see running metrics once. When
//! the test completes, we get more detail in the final summary. The first two tables
//! are the same as what we saw earlier, however now they include all metrics for the
//! entire length of the load test:
//!
//! ```bash
//!  === PER TASK METRICS ===
//!  ------------------------------------------------------------------------------
//!  Name                     |   # times run |        # fails |   task/s |  fail/s
//!  ------------------------------------------------------------------------------
//!  1: LoadtestTasks         |
//!    1:                     |         9,974 |         0 (0%) |   332.47 |    0.00
//!    2: bar                 |         1,995 |         0 (0%) |    66.50 |    0.00
//!  -------------------------+---------------+----------------+----------+--------
//!  Aggregated               |        11,969 |         0 (0%) |   398.97 |    0.00
//!  ------------------------------------------------------------------------------
//!  Name                     |    Avg (ms) |        Min |         Max |     Median
//!  ------------------------------------------------------------------------------
//!  1: LoadtestTasks         |
//!    1:                     |       19.65 |          8 |         151 |         18
//!    2: bar                 |       19.92 |          9 |         156 |         18
//!  -------------------------+-------------+------------+-------------+-----------
//!  Aggregated               |       19.69 |          8 |         156 |         18
//!
//!  === PER REQUEST METRICS ===
//!  ------------------------------------------------------------------------------
//!  Name                     |        # reqs |        # fails |    req/s |  fail/s
//!  ------------------------------------------------------------------------------
//!  GET /                    |         9,974 |         0 (0%) |   332.47 |    0.00
//!  GET bar                  |         1,995 |         0 (0%) |    66.50 |    0.00
//!  -------------------------+---------------+----------------+----------+--------
//!  Aggregated               |        11,969 |         0 (0%) |   398.97 |    0.00
//!  ------------------------------------------------------------------------------
//!  Name                     |    Avg (ms) |        Min |        Max |      Median
//!  ------------------------------------------------------------------------------
//!  GET /                    |       19.61 |          8 |         151 |         18
//!  GET bar                  |       19.88 |          9 |         156 |         18
//!  -------------------------+-------------+------------+-------------+-----------
//!  Aggregated               |       19.66 |          8 |         156 |         18
//!  ------------------------------------------------------------------------------
//! ```
//!
//! The ratio between `foo` and `bar` remained 5:2 as expected.
//!
//! ```bash
//!  ------------------------------------------------------------------------------
//!  Slowest page load within specified percentile of requests (in ms):
//!  ------------------------------------------------------------------------------
//!  Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
//!  ------------------------------------------------------------------------------
//!  GET /                    |     18 |     21 |     29 |     79 |    140 |    140
//!  GET bar                  |     18 |     21 |     29 |    120 |    150 |    150
//!  -------------------------+--------+--------+--------+--------+--------+-------
//!  Aggregated               |     18 |     21 |     29 |     84 |    140 |    156
//! ```
//!
//! A new table shows additional information, breaking down response-time by
//! percentile. This shows that the slowest page loads only happened in the
//! slowest 1% of page loads, so were an edge case. 98% of the time page loads
//! happened in 29 milliseconds or less.
//!
//! ## License
//!
//! Copyright 2020-21 Jeremy Andrews
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

#[macro_use]
extern crate log;

pub mod controller;
pub mod logger;
#[cfg(feature = "gaggle")]
mod manager;
pub mod metrics;
pub mod prelude;
mod report;
pub mod swanling;
mod throttle;
mod user;
pub mod util;
#[cfg(feature = "gaggle")]
mod worker;

use chrono::prelude::*;
use gumdrop::Options;
use lazy_static::lazy_static;
#[cfg(feature = "gaggle")]
use nng::Socket;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use std::{fmt, io, time};
use tokio::fs::File;
use tokio::runtime::Runtime;

use crate::controller::{SwanlingControllerProtocol, SwanlingControllerRequest};
use crate::logger::{SwanlingLogFormat, SwanlingLoggerJoinHandle, SwanlingLoggerTx};
use crate::metrics::{SwanlingCoordinatedOmissionMitigation, SwanlingMetric, SwanlingMetrics};
use crate::swanling::{
    GaggleUser, SwanlingTask, SwanlingTaskSet, SwanlingUser, SwanlingUserCommand,
};
#[cfg(feature = "gaggle")]
use crate::worker::{register_shutdown_pipe_handler, GaggleMetrics};

/// Constant defining Swanling's default port when running a Regatta.
const DEFAULT_PORT: &str = "5115";

/// Constant defining Swanling's default telnet Controller port.
const DEFAULT_TELNET_PORT: &str = "5116";

/// Constant defining Swanling's default WebSocket Controller port.
const DEFAULT_WEBSOCKET_PORT: &str = "5117";

// WORKER_ID is only used when running a gaggle (a distributed load test).
lazy_static! {
    static ref WORKER_ID: AtomicUsize = AtomicUsize::new(0);
}

/// Internal representation of a weighted task list.
type WeightedSwanlingTasks = Vec<(usize, String)>;

/// Internal representation of unsequenced tasks.
type UnsequencedSwanlingTasks = Vec<SwanlingTask>;
/// Internal representation of sequenced tasks.
type SequencedSwanlingTasks = BTreeMap<usize, Vec<SwanlingTask>>;

/// Returns the unique identifier of the running Worker when running in Regatta mode.
///
/// The first Worker to connect to the Manager is assigned an ID of 1. For each
/// subsequent Worker to connect to the Manager the ID is incremented by 1. This
/// identifier is primarily an aid in tracing logs.
pub fn get_worker_id() -> usize {
    WORKER_ID.load(Ordering::Relaxed)
}

#[cfg(not(feature = "gaggle"))]
#[derive(Debug, Clone)]
/// Socket used for coordinating a Regatta distributed load test.
pub(crate) struct Socket {}

/// An enumeration of all errors a [`SwanlingAttack`](./struct.SwanlingAttack.html) can return.
#[derive(Debug)]
pub enum SwanlingError {
    /// Wraps a [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html).
    Io(io::Error),
    /// Wraps a [`reqwest::Error`](https://docs.rs/reqwest/*/reqwest/struct.Error.html).
    Reqwest(reqwest::Error),
    /// Wraps a ['tokio::task::JoinError'](https://tokio-rs.github.io/tokio/doc/tokio/task/struct.JoinError.html).
    TokioJoin(tokio::task::JoinError),
    //std::convert::From<tokio::task::JoinError>
    /// Failed attempt to use code that requires a compile-time feature be enabled.
    FeatureNotEnabled {
        /// The missing compile-time feature.
        feature: String,
        /// An optional explanation of the error.
        detail: String,
    },
    /// Failed to parse a hostname.
    InvalidHost {
        /// The invalid hostname that caused this error.
        host: String,
        /// An optional explanation of the error.
        detail: String,
        /// Wraps a [`url::ParseError`](https://docs.rs/url/*/url/enum.ParseError.html).
        parse_error: url::ParseError,
    },
    /// Invalid option or value specified, may only be invalid in context.
    InvalidOption {
        /// The invalid option that caused this error, may be only invalid in context.
        option: String,
        /// The invalid value that caused this error, may be only invalid in context.
        value: String,
        /// An optional explanation of the error.
        detail: String,
    },
    /// Invalid wait time specified.
    InvalidWaitTime {
        // The specified minimum wait time.
        min_wait: usize,
        // The specified maximum wait time.
        max_wait: usize,
        /// An optional explanation of the error.
        detail: String,
    },
    /// Invalid weight specified.
    InvalidWeight {
        // The specified weight.
        weight: usize,
        /// An optional explanation of the error.
        detail: String,
    },
    /// [`SwanlingAttack`](./struct.SwanlingAttack.html) has no [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html) defined.
    NoTaskSets {
        /// An optional explanation of the error.
        detail: String,
    },
}
/// Implement a helper to provide a text description of all possible types of errors.
impl SwanlingError {
    fn describe(&self) -> &str {
        match *self {
            SwanlingError::Io(_) => "io::Error",
            SwanlingError::Reqwest(_) => "reqwest::Error",
            SwanlingError::TokioJoin(_) => "tokio::task::JoinError",
            SwanlingError::FeatureNotEnabled { .. } => "required compile-time feature not enabled",
            SwanlingError::InvalidHost { .. } => "failed to parse hostname",
            SwanlingError::InvalidOption { .. } => "invalid option or value specified",
            SwanlingError::InvalidWaitTime { .. } => "invalid wait_time specified",
            SwanlingError::InvalidWeight { .. } => "invalid weight specified",
            SwanlingError::NoTaskSets { .. } => "no task sets defined",
        }
    }
}

/// Implement format trait to allow displaying errors.
impl fmt::Display for SwanlingError {
    // Implement display of error with `{}` marker.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SwanlingError::Io(ref source) => {
                write!(f, "SwanlingError: {} ({})", self.describe(), source)
            }
            SwanlingError::Reqwest(ref source) => {
                write!(f, "SwanlingError: {} ({})", self.describe(), source)
            }
            SwanlingError::TokioJoin(ref source) => {
                write!(f, "SwanlingError: {} ({})", self.describe(), source)
            }
            SwanlingError::InvalidHost {
                ref parse_error, ..
            } => write!(f, "SwanlingError: {} ({})", self.describe(), parse_error),
            _ => write!(f, "SwanlingError: {}", self.describe()),
        }
    }
}

// Define the lower level source of this error, if any.
impl std::error::Error for SwanlingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SwanlingError::Io(ref source) => Some(source),
            SwanlingError::Reqwest(ref source) => Some(source),
            SwanlingError::TokioJoin(ref source) => Some(source),
            SwanlingError::InvalidHost {
                ref parse_error, ..
            } => Some(parse_error),
            _ => None,
        }
    }
}

/// Auto-convert Reqwest errors.
impl From<reqwest::Error> for SwanlingError {
    fn from(err: reqwest::Error) -> SwanlingError {
        SwanlingError::Reqwest(err)
    }
}

/// Auto-convert IO errors.
impl From<io::Error> for SwanlingError {
    fn from(err: io::Error) -> SwanlingError {
        SwanlingError::Io(err)
    }
}

/// Auto-convert TokioJoin errors.
impl From<tokio::task::JoinError> for SwanlingError {
    fn from(err: tokio::task::JoinError) -> SwanlingError {
        SwanlingError::TokioJoin(err)
    }
}

#[derive(Clone, Debug, PartialEq)]
/// A [`SwanlingAttack`](./struct.SwanlingAttack.html) load test operates in one (and only one)
/// of the following modes.
pub enum AttackMode {
    /// During early startup before one of the following modes gets assigned.
    Undefined,
    /// A single standalone process performing a load test.
    StandAlone,
    /// The controlling process in a Regatta distributed load test.
    Manager,
    /// One of one or more working processes in a Regatta distributed load test.
    Worker,
}

#[derive(Clone, Debug, PartialEq)]
/// A [`SwanlingAttack`](./struct.SwanlingAttack.html) load test moves through each of the following
/// phases during a complete load test.
pub enum AttackPhase {
    /// No load test is running, configuration can be changed by a Controller.
    Idle,
    /// [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s are launching and beginning to generate
    /// load.
    Starting,
    /// All [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s have launched and are generating load.
    Running,
    /// [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s are stopping.
    Stopping,
    /// Exiting the load test.
    Shutdown,
}

#[derive(Clone, Debug, PartialEq)]
/// Used to define the order [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s and
/// [`SwanlingTask`](./swanling/struct.SwanlingTask.html)s are allocated.
///
/// In order to configure the scheduler, and to see examples of the different scheduler
/// variants, review the
/// [`SwanlingAttack::set_scheduler`](./struct.SwanlingAttack.html#method.set_scheduler)
/// documentation.
pub enum SwanlingScheduler {
    /// Allocate one of each available type at a time (default).
    RoundRobin,
    /// Allocate in the order and weighting defined.
    Serial,
    /// Allocate in a random order.
    Random,
}

/// Optional default values for Swanling run-time options.
#[derive(Clone, Debug, Default)]
pub struct SwanlingDefaults {
    /// An optional default host to run this load test against.
    host: Option<String>,
    /// An optional default number of users to simulate.
    users: Option<usize>,
    /// An optional default number of clients to start per second.
    hatch_rate: Option<String>,
    /// An optional default number of seconds for the test to run.
    run_time: Option<usize>,
    /// An optional default log level.
    log_level: Option<u8>,
    /// An optional default for the swanling log file name.
    swanling_log: Option<String>,
    /// An optional default value for verbosity level.
    verbose: Option<u8>,
    /// An optional default for printing running metrics.
    running_metrics: Option<usize>,
    /// An optional default for not resetting metrics after all users started.
    no_reset_metrics: Option<bool>,
    /// An optional default for not tracking metrics.
    no_metrics: Option<bool>,
    /// An optional default for not tracking task metrics.
    no_task_metrics: Option<bool>,
    /// An optional default for not displaying an error summary.
    no_error_summary: Option<bool>,
    /// An optional default for the html-formatted report file name.
    report_file: Option<String>,
    /// An optional default for the requests log file name.
    request_log: Option<String>,
    /// An optional default for the requests log file format.
    request_format: Option<SwanlingLogFormat>,
    /// An optional default for the tasks log file name.
    task_log: Option<String>,
    /// An optional default for the tasks log file format.
    task_format: Option<SwanlingLogFormat>,
    /// An optional default for the error log file name.
    error_log: Option<String>,
    /// An optional default for the error log format.
    error_format: Option<SwanlingLogFormat>,
    /// An optional default for the debug log file name.
    debug_log: Option<String>,
    /// An optional default for the debug log format.
    debug_format: Option<SwanlingLogFormat>,
    /// An optional default for not logging response body in debug log.
    no_debug_body: Option<bool>,
    /// An optional default for not enabling telnet Controller thread.
    no_telnet: Option<bool>,
    /// An optional default for not enabling WebSocket Controller thread.
    no_websocket: Option<bool>,
    /// An optional default for not auto-starting the load test.
    no_autostart: Option<bool>,
    /// An optional default for coordinated omission mitigation.
    co_mitigation: Option<SwanlingCoordinatedOmissionMitigation>,
    /// An optional default to track additional status code metrics.
    status_codes: Option<bool>,
    /// An optional default maximum requests per second.
    throttle_requests: Option<usize>,
    /// An optional default to follows base_url redirect with subsequent request.
    sticky_follow: Option<bool>,
    /// An optional default to enable Manager mode.
    manager: Option<bool>,
    /// An optional default for number of Workers to expect.
    expect_workers: Option<u16>,
    /// An optional default for Manager to ignore load test checksum.
    no_hash_check: Option<bool>,
    /// An optional default for host telnet Controller listens on.
    telnet_host: Option<String>,
    /// An optional default for port telnet Controller listens on.
    telnet_port: Option<u16>,
    /// An optional default for host WebSocket Controller listens on.
    websocket_host: Option<String>,
    /// An optional default for port WebSocket Controller listens on.
    websocket_port: Option<u16>,
    /// An optional default for host Manager listens on.
    manager_bind_host: Option<String>,
    /// An optional default for port Manager listens on.
    manager_bind_port: Option<u16>,
    /// An optional default to enable Worker mode.
    worker: Option<bool>,
    /// An optional default for host Worker connects to.
    manager_host: Option<String>,
    /// An optional default for port Worker connects to.
    manager_port: Option<u16>,
}

/// Allows the optional configuration of Swanling's defaults.
#[derive(Debug)]
pub enum SwanlingDefault {
    /// An optional default host to run this load test against.
    Host,
    /// An optional default number of users to simulate.
    Users,
    /// An optional default number of clients to start per second.
    HatchRate,
    /// An optional default number of seconds for the test to run.
    RunTime,
    /// An optional default log level.
    LogLevel,
    /// An optional default for the log file name.
    SwanlingLog,
    /// An optional default value for verbosity level.
    Verbose,
    /// An optional default for printing running metrics.
    RunningMetrics,
    /// An optional default for not resetting metrics after all users started.
    NoResetMetrics,
    /// An optional default for not tracking metrics.
    NoMetrics,
    /// An optional default for not tracking task metrics.
    NoTaskMetrics,
    /// An optional default for not displaying an error summary.
    NoErrorSummary,
    /// An optional default for the report file name.
    ReportFile,
    /// An optional default for the request log file name.
    RequestLog,
    /// An optional default for the request log file format.
    RequestFormat,
    /// An optional default for the task log file name.
    TaskLog,
    /// An optional default for the task log file format.
    TaskFormat,
    /// An optional default for the error log file name.
    ErrorLog,
    /// An optional default for the error log format.
    ErrorFormat,
    /// An optional default for the debug log file name.
    DebugLog,
    /// An optional default for the debug log format.
    DebugFormat,
    /// An optional default for not logging the response body in the debug log.
    NoDebugBody,
    /// An optional default for not enabling telnet Controller thread.
    NoTelnet,
    /// An optional default for not enabling WebSocket Controller thread.
    NoWebSocket,
    /// An optional default for coordinated omission mitigation.
    CoordinatedOmissionMitigation,
    /// An optional default for not automatically starting load test.
    NoAutoStart,
    /// An optional default to track additional status code metrics.
    StatusCodes,
    /// An optional default maximum requests per second.
    ThrottleRequests,
    /// An optional default to follows base_url redirect with subsequent request.
    StickyFollow,
    /// An optional default to enable Manager mode.
    Manager,
    /// An optional default for number of Workers to expect.
    ExpectWorkers,
    /// An optional default for Manager to ignore load test checksum.
    NoHashCheck,
    /// An optional default for host telnet Controller listens on.
    TelnetHost,
    /// An optional default for port telnet Controller listens on.
    TelnetPort,
    /// An optional default for host Websocket Controller listens on.
    WebSocketHost,
    /// An optional default for port WebSocket Controller listens on.
    WebSocketPort,
    /// An optional default for host Manager listens on.
    ManagerBindHost,
    /// An optional default for port Manager listens on.
    ManagerBindPort,
    /// An optional default to enable Worker mode.
    Worker,
    /// An optional default for host Worker connects to.
    ManagerHost,
    /// An optional default for port Worker connects to.
    ManagerPort,
}

#[derive(Debug)]
/// Internal global run state for load test.
struct SwanlingAttackRunState {
    /// A timestamp tracking when the previous [`SwanlingUser`](./swanling/struct.SwanlingUser.html)
    /// was launched.
    spawn_user_timer: std::time::Instant,
    /// How many milliseconds until the next [`SwanlingUser`](./swanling/struct.SwanlingUser.html)
    /// should be spawned.
    spawn_user_in_ms: usize,
    /// A counter tracking which [`SwanlingUser`](./swanling/struct.SwanlingUser.html) is being
    /// spawned.
    spawn_user_counter: usize,
    /// This variable accounts for time spent doing things which is then subtracted from
    /// the time sleeping to avoid an unintentional drift in events that are supposed to
    /// happen regularly.
    drift_timer: tokio::time::Instant,
    /// Unbounded sender used by all [`SwanlingUser`](./swanling/struct.SwanlingUser.html)
    /// threads to send metrics to parent.
    all_threads_metrics_tx: flume::Sender<SwanlingMetric>,
    /// Unbounded receiver used by Swanling parent to receive metrics from
    /// [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s.
    metrics_rx: flume::Receiver<SwanlingMetric>,
    /// Optional unbounded receiver for logger thread, if enabled.
    logger_handle: SwanlingLoggerJoinHandle,
    /// Optional unbounded sender from all [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s
    /// to logger thread, if enabled.
    all_threads_logger_tx: SwanlingLoggerTx,
    /// Optional receiver for all [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s from
    /// throttle thread, if enabled.
    throttle_threads_tx: Option<flume::Sender<bool>>,
    /// Optional sender for throttle thread, if enabled.
    parent_to_throttle_tx: Option<flume::Sender<bool>>,
    /// Optional channel allowing controller thread to make requests, if not disabled.
    controller_channel_rx: Option<flume::Receiver<SwanlingControllerRequest>>,
    /// Optional unbuffered writer for html-formatted report file, if enabled.
    report_file: Option<File>,
    /// A flag tracking whether or not the header has been written when the metrics
    /// log is enabled.
    metrics_header_displayed: bool,
    /// When entering the idle phase use this flag to only display a message one time.
    idle_status_displayed: bool,
    /// Collection of all [`SwanlingUser`](./swanling/struct.SwanlingUser.html) threads so they
    /// can be stopped later.
    users: Vec<tokio::task::JoinHandle<()>>,
    /// All unbounded senders to allow communication with
    /// [`SwanlingUser`](./swanling/struct.SwanlingUser.html) threads.
    user_channels: Vec<flume::Sender<SwanlingUserCommand>>,
    /// Timer tracking when to display running metrics, if enabled.
    running_metrics_timer: std::time::Instant,
    /// Boolean flag indicating if running metrics should be displayed.
    display_running_metrics: bool,
    /// Boolean flag indicating if all [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s
    /// have been spawned.
    all_users_spawned: bool,
    /// Boolean flag indicating of Swanling should shutdown after stopping a running load test.
    shutdown_after_stop: bool,
    /// Thread-safe boolean flag indicating if the [`SwanlingAttack`](./struct.SwanlingAttack.html)
    /// has been canceled.
    canceled: Arc<AtomicBool>,
    /// Optional socket used to coordinate a distributed Regatta.
    socket: Option<Socket>,
}

/// Global internal state for the load test.
#[derive(Clone)]
pub struct SwanlingAttack {
    /// An optional task that is run one time before starting SwanlingUsers and running SwanlingTaskSets.
    test_start_task: Option<SwanlingTask>,
    /// An optional task that is run one time after all SwanlingUsers have finished.
    test_stop_task: Option<SwanlingTask>,
    /// A vector containing one copy of each SwanlingTaskSet defined by this load test.
    task_sets: Vec<SwanlingTaskSet>,
    /// A weighted vector containing a SwanlingUser object for each SwanlingUser that will run during this load test.
    weighted_users: Vec<SwanlingUser>,
    /// A weighted vector containing a lightweight GaggleUser object that is sent to all Workers if running in Regatta mode.
    weighted_gaggle_users: Vec<GaggleUser>,
    /// Optional default values for Swanling run-time options.
    defaults: SwanlingDefaults,
    /// Configuration object holding options set when launching the load test.
    configuration: SwanlingConfiguration,
    /// How long (in seconds) the load test should run.
    run_time: usize,
    /// The load test operates in only one of the following modes: StandAlone, Manager, or Worker.
    attack_mode: AttackMode,
    /// Which phase the load test is currently operating in.
    attack_phase: AttackPhase,
    /// Defines the order [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s and
    /// [`SwanlingTask`](./swanling/struct.SwanlingTask.html)s are allocated.
    scheduler: SwanlingScheduler,
    /// When the load test started.
    started: Option<time::Instant>,
    /// All metrics merged together.
    metrics: SwanlingMetrics,
}
/// Swanling's internal global state.
impl SwanlingAttack {
    /// Load configuration and initialize a [`SwanlingAttack`](./struct.SwanlingAttack.html).
    ///
    /// # Example
    /// ```rust
    /// use swanling::prelude::*;
    ///
    /// let mut swanling_attack = SwanlingAttack::initialize();
    /// ```
    pub fn initialize() -> Result<SwanlingAttack, SwanlingError> {
        Ok(SwanlingAttack {
            test_start_task: None,
            test_stop_task: None,
            task_sets: Vec::new(),
            weighted_users: Vec::new(),
            weighted_gaggle_users: Vec::new(),
            defaults: SwanlingDefaults::default(),
            configuration: SwanlingConfiguration::parse_args_default_or_exit(),
            run_time: 0,
            attack_mode: AttackMode::Undefined,
            attack_phase: AttackPhase::Idle,
            scheduler: SwanlingScheduler::RoundRobin,
            started: None,
            metrics: SwanlingMetrics::default(),
        })
    }

    /// Initialize a [`SwanlingAttack`](./struct.SwanlingAttack.html) with an already loaded
    /// configuration.
    ///
    /// This is generally used by Worker instances and tests.
    ///
    /// # Example
    /// ```rust
    /// use swanling::{SwanlingAttack, SwanlingConfiguration};
    /// use gumdrop::Options;
    ///
    /// let configuration = SwanlingConfiguration::parse_args_default_or_exit();
    /// let mut swanling_attack = SwanlingAttack::initialize_with_config(configuration);
    /// ```
    pub fn initialize_with_config(
        configuration: SwanlingConfiguration,
    ) -> Result<SwanlingAttack, SwanlingError> {
        Ok(SwanlingAttack {
            test_start_task: None,
            test_stop_task: None,
            task_sets: Vec::new(),
            weighted_users: Vec::new(),
            weighted_gaggle_users: Vec::new(),
            defaults: SwanlingDefaults::default(),
            configuration,
            run_time: 0,
            attack_mode: AttackMode::Undefined,
            attack_phase: AttackPhase::Idle,
            scheduler: SwanlingScheduler::RoundRobin,
            started: None,
            metrics: SwanlingMetrics::default(),
        })
    }

    /// Optionally initialize the logger which writes to standard out and/or to
    /// a configurable log file.
    ///
    /// This method is invoked by
    /// [`SwanlingAttack.execute()`](./struct.SwanlingAttack.html#method.execute).
    pub(crate) fn initialize_logger(&self) {
        // Allow optionally controlling debug output level
        let debug_level;
        match self.configuration.verbose {
            0 => debug_level = LevelFilter::Warn,
            1 => debug_level = LevelFilter::Info,
            2 => debug_level = LevelFilter::Debug,
            _ => debug_level = LevelFilter::Trace,
        }

        // Set log level based on run-time option or default if set.
        let log_level_value = if self.configuration.log_level > 0 {
            self.configuration.log_level
        } else if let Some(default_log_level) = self.defaults.log_level {
            default_log_level
        } else {
            0
        };
        let log_level = match log_level_value {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        let swanling_log: Option<PathBuf>;
        // Use --log-file if set.
        if !self.configuration.swanling_log.is_empty() {
            swanling_log = Some(PathBuf::from(&self.configuration.swanling_log));
        }
        // Otherwise use swanling_attack.defaults.swanling_log if set.
        else if let Some(default_swanling_log) = &self.defaults.swanling_log {
            swanling_log = Some(PathBuf::from(default_swanling_log));
        }
        // Otherwise disable the log.
        else {
            swanling_log = None;
        }

        if let Some(log_to_file) = swanling_log {
            match CombinedLogger::init(vec![
                SimpleLogger::new(debug_level, Config::default()),
                WriteLogger::new(
                    log_level,
                    Config::default(),
                    std::fs::File::create(&log_to_file).unwrap(),
                ),
            ]) {
                Ok(_) => (),
                Err(e) => {
                    info!("failed to initialize CombinedLogger: {}", e);
                }
            }
            info!("Writing to log file: {}", log_to_file.display());
        } else {
            match CombinedLogger::init(vec![SimpleLogger::new(debug_level, Config::default())]) {
                Ok(_) => (),
                Err(e) => {
                    info!("failed to initialize CombinedLogger: {}", e);
                }
            }
        }

        info!("Output verbosity level: {}", debug_level);
        info!("Logfile verbosity level: {}", log_level);
    }

    /// Define the order [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s are
    /// allocated to new [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s as they are
    /// launched.
    ///
    /// By default, [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s are allocated
    /// to new [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s in a round robin style.
    /// For example, if TaskSet A has a weight of 5, TaskSet B has a weight of 3, and
    /// you launch 20 users, they will be launched in the following order:
    ///  A, B, A, B, A, B, A, A, A, B, A, B, A, B, A, A, A, B, A, B
    ///
    /// Note that the following pattern is repeated:
    ///  A, B, A, B, A, B, A, A
    ///
    /// If reconfigured to schedule serially, then they will instead be allocated in
    /// the following order:
    ///  A, A, A, A, A, B, B, B, A, A, A, A, A, B, B, B, A, A, A, A
    ///
    /// In the serial case, the following pattern is repeated:
    ///  A, A, A, A, A, B, B, B
    ///
    /// In the following example, [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s
    /// are allocated to launching [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s in a
    /// random order. This means running the test multiple times can generate
    /// different amounts of load, as depending on your weighting rules you may
    /// have a different number of [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s
    /// running each [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html) each time.
    ///
    /// # Example
    /// ```rust
    /// use swanling::prelude::*;
    ///
    /// fn main() -> Result<(), SwanlingError> {
    ///     SwanlingAttack::initialize()?
    ///         .set_scheduler(SwanlingScheduler::Random)
    ///         .register_taskset(taskset!("A Tasks")
    ///             .set_weight(5)?
    ///             .register_task(task!(a_task_1))
    ///         )
    ///         .register_taskset(taskset!("B Tasks")
    ///             .set_weight(3)?
    ///             .register_task(task!(b_task_1))
    ///         );
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn a_task_1(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     let _swanling = user.get("/foo").await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn b_task_1(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     let _swanling = user.get("/bar").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_scheduler(mut self, scheduler: SwanlingScheduler) -> Self {
        self.scheduler = scheduler;
        self
    }

    /// A load test must contain one or more [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s
    /// be registered into Swanling's global state with this method for it to run.
    ///
    /// # Example
    /// ```rust
    /// use swanling::prelude::*;
    ///
    /// fn main() -> Result<(), SwanlingError> {
    ///     SwanlingAttack::initialize()?
    ///         .register_taskset(taskset!("ExampleTasks")
    ///             .register_task(task!(example_task))
    ///         )
    ///         .register_taskset(taskset!("OtherTasks")
    ///             .register_task(task!(other_task))
    ///         );
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn example_task(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     let _swanling = user.get("/foo").await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn other_task(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     let _swanling = user.get("/bar").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn register_taskset(mut self, mut taskset: SwanlingTaskSet) -> Self {
        taskset.task_sets_index = self.task_sets.len();
        self.task_sets.push(taskset);
        self
    }

    /// Optionally define a task to run before users are started and all task sets
    /// start running. This is would generally be used to set up anything required
    /// for the load test.
    ///
    /// The [`SwanlingUser`](./swanling/struct.SwanlingUser.html) used to run the `test_start`
    /// tasks is not preserved and does not otherwise affect the subsequent
    /// [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s that run the rest of the load
    /// test. For example, if the [`SwanlingUser`](./swanling/struct.SwanlingUser.html)
    /// logs in during `test_start`, subsequent [`SwanlingUser`](./swanling/struct.SwanlingUser.html)
    /// do not retain this session and are therefor not already logged in.
    ///
    /// When running in a distributed Regatta, this task is only run one time by the
    /// Manager.
    ///
    /// # Example
    /// ```rust
    /// use swanling::prelude::*;
    ///
    /// fn main() -> Result<(), SwanlingError> {
    ///     SwanlingAttack::initialize()?
    ///         .test_start(task!(setup));
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn setup(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     // do stuff to set up load test ...
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn test_start(mut self, task: SwanlingTask) -> Self {
        self.test_start_task = Some(task);
        self
    }

    /// Optionally define a task to run after all users have finished running
    /// all defined task sets. This would generally be used to clean up anything
    /// that was specifically set up for the load test.
    ///
    /// When running in a distributed Regatta, this task is only run one time by the
    /// Manager.
    ///
    /// # Example
    /// ```rust
    /// use swanling::prelude::*;
    ///
    /// fn main() -> Result<(), SwanlingError> {
    ///     SwanlingAttack::initialize()?
    ///         .test_stop(task!(teardown));
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn teardown(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     // do stuff to tear down the load test ...
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn test_stop(mut self, task: SwanlingTask) -> Self {
        self.test_stop_task = Some(task);
        self
    }

    /// Use configured SwanlingScheduler to build out a properly weighted list of
    /// [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html)s to be assigned to
    /// [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s
    fn allocate_task_sets(&mut self) -> Vec<usize> {
        trace!("allocate_task_sets");

        let mut u: usize = 0;
        let mut v: usize;
        for task_set in &self.task_sets {
            if u == 0 {
                u = task_set.weight;
            } else {
                v = task_set.weight;
                trace!("calculating greatest common denominator of {} and {}", u, v);
                u = util::gcd(u, v);
                trace!("inner gcd: {}", u);
            }
        }
        // 'u' will always be the greatest common divisor
        debug!("gcd: {}", u);

        // Build a vector of vectors to be used to schedule users.
        let mut available_task_sets = Vec::with_capacity(self.task_sets.len());
        let mut total_task_sets = 0;
        for (index, task_set) in self.task_sets.iter().enumerate() {
            // divide by greatest common divisor so vector is as short as possible
            let weight = task_set.weight / u;
            trace!(
                "{}: {} has weight of {} (reduced with gcd to {})",
                index,
                task_set.name,
                task_set.weight,
                weight
            );
            let weighted_sets = vec![index; weight];
            total_task_sets += weight;
            available_task_sets.push(weighted_sets);
        }

        info!(
            "allocating tasks and task sets with {:?} scheduler",
            self.scheduler
        );

        // Now build the weighted list with the appropriate scheduler.
        let mut weighted_task_sets = Vec::new();
        match self.scheduler {
            SwanlingScheduler::RoundRobin => {
                // Allocate task sets round robin.
                let task_sets_len = available_task_sets.len();
                loop {
                    for (task_set_index, task_sets) in available_task_sets
                        .iter_mut()
                        .enumerate()
                        .take(task_sets_len)
                    {
                        if let Some(task_set) = task_sets.pop() {
                            debug!("allocating 1 user from TaskSet {}", task_set_index);
                            weighted_task_sets.push(task_set);
                        }
                    }
                    if weighted_task_sets.len() >= total_task_sets {
                        break;
                    }
                }
            }
            SwanlingScheduler::Serial => {
                // Allocate task sets serially in the weighted order defined.
                for (task_set_index, task_sets) in available_task_sets.iter().enumerate() {
                    debug!(
                        "allocating all {} users from TaskSet {}",
                        task_sets.len(),
                        task_set_index
                    );
                    weighted_task_sets.append(&mut task_sets.clone());
                }
            }
            SwanlingScheduler::Random => {
                // Allocate task sets randomly.
                loop {
                    let task_set = available_task_sets.choose_mut(&mut rand::thread_rng());
                    match task_set {
                        Some(set) => {
                            if let Some(s) = set.pop() {
                                weighted_task_sets.push(s);
                            }
                        }
                        None => warn!("randomly allocating a SwanlingTaskSet failed, trying again"),
                    }
                    if weighted_task_sets.len() >= total_task_sets {
                        break;
                    }
                }
            }
        }
        weighted_task_sets
    }

    /// Allocate a vector of weighted [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s.
    fn weight_task_set_users(&mut self) -> Result<Vec<SwanlingUser>, SwanlingError> {
        trace!("weight_task_set_users");

        let weighted_task_sets = self.allocate_task_sets();

        // Allocate a state for each user that will be hatched.
        info!("initializing user states...");
        let mut weighted_users = Vec::new();
        let mut user_count = 0;
        loop {
            for task_sets_index in &weighted_task_sets {
                debug!(
                    "creating user state: {} ({})",
                    weighted_users.len(),
                    task_sets_index
                );
                let base_url = swanling::get_base_url(
                    self.get_configuration_host(),
                    self.task_sets[*task_sets_index].host.clone(),
                    self.defaults.host.clone(),
                )?;
                weighted_users.push(SwanlingUser::new(
                    self.task_sets[*task_sets_index].task_sets_index,
                    base_url,
                    self.task_sets[*task_sets_index].min_wait,
                    self.task_sets[*task_sets_index].max_wait,
                    &self.configuration,
                    self.metrics.hash,
                )?);
                user_count += 1;
                // Users are required here so unwrap() is safe.
                if user_count >= self.configuration.users.unwrap() {
                    debug!("created {} weighted_users", user_count);
                    return Ok(weighted_users);
                }
            }
        }
    }

    /// Allocate a vector of weighted [`GaggleUser`](./swanling/struct.GaggleUser.html).
    fn prepare_worker_task_set_users(&mut self) -> Result<Vec<GaggleUser>, SwanlingError> {
        trace!("prepare_worker_task_set_users");

        let weighted_task_sets = self.allocate_task_sets();

        // Determine the users sent to each Worker.
        info!("preparing users for Workers...");
        let mut weighted_users = Vec::new();
        let mut user_count = 0;
        loop {
            for task_sets_index in &weighted_task_sets {
                let base_url = swanling::get_base_url(
                    self.get_configuration_host(),
                    self.task_sets[*task_sets_index].host.clone(),
                    self.defaults.host.clone(),
                )?;
                weighted_users.push(GaggleUser::new(
                    self.task_sets[*task_sets_index].task_sets_index,
                    base_url,
                    self.task_sets[*task_sets_index].min_wait,
                    self.task_sets[*task_sets_index].max_wait,
                    &self.configuration,
                    self.metrics.hash,
                ));
                user_count += 1;
                // Users are required here so unwrap() is safe.
                if user_count >= self.configuration.users.unwrap() {
                    debug!("prepared {} weighted_gaggle_users", user_count);
                    return Ok(weighted_users);
                }
            }
        }
    }

    // Configure which mode this [`SwanlingAttack`](./struct.SwanlingAttack.html)
    // will run in.
    fn set_attack_mode(&mut self) -> Result<(), SwanlingError> {
        // Determine if Manager is enabled by default.
        let manager_is_default = if let Some(value) = self.defaults.manager {
            value
        } else {
            false
        };

        // Determine if Worker is enabled by default.
        let worker_is_default = if let Some(value) = self.defaults.worker {
            value
        } else {
            false
        };

        // Don't allow Manager and Worker to both be the default.
        if manager_is_default && worker_is_default {
            return Err(SwanlingError::InvalidOption {
                option: "SwanlingDefault::Worker".to_string(),
                value: "true".to_string(),
                detail: "The SwanlingDefault::Worker default can not be set together with the SwanlingDefault::Manager default"
                    .to_string(),
            });
        }

        // Manager mode if --manager is set, or --worker is not set and Manager is default.
        if self.configuration.manager || (!self.configuration.worker && manager_is_default) {
            self.attack_mode = AttackMode::Manager;
            if self.configuration.worker {
                return Err(SwanlingError::InvalidOption {
                    option: "--worker".to_string(),
                    value: "true".to_string(),
                    detail: "The --worker flag can not be set together with the --manager flag"
                        .to_string(),
                });
            }

            if !self.configuration.debug_log.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: "--debug-file".to_string(),
                    value: self.configuration.debug_log.clone(),
                    detail:
                        "The --debug-file option can not be set together with the --manager flag."
                            .to_string(),
                });
            }
        }

        // Worker mode if --worker is set, or --manager is not set and Worker is default.
        if self.configuration.worker || (!self.configuration.manager && worker_is_default) {
            self.attack_mode = AttackMode::Worker;
            if self.configuration.manager {
                return Err(SwanlingError::InvalidOption {
                    option: "--manager".to_string(),
                    value: "true".to_string(),
                    detail: "The --manager flag can not be set together with the --worker flag."
                        .to_string(),
                });
            }

            if !self.configuration.host.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: "--host".to_string(),
                    value: self.configuration.host.clone(),
                    detail: "The --host option can not be set together with the --worker flag."
                        .to_string(),
                });
            }
        }

        // Otherwise run in standalone attack mode.
        if self.attack_mode == AttackMode::Undefined {
            self.attack_mode = AttackMode::StandAlone;

            if self.configuration.no_hash_check {
                return Err(SwanlingError::InvalidOption {
                    option: "--no-hash-check".to_string(),
                    value: self.configuration.no_hash_check.to_string(),
                    detail: "The --no-hash-check flag can not be set without also setting the --manager flag.".to_string(),
                });
            }
        }

        Ok(())
    }

    // Change from one attack_phase to another.
    fn set_attack_phase(
        &mut self,
        swanling_attack_run_state: &mut SwanlingAttackRunState,
        phase: AttackPhase,
    ) {
        // There's nothing to do if already in the specified phase.
        if self.attack_phase == phase {
            return;
        }

        // The drift timer starts at 0 any time the phase is changed.
        swanling_attack_run_state.drift_timer = tokio::time::Instant::now();

        // Optional debug output.
        info!("entering SwanlingAttack phase: {:?}", &phase);

        // Update the current phase.
        self.attack_phase = phase;
    }

    // Determine how many Workers to expect.
    fn set_expect_workers(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.expect_workers";

        // Check if --expect-workers was set.
        if self.configuration.expect_workers.is_some() {
            key = "--expect-workers";
        // Otherwise check if a custom default is set.
        } else if let Some(default_expect_workers) = self.defaults.expect_workers {
            if self.attack_mode == AttackMode::Manager {
                key = "set_default(SwanlingDefault::ExpectWorkers)";

                self.configuration.expect_workers = Some(default_expect_workers);
            }
        }

        if let Some(expect_workers) = self.configuration.expect_workers {
            // Disallow --expect-workers without --manager.
            if self.attack_mode != AttackMode::Manager {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: expect_workers.to_string(),
                    detail: format!(
                        "{} can not be set without also setting the --manager flag.",
                        key
                    ),
                });
            } else {
                // Must expect at least 1 Worker when running as Manager.
                if expect_workers < 1 {
                    return Err(SwanlingError::InvalidOption {
                        option: key.to_string(),
                        value: expect_workers.to_string(),
                        detail: format!("{} must be set to at least 1.", key),
                    });
                }

                // Must not expect more Workers than Users. Users are required at this point so
                // using unwrap() is safe.
                if expect_workers as usize > self.configuration.users.unwrap() {
                    return Err(SwanlingError::InvalidOption {
                        option: key.to_string(),
                        value: expect_workers.to_string(),
                        detail: format!(
                            "{} can not be set to a value larger than --users option.",
                            key
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    // Configure the host and port the Manager listens on.
    fn set_gaggle_host_and_port(&mut self) -> Result<(), SwanlingError> {
        // Configure manager_bind_host and manager_bind_port.
        if self.attack_mode == AttackMode::Manager {
            // Use default if run-time option not set.
            if self.configuration.manager_bind_host.is_empty() {
                self.configuration.manager_bind_host =
                    if let Some(host) = self.defaults.manager_bind_host.clone() {
                        host
                    } else {
                        "0.0.0.0".to_string()
                    }
            }

            // Use default if run-time option not set.
            if self.configuration.manager_bind_port == 0 {
                self.configuration.manager_bind_port =
                    if let Some(port) = self.defaults.manager_bind_port {
                        port
                    } else {
                        DEFAULT_PORT.to_string().parse().unwrap()
                    };
            }
        } else {
            if !self.configuration.manager_bind_host.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: "--manager-bind-host".to_string(),
                    value: self.configuration.manager_bind_host.clone(),
                    detail: "The --manager-bind-host option can not be set together with the --worker flag.".to_string(),
                });
            }

            if self.configuration.manager_bind_port != 0 {
                return Err(SwanlingError::InvalidOption {
                    option: "--manager-bind-port".to_string(),
                    value: self.configuration.manager_bind_port.to_string(),
                    detail: "The --manager-bind-port option can not be set together with the --worker flag.".to_string(),
                });
            }
        }

        // Configure manager_host and manager_port.
        if self.attack_mode == AttackMode::Worker {
            // Use default if run-time option not set.
            if self.configuration.manager_host.is_empty() {
                self.configuration.manager_host =
                    if let Some(host) = self.defaults.manager_host.clone() {
                        host
                    } else {
                        "127.0.0.1".to_string()
                    }
            }

            // Use default if run-time option not set.
            if self.configuration.manager_port == 0 {
                self.configuration.manager_port = if let Some(port) = self.defaults.manager_port {
                    port
                } else {
                    DEFAULT_PORT.to_string().parse().unwrap()
                };
            }
        } else {
            if !self.configuration.manager_host.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: "--manager-host".to_string(),
                    value: self.configuration.manager_host.clone(),
                    detail:
                        "The --manager-host option must be set together with the --worker flag."
                            .to_string(),
                });
            }

            if self.configuration.manager_port != 0 {
                return Err(SwanlingError::InvalidOption {
                    option: "--manager-port".to_string(),
                    value: self.configuration.manager_port.to_string(),
                    detail:
                        "The --manager-port option must be set together with the --worker flag."
                            .to_string(),
                });
            }
        }

        Ok(())
    }

    // Configure how many [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s to hatch.
    fn set_users(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.users";
        let mut value = 0;

        // Check if --users is set.
        if let Some(users) = self.configuration.users {
            key = "--users";
            value = users;
        // If not, check if a default for users is set.
        } else if let Some(default_users) = self.defaults.users {
            // On Worker users comes from the Manager.
            if self.attack_mode == AttackMode::Worker {
                self.configuration.users = None;
            // Otherwise use default.
            } else {
                key = "set_default(SwanlingDefault::Users)";
                value = default_users;

                self.configuration.users = Some(default_users);
            }
        // If not and if not running on Worker, default to 1.
        } else if self.attack_mode != AttackMode::Worker {
            // This should not be able to fail, but setting up debug in case the number
            // of cpus library returns an invalid number.
            key = "num_cpus::get()";
            value = num_cpus::get();

            info!("concurrent users defaulted to {} (number of CPUs)", value);

            self.configuration.users = Some(value);
        }

        // Perform bounds checking.
        if let Some(users) = self.configuration.users {
            // Setting --users with --worker is not allowed.
            if self.attack_mode == AttackMode::Worker {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!("{} can not be set together with the --worker flag.", key),
                });
            }

            // Setting users to 0 is not allowed.
            if users == 0 {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: "0".to_string(),
                    detail: "The --users option must be set to at least 1.".to_string(),
                });
            }

            // Debug output.
            info!("users = {}", users);
        }

        Ok(())
    }

    // Configure maximum run time if specified, otherwise run until canceled.
    fn set_run_time(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.run_time";
        let mut value = 0;

        // Use --run-time if set, don't allow on Worker.
        self.run_time = if !self.configuration.run_time.is_empty() {
            key = "--run-time";
            value = util::parse_timespan(&self.configuration.run_time);
            value
        // Otherwise, use default if set, but not on Worker.
        } else if let Some(default_run_time) = self.defaults.run_time {
            if self.attack_mode == AttackMode::Worker {
                0
            } else {
                key = "set_default(SwanlingDefault::RunTime)";
                value = default_run_time;
                default_run_time
            }
        }
        // Otherwise the test runs until canceled.
        else {
            0
        };

        if self.run_time > 0 {
            if self.attack_mode == AttackMode::Worker {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!("{} can not be set together with the --worker flag.", key),
                });
            }

            // Debug output.
            info!("run_time = {}", self.run_time);
        }

        Ok(())
    }

    // Configure how quickly to hatch [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s.
    fn set_hatch_rate(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.hatch_rate";
        let mut value = "".to_string();

        // Check if --hash-rate is set.
        if let Some(hatch_rate) = &self.configuration.hatch_rate {
            key = "--hatch_rate";
            value = hatch_rate.to_string();
        // If not, check if a default hatch_rate is set.
        } else if let Some(default_hatch_rate) = &self.defaults.hatch_rate {
            // On Worker hatch_rate comes from the Manager.
            if self.attack_mode == AttackMode::Worker {
                self.configuration.hatch_rate = None;
            // Otherwise use default.
            } else {
                key = "set_default(SwanlingDefault::HatchRate)";
                value = default_hatch_rate.to_string();
                self.configuration.hatch_rate = Some(default_hatch_rate.to_string());
            }
        // If not and if not running on Worker, default to 1.
        } else if self.attack_mode != AttackMode::Worker {
            // This should not be able to fail, but setting up debug in case a later
            // change introduces the potential for failure.
            key = "Swanling default";
            value = "1".to_string();
            self.configuration.hatch_rate = Some(value.to_string());
        }

        // Verbose output.
        if let Some(hatch_rate) = &self.configuration.hatch_rate {
            // Setting --hatch-rate with --worker is not allowed.
            if self.attack_mode == AttackMode::Worker {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value,
                    detail: format!("{} can not be set together with the --worker flag.", key),
                });
            }

            // Setting --hatch-rate of 0 is not allowed.
            if hatch_rate.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value,
                    detail: format!("{} must be set to at least 1.", key),
                });
            }

            // Debug output.
            info!("hatch_rate = {}", hatch_rate);
        }

        Ok(())
    }

    // Configure the coordinated omission mitigation strategy.
    fn set_coordinated_omission(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.coordinated_omission";
        let mut value = Some(SwanlingCoordinatedOmissionMitigation::Disabled);

        if self.configuration.co_mitigation.is_some() {
            key = "--co-mitigation";
            value = self.configuration.co_mitigation.clone();
        }

        // Use default for co_mitigation if set and not on Worker.
        if self.configuration.co_mitigation.is_none() {
            if let Some(default_co_mitigation) = self.defaults.co_mitigation.as_ref() {
                // In Gaggles, co_mitigation is only set on Manager.
                if self.attack_mode != AttackMode::Worker {
                    key = "set_default(SwanlingDefault::CoordinatedOmissionMitigation)";
                    value = Some(default_co_mitigation.clone());

                    self.configuration.co_mitigation = Some(default_co_mitigation.clone());
                }
            }
        }

        // Otherwise default to SwanlingCoordinaatedOmissionMitigation::Average.
        if self.configuration.co_mitigation.is_none() && self.attack_mode != AttackMode::Worker {
            self.configuration.co_mitigation = value.clone();
        }

        if let Some(co_mitigation) = self.configuration.co_mitigation.as_ref() {
            // Setting --co-mitigation with --worker is not allowed.
            if self.attack_mode == AttackMode::Worker {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: format!("{} can not be set together with the --worker flag.", key),
                });
            }

            // Setting --co-mitigation with --no-metrics is not allowed.
            if self.configuration.no_metrics {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: format!(
                        "{} can not be set together with the --no-metrics flag.",
                        key
                    ),
                });
            }

            if co_mitigation != &SwanlingCoordinatedOmissionMitigation::Disabled
                && self.scheduler == SwanlingScheduler::Random
            {
                // Coordinated Omission Mitigation is not possible together with the random scheduler,
                // as it's impossible to calculate an accurate request cadence.
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: format!(
                        "{} can not be set together with SwanlingScheduler::Random.",
                        key
                    ),
                });
            }

            info!(
                "co_mitigation = {:?}",
                self.configuration.co_mitigation.as_ref().unwrap()
            );
        }

        Ok(())
    }

    // Configure maximum requests per second if throttle enabled.
    fn set_throttle_requests(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.throttle_requests";
        let mut value = 0;

        if self.configuration.throttle_requests > 0 {
            key = "--throttle-requests";
            value = self.configuration.throttle_requests;
        }

        // Use default for throttle_requests if set and not on Worker.
        if self.configuration.throttle_requests == 0 {
            if let Some(default_throttle_requests) = self.defaults.throttle_requests {
                // In Gaggles, throttle_requests is only set on Worker.
                if self.attack_mode != AttackMode::Manager {
                    key = "set_default(SwanlingDefault::ThrottleRequests)";
                    value = default_throttle_requests;

                    self.configuration.throttle_requests = default_throttle_requests;
                }
            }
        }

        if self.configuration.throttle_requests > 0 {
            // Setting --throttle-requests with --worker is not allowed.
            if self.attack_mode == AttackMode::Manager {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!("{} can not be set together with the --manager flag.", key),
                });
            }

            // Be sure throttle_requests is in allowed range.
            if self.configuration.throttle_requests == 0 {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!("{} must be set to at least 1 request per second.", key),
                });
            } else if self.configuration.throttle_requests > 1_000_000 {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!(
                        "{} can not be set to more than 1,000,000 requests per second.",
                        key
                    ),
                });
            }

            info!(
                "throttle_requests = {}",
                self.configuration.throttle_requests
            );
        }

        Ok(())
    }

    // Determine if `no_reset_statics` is enabled.
    fn set_no_reset_metrics(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_reset_metrics";
        let mut value = false;

        if self.configuration.no_reset_metrics {
            key = "--no-reset-metrics";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            if let Some(default_no_reset_metrics) = self.defaults.no_reset_metrics {
                key = "set_default(SwanlingDefault::NoResetMetrics)";
                value = default_no_reset_metrics;

                // Optionally set default.
                self.configuration.no_reset_metrics = default_no_reset_metrics;
            }
        }

        // Setting --no-reset-metrics with --worker is not allowed.
        if self.configuration.no_reset_metrics && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        Ok(())
    }

    // Determine if the `--status-codes` flag is enabled.
    fn set_status_codes(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.status_codes";
        let mut value = false;

        if self.configuration.status_codes {
            key = "--status-codes";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            if let Some(default_status_codes) = self.defaults.status_codes {
                key = "set_default(SwanlingDefault::StatusCodes)";
                value = default_status_codes;

                // Optionally set default.
                self.configuration.status_codes = default_status_codes;
            }
        }

        // Setting --status-codes with --worker is not allowed.
        if self.configuration.status_codes && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        Ok(())
    }

    // Determine if the `--running-metrics` flag is enabled.
    fn set_running_metrics(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.running_metrics";
        let mut value = 0;

        if let Some(running_metrics) = self.configuration.running_metrics {
            key = "--running-metrics";
            value = running_metrics;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            // Optionally set default.
            if let Some(default_running_metrics) = self.defaults.running_metrics {
                key = "set_default(SwanlingDefault::RunningMetrics)";
                value = default_running_metrics;

                self.configuration.running_metrics = Some(default_running_metrics);
            }
        }

        // Setting --running-metrics with --worker is not allowed.
        if let Some(running_metrics) = self.configuration.running_metrics {
            if self.attack_mode == AttackMode::Worker {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!("{} can not be set together with the --worker flag.", key),
                });
            }

            if running_metrics > 0 {
                info!("running_metrics = {}", running_metrics);
            }
        }

        Ok(())
    }

    // Determine if the `--no-task-metrics` flag is enabled.
    fn set_no_task_metrics(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_task_metrics";
        let mut value = false;

        if self.configuration.no_task_metrics {
            key = "--no-task-metrics";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            // Optionally set default.
            if let Some(default_no_task_metrics) = self.defaults.no_task_metrics {
                key = "set_default(SwanlingDefault::NoTaskMetrics)";
                value = default_no_task_metrics;

                self.configuration.no_task_metrics = default_no_task_metrics;
            }
        }

        // Setting --no-task-metrics with --worker is not allowed.
        if self.configuration.no_task_metrics && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        Ok(())
    }

    // Determine if the `--no-error-summary` flag is enabled.
    fn set_no_error_summary(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_error_summary";
        let mut value = false;

        if self.configuration.no_error_summary {
            key = "--no-error-summary";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            // Optionally set default.
            if let Some(default_no_error_summary) = self.defaults.no_error_summary {
                key = "set_default(SwanlingDefault::NoErrorSummary)";
                value = default_no_error_summary;

                self.configuration.no_error_summary = default_no_error_summary;
            }
        }

        // Setting --no-error-summary with --worker is not allowed.
        if self.configuration.no_error_summary && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        Ok(())
    }

    // Determine if the `--no-metrics` flag is enabled.
    fn set_no_metrics(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_metrics";
        let mut value = false;

        if self.configuration.no_metrics {
            key = "--no-metrics";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            // Optionally set default.
            if let Some(default_no_metrics) = self.defaults.no_metrics {
                key = "set_default(SwanlingDefault::NoMetrics)";
                value = default_no_metrics;

                self.configuration.no_metrics = default_no_metrics;
            }
        }

        // Setting --no-metrics with --worker is not allowed.
        if self.configuration.no_metrics && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        // Don't allow overhead of collecting metrics unless we're printing them.
        if self.configuration.no_metrics {
            if self.configuration.status_codes {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!(
                        "{} can not be set together with the --status-codes flag.",
                        key
                    ),
                });
            }

            // Don't allow overhead of collecting metrics unless we're printing them.
            if self.configuration.running_metrics.is_some() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!(
                        "{} can not be set together with the --running_metrics option.",
                        key
                    ),
                });
            }

            // There is nothing to log if metrics are disabled.
            if !self.configuration.request_log.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!(
                        "{} can not be set together with the --requests-file option.",
                        key
                    ),
                });
            }
        }

        Ok(())
    }

    // Determine if the `--sticky-follow` flag is enabled.
    fn set_sticky_follow(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.sticky_follow";
        let mut value = false;

        if self.configuration.sticky_follow {
            key = "--sticky-follow";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            // Optionally set default.
            if let Some(default_sticky_follow) = self.defaults.sticky_follow {
                key = "set_default(SwanlingDefault::StickyFollow)";
                value = default_sticky_follow;

                self.configuration.sticky_follow = default_sticky_follow;
            }
        }

        if self.configuration.sticky_follow && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        Ok(())
    }

    #[cfg(feature = "gaggle")]
    // Determine if `--no-hash-check` flag is enabled.
    fn set_no_hash_check(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_hash_check";
        let mut value = false;

        if self.configuration.no_hash_check {
            key = "--no-hash-check";
            value = true;
        // If not otherwise set and not Worker, check if there's a default.
        } else if self.attack_mode != AttackMode::Worker {
            // Optionally set default.
            if let Some(default_no_hash_check) = self.defaults.no_hash_check {
                key = "set_default(SwanlingDefault::NoHashCheck)";
                value = default_no_hash_check;

                self.configuration.no_hash_check = default_no_hash_check;
            }
        }

        if self.configuration.no_hash_check && self.attack_mode == AttackMode::Worker {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --worker flag.", key),
            });
        }

        Ok(())
    }

    // If enabled, returns the path of the report_file, otherwise returns None.
    fn get_report_file_path(&mut self) -> Option<String> {
        // If metrics are disabled, or running in Manager mode, there is no
        // report file, exit immediately.
        if self.configuration.no_metrics || self.attack_mode == AttackMode::Manager {
            return None;
        }

        // If --report-file is set, return it.
        if !self.configuration.report_file.is_empty() {
            return Some(self.configuration.report_file.to_string());
        }

        // If SwanlingDefault::ReportFile is set, return it.
        if let Some(default_report_file) = &self.defaults.report_file {
            return Some(default_report_file.to_string());
        }

        // Otherwise there is no report file.
        None
    }

    // Configure requests log format.
    fn set_request_format(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.request_format";
        let mut value = Some(SwanlingLogFormat::Json);

        if self.configuration.request_format.is_some() {
            key = "--requests-format";
            value = self.configuration.request_format.clone();
        } else if let Some(default_request_format) = self.defaults.request_format.as_ref() {
            // In Gaggles, request_format is only set on Worker.
            if self.attack_mode != AttackMode::Manager {
                key = "set_default(SwanlingDefault::RequestFormat)";
                value = Some(default_request_format.clone());
                self.configuration.request_format = Some(default_request_format.clone());
            }
        }

        // Otherwise default to SwanlingLogFormat::Json.
        if !self.configuration.request_log.is_empty()
            && self.configuration.request_format.is_none()
            && self.attack_mode != AttackMode::Manager
        {
            self.configuration.request_format = value.clone();
        }

        if self.configuration.request_format.is_some() {
            // Log format isn't relevant if metrics aren't enabled.
            if self.configuration.no_metrics {
                return Err(SwanlingError::InvalidOption {
                    option: "--no-metrics".to_string(),
                    value: "true".to_string(),
                    detail: "The --no-metrics flag can not be set together with the --requests-format option.".to_string(),
                });
            }
            // Log format isn't relevant if log not enabled.
            else if self.configuration.request_log.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: "The --requests-file option must be set together with the --requests-format option.".to_string(),
                });
            }
        }

        Ok(())
    }

    // Configure tasks log format.
    fn set_task_format(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.task_format";
        let mut value = Some(SwanlingLogFormat::Json);

        if self.configuration.task_format.is_some() {
            key = "--tasks-format";
            value = self.configuration.task_format.clone();
        } else if let Some(default_task_format) = self.defaults.task_format.as_ref() {
            // In Gaggles, task_format is only set on Worker.
            if self.attack_mode != AttackMode::Manager {
                key = "set_default(SwanlingDefault::TaskFormat)";
                value = Some(default_task_format.clone());
                self.configuration.task_format = Some(default_task_format.clone());
            }
        }

        // Otherwise default to SwanlingLogFormat::Json.
        if !self.configuration.task_log.is_empty()
            && self.configuration.task_format.is_none()
            && self.attack_mode != AttackMode::Manager
        {
            self.configuration.task_format = value.clone();
        }

        if self.configuration.task_format.is_some() {
            // Log format isn't relevant if metrics aren't enabled.
            if self.configuration.no_metrics {
                return Err(SwanlingError::InvalidOption {
                    option: "--no-metrics".to_string(),
                    value: "true".to_string(),
                    detail: "The --no-metrics flag can not be set together with the --tasks-format option.".to_string(),
                });
            }
            // Log format isn't relevant if log not enabled.
            else if self.configuration.task_log.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: "The --tasks-file option must be set together with the --tasks-format option.".to_string(),
                });
            }
        }

        Ok(())
    }

    // Configure tasks log format.
    fn set_error_format(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.error_format";
        let mut value = Some(SwanlingLogFormat::Json);

        if self.configuration.error_format.is_some() {
            key = "--error-format";
            value = self.configuration.error_format.clone();
        } else if let Some(default_error_format) = self.defaults.error_format.as_ref() {
            // In Gaggles, error_format is only set on Worker.
            if self.attack_mode != AttackMode::Manager {
                key = "set_default(SwanlingDefault::ErrorFormat)";
                value = Some(default_error_format.clone());
                self.configuration.error_format = Some(default_error_format.clone());
            }
        }

        // Otherwise default to SwanlingLogFormat::Json.
        if !self.configuration.error_log.is_empty()
            && self.configuration.error_format.is_none()
            && self.attack_mode != AttackMode::Manager
        {
            self.configuration.error_format = value.clone();
        }

        if self.configuration.error_format.is_some() {
            // Log format isn't relevant if metrics aren't enabled.
            if self.configuration.no_metrics {
                return Err(SwanlingError::InvalidOption {
                    option: "--no-metrics".to_string(),
                    value: "true".to_string(),
                    detail: "The --no-metrics flag can not be set together with the --error-format option.".to_string(),
                });
            }
            // Log format isn't relevant if log not enabled.
            else if self.configuration.error_log.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: "The --error-file option must be set together with the --error-format option.".to_string(),
                });
            }
        }

        Ok(())
    }

    // Configure debug log format.
    fn set_debug_format(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.debug_format";
        let mut value = Some(SwanlingLogFormat::Json);

        if self.configuration.debug_format.is_some() {
            key = "--debug-format";
            value = self.configuration.debug_format.clone();
        } else if let Some(default_debug_format) = self.defaults.debug_format.as_ref() {
            // In Gaggles, debug_format is only set on Worker.
            if self.attack_mode != AttackMode::Manager {
                key = "set_default(SwanlingDefault::DebugFormat)";
                value = Some(default_debug_format.clone());
                self.configuration.debug_format = Some(default_debug_format.clone());
            }
        }

        // Otherwise default to SwanlingLogFormat::Json.
        if !self.configuration.debug_log.is_empty()
            && self.configuration.debug_format.is_none()
            && self.attack_mode != AttackMode::Manager
        {
            self.configuration.debug_format = value.clone();
        }

        if self.configuration.debug_format.is_some() {
            // Log format isn't relevant if log not enabled.
            if self.configuration.debug_log.is_empty() {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: format!("{:?}", value),
                    detail: "The --debug-file option must be set together with the --debug-format option.".to_string(),
                });
            }
        }

        Ok(())
    }

    // Configure whether or not to enable the telnet Controller. Always disable when in Regatta mode.
    fn set_no_telnet(&mut self) {
        // Currently Gaggles are not Controller-aware, force disable.
        if [AttackMode::Manager, AttackMode::Worker].contains(&self.attack_mode) {
            self.configuration.no_telnet = true;
        // Otherwise, if --no-telnet flag not set, respect default if configured.
        } else if !self.configuration.no_telnet {
            if let Some(default_no_telnet) = self.defaults.no_telnet {
                self.configuration.no_telnet = default_no_telnet;
            }
        }
    }

    // Configure whether or not to enable the WebSocket Controller. Always disable when in Regatta mode.
    fn set_no_websocket(&mut self) {
        // Currently Gaggles are not Controller-aware, force disable.
        if [AttackMode::Manager, AttackMode::Worker].contains(&self.attack_mode) {
            self.configuration.no_websocket = true;
        // Otherwise, if --no-websocket flag not set, respect default if configured.
        } else if !self.configuration.no_websocket {
            if let Some(default_no_telnet) = self.defaults.no_telnet {
                self.configuration.no_websocket = default_no_telnet;
            }
        }
    }

    // Configure whether or not to autostart the load test.
    fn set_no_autostart(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_autostart";
        let mut value = false;

        // Currently Gaggles are not Controller-aware.
        if self.configuration.no_autostart {
            key = "--no-autostart";
            value = true;
        // Otherwise set default if configured.
        } else if let Some(default_no_autostart) = self.defaults.no_autostart {
            key = "set_default(SwanlingDefault::NoAutoStart)";
            value = default_no_autostart;

            self.configuration.no_autostart = default_no_autostart;
        }

        if self.configuration.no_autostart {
            // Can't disable autostart in Regatta mode.
            if [AttackMode::Manager, AttackMode::Worker].contains(&self.attack_mode) {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!(
                        "{} can not be set together with the --manager or --worker flags.",
                        key
                    ),
                });
            }

            // Can't disable autostart if there's no Controller enabled.
            if self.configuration.no_telnet && self.configuration.no_websocket {
                return Err(SwanlingError::InvalidOption {
                    option: key.to_string(),
                    value: value.to_string(),
                    detail: format!("{} can not be set together with both the --no-telnet and --no-websocket flags.", key),
                });
            }
        }

        Ok(())
    }

    // Configure whether to log response body.
    fn set_no_debug_body(&mut self) -> Result<(), SwanlingError> {
        // Track how value gets set so we can return a meaningful error if necessary.
        let mut key = "configuration.no_debug_body";
        let mut value = false;

        if self.configuration.no_debug_body {
            key = "--no-debug-body";
            value = true;
        // If not otherwise set and not Manager, check if there's a default.
        } else if self.attack_mode != AttackMode::Manager {
            // Optionally set default.
            if let Some(default_no_debug_body) = self.defaults.no_debug_body {
                key = "set_default(SwanlingDefault::NoDebugBody)";
                value = default_no_debug_body;

                self.configuration.no_debug_body = default_no_debug_body;
            }
        }

        if self.configuration.no_debug_body && self.attack_mode == AttackMode::Manager {
            return Err(SwanlingError::InvalidOption {
                option: key.to_string(),
                value: value.to_string(),
                detail: format!("{} can not be set together with the --manager flag.", key),
            });
        }

        Ok(())
    }

    /// Execute the [`SwanlingAttack`](./struct.SwanlingAttack.html) load test.
    ///
    /// # Example
    /// ```rust
    /// use swanling::prelude::*;
    ///
    /// fn main() -> Result<(), SwanlingError> {
    ///     let _swanling_metrics = SwanlingAttack::initialize()?
    ///         .register_taskset(taskset!("ExampleTasks")
    ///             .register_task(task!(example_task).set_weight(2)?)
    ///             .register_task(task!(another_example_task).set_weight(3)?)
    ///             // Swanling must run against a host, point to localhost so test starts.
    ///             .set_host("http://localhost")
    ///         )
    ///         // Exit after one second so test doesn't run forever.
    ///         .set_default(SwanlingDefault::RunTime, 1)?
    ///         .execute()?;
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn example_task(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     let _swanling = user.get("/foo").await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn another_example_task(user: &SwanlingUser) -> SwanlingTaskResult {
    ///     let _swanling = user.get("/bar").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn execute(mut self) -> Result<SwanlingMetrics, SwanlingError> {
        // If version flag is set, display package name and version and exit.
        if self.configuration.version {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        // At least one task set is required.
        if self.task_sets.is_empty() {
            return Err(SwanlingError::NoTaskSets {
                detail: "No task sets are defined.".to_string(),
            });
        }

        // Display task sets and tasks, then exit.
        if self.configuration.list {
            println!("Available tasks:");
            for task_set in self.task_sets {
                println!(" - {} (weight: {})", task_set.name, task_set.weight);
                for task in task_set.tasks {
                    println!("    o {} (weight: {})", task.name, task.weight);
                }
            }
            std::process::exit(0);
        }

        // Configure loggers.
        self.configuration.configure_loggers(&self.defaults);

        // Initialize logger.
        self.initialize_logger();

        // Configure run mode (StandAlone, Worker, Manager).
        self.set_attack_mode()?;

        // Determine whether or not to enable the telnet Controller.
        self.set_no_telnet();

        // Determine whether or not to enable the WebSocket Controller.
        self.set_no_websocket();

        // Determine whether or not to autostart load test.
        self.set_no_autostart()?;

        // Configure number of users to simulate.
        self.set_users()?;

        // Configure expect_workers if running in Manager attack mode.
        self.set_expect_workers()?;

        // Configure host and ports if running in a Regatta distributed load test.
        self.set_gaggle_host_and_port()?;

        // Configure how long to run.
        self.set_run_time()?;

        // Configure how many users to hatch per second.
        self.set_hatch_rate()?;

        // Configure the requests log format.
        self.set_request_format()?;

        // Configure the tasks log format.
        self.set_task_format()?;

        // Configure the tasks log format.
        self.set_error_format()?;

        // Configure the debug log format.
        self.set_debug_format()?;

        // Determine whether or not to log response body.
        self.set_no_debug_body()?;

        // Configure coordinated ommission mitigation strategy.
        self.set_coordinated_omission()?;

        // Configure throttle if enabled.
        self.set_throttle_requests()?;

        // Configure status_codes flag.
        self.set_status_codes()?;

        // Configure running_metrics flag.
        self.set_running_metrics()?;

        // Configure no_reset_metrics flag.
        self.set_no_reset_metrics()?;

        // Configure no_task_metrics flag.
        self.set_no_task_metrics()?;

        // Configure no_error_summary flag.
        self.set_no_error_summary()?;

        // Configure no_metrics flag.
        self.set_no_metrics()?;

        // Configure sticky_follow flag.
        self.set_sticky_follow()?;

        // Configure no_hash_check flag.
        #[cfg(feature = "gaggle")]
        self.set_no_hash_check()?;

        // Confirm there's either a global host, or each task set has a host defined.
        if let Err(e) = self.validate_host() {
            if self.configuration.no_autostart {
                info!("host must be configured via Controller before starting load test");
            } else {
                // If auto-starting, host must be valid.
                return Err(e);
            }
        } else {
            info!("global host configured: {}", self.configuration.host);
            self.prepare_load_test()?;
        }

        // Calculate a unique hash for the current load test.
        let mut s = DefaultHasher::new();
        self.task_sets.hash(&mut s);
        self.metrics.hash = s.finish();
        debug!("hash: {}", self.metrics.hash);

        // Start swanling in manager mode.
        if self.attack_mode == AttackMode::Manager {
            #[cfg(feature = "gaggle")]
            {
                let rt = Runtime::new().unwrap();
                self = rt.block_on(manager::manager_main(self));
            }

            #[cfg(not(feature = "gaggle"))]
            {
                return Err(SwanlingError::FeatureNotEnabled {
                    feature: "gaggle".to_string(), detail: "Load test must be recompiled with `--features gaggle` to start in manager mode.".to_string()
                });
            }
        }
        // Start swanling in worker mode.
        else if self.attack_mode == AttackMode::Worker {
            #[cfg(feature = "gaggle")]
            {
                let rt = Runtime::new().unwrap();
                self = rt.block_on(worker::worker_main(&self));
            }

            #[cfg(not(feature = "gaggle"))]
            {
                return Err(SwanlingError::FeatureNotEnabled {
                    feature: "gaggle".to_string(),
                    detail: "Load test must be recompiled with `--features gaggle` to start in worker mode.".to_string(),
                });
            }
        }
        // Start swanling in single-process mode.
        else {
            let rt = Runtime::new().unwrap();
            self = rt.block_on(self.start_attack(None))?;
        }

        Ok(self.metrics)
    }

    // Returns OK(()) if there's a valid host, SwanlingError with details if not.
    fn validate_host(&mut self) -> Result<(), SwanlingError> {
        if self.configuration.host.is_empty() {
            for task_set in &self.task_sets {
                match &task_set.host {
                    Some(h) => {
                        if util::is_valid_host(h).is_ok() {
                            info!("host for {} configured: {}", task_set.name, h);
                        }
                    }
                    None => match &self.defaults.host {
                        Some(h) => {
                            if util::is_valid_host(h).is_ok() {
                                info!("host for {} configured: {}", task_set.name, h);
                            }
                        }
                        None => {
                            if self.attack_mode != AttackMode::Worker {
                                return Err(SwanlingError::InvalidOption {
                                    option: "--host".to_string(),
                                    value: "".to_string(),
                                    detail: format!("A host must be defined via the --host option, the SwanlingAttack.set_default() function, or the SwanlingTaskSet.set_host() function (no host defined for {}).", task_set.name)
                                });
                            }
                        }
                    },
                }
            }
        }
        Ok(())
    }

    // Create and schedule SwanlingUsers. This requires that the host that will be load tested
    // has been configured.
    fn prepare_load_test(&mut self) -> Result<(), SwanlingError> {
        // If not on a Worker, be sure a valid host has been defined before building configuration.
        if self.attack_mode != AttackMode::Worker {
            self.validate_host()?;
        }

        // Apply weights to tasks in each task set.
        for task_set in &mut self.task_sets {
            let (weighted_on_start_tasks, weighted_tasks, weighted_on_stop_tasks) =
                allocate_tasks(&task_set, &self.scheduler);
            task_set.weighted_on_start_tasks = weighted_on_start_tasks;
            task_set.weighted_tasks = weighted_tasks;
            task_set.weighted_on_stop_tasks = weighted_on_stop_tasks;
            debug!(
                "weighted {} on_start: {:?} tasks: {:?} on_stop: {:?}",
                task_set.name,
                task_set.weighted_on_start_tasks,
                task_set.weighted_tasks,
                task_set.weighted_on_stop_tasks
            );
        }

        if self.attack_mode != AttackMode::Worker {
            // Stand-alone and Manager processes can display metrics.
            if !self.configuration.no_metrics {
                self.metrics.display_metrics = true;
            }

            if self.attack_mode == AttackMode::StandAlone {
                // Allocate a state for each of the users we are about to start.
                self.weighted_users = self.weight_task_set_users()?;
            } else if self.attack_mode == AttackMode::Manager {
                // Build a list of users to be allocated on Workers.
                self.weighted_gaggle_users = self.prepare_worker_task_set_users()?;
            }
        }

        Ok(())
    }

    /// Helper to wrap configured host in `Option<>` if set.
    fn get_configuration_host(&self) -> Option<String> {
        if self.configuration.host.is_empty() {
            None
        } else {
            Some(self.configuration.host.to_string())
        }
    }

    // Helper to spawn a throttle thread if configured. The throttle thread opens
    // a bounded channel to control how quickly [`SwanlingUser`](./swanling/struct.SwanlingUser.html)
    // threads can make requests.
    async fn setup_throttle(
        &self,
    ) -> (
        // A channel used by [`SwanlingUser`](./swanling/struct.SwanlingUser.html)s to throttle requests.
        Option<flume::Sender<bool>>,
        // A channel used by parent to tell throttle the load test is complete.
        Option<flume::Sender<bool>>,
    ) {
        // If the throttle isn't enabled, return immediately.
        if self.configuration.throttle_requests == 0 {
            return (None, None);
        }

        // Create a bounded channel allowing single-sender multi-receiver to throttle
        // [`SwanlingUser`](./swanling/struct.SwanlingUser.html) threads.
        let (all_threads_throttle, throttle_receiver): (
            flume::Sender<bool>,
            flume::Receiver<bool>,
        ) = flume::bounded(self.configuration.throttle_requests);

        // Create a channel allowing the parent to inform the throttle thread when the
        // load test is finished. Even though we only send one message, we can't use a
        // oneshot channel as we don't want to block waiting for a message.
        let (parent_to_throttle_tx, throttle_rx) = flume::bounded(1);

        // Launch a new thread for throttling, no need to rejoin it.
        let _ = Some(tokio::spawn(throttle::throttle_main(
            self.configuration.throttle_requests,
            throttle_receiver,
            throttle_rx,
        )));

        let sender = all_threads_throttle.clone();
        // We start from 1 instead of 0 to intentionally fill all but one slot in the
        // channel to avoid a burst of traffic during startup. The channel then provides
        // an implementation of the leaky bucket algorithm as a queue. Requests have to
        // add a token to the bucket before making a request, and are blocked until this
        // throttle thread "leaks out" a token thereby creating space. More information
        // can be found at: https://en.wikipedia.org/wiki/Leaky_bucket
        for _ in 1..self.configuration.throttle_requests {
            let _ = sender.send_async(true).await;
        }

        (Some(all_threads_throttle), Some(parent_to_throttle_tx))
    }

    // Helper to optionally spawn a telnet and/or WebSocket Controller thread. The Controller
    // threads share a control channel, allowing it to send requests to the parent process. When
    // a response is required, the Controller will also send a one-shot channel allowing a direct
    // reply.
    async fn setup_controllers(&mut self) -> Option<flume::Receiver<SwanlingControllerRequest>> {
        // If the telnet controller is disabled, return immediately.
        if self.configuration.no_telnet && self.configuration.no_websocket {
            return None;
        }

        // Create an unbounded channel for controller threads to send requests to the parent
        // process.
        let (all_threads_controller_request_tx, controller_request_rx): (
            flume::Sender<SwanlingControllerRequest>,
            flume::Receiver<SwanlingControllerRequest>,
        ) = flume::unbounded();

        // Configured telnet Controller if not disabled.
        if !self.configuration.no_telnet {
            // Configure telnet_host, using default if run-time option is not set.
            if self.configuration.telnet_host.is_empty() {
                self.configuration.telnet_host =
                    if let Some(host) = self.defaults.telnet_host.clone() {
                        host
                    } else {
                        "0.0.0.0".to_string()
                    }
            }

            // Then configure telnet_port, using default if run-time option is not set.
            if self.configuration.telnet_port == 0 {
                self.configuration.telnet_port = if let Some(port) = self.defaults.telnet_port {
                    port
                } else {
                    DEFAULT_TELNET_PORT.to_string().parse().unwrap()
                };
            }

            // Spawn the initial controller thread to allow real-time control of the load test.
            // There is no need to rejoin this thread when the load test ends.
            let _ = Some(tokio::spawn(controller::controller_main(
                self.configuration.clone(),
                all_threads_controller_request_tx.clone(),
                SwanlingControllerProtocol::Telnet,
            )));
        }

        // Configured WebSocket Controller if not disabled.
        if !self.configuration.no_websocket {
            // Configure websocket_host, using default if run-time option is not set.
            if self.configuration.websocket_host.is_empty() {
                self.configuration.websocket_host =
                    if let Some(host) = self.defaults.websocket_host.clone() {
                        host
                    } else {
                        "0.0.0.0".to_string()
                    }
            }

            // Then configure websocket_port, using default if run-time option is not set.
            if self.configuration.websocket_port == 0 {
                self.configuration.websocket_port = if let Some(port) = self.defaults.websocket_port
                {
                    port
                } else {
                    DEFAULT_WEBSOCKET_PORT.to_string().parse().unwrap()
                };
            }

            // Spawn the initial controller thread to allow real-time control of the load test.
            // There is no need to rejoin this thread when the load test ends.
            let _ = Some(tokio::spawn(controller::controller_main(
                self.configuration.clone(),
                all_threads_controller_request_tx,
                SwanlingControllerProtocol::WebSocket,
            )));
        }

        // Return the parent end of the Controller channel.
        Some(controller_request_rx)
    }

    // Prepare an asynchronous file writer for `report_file` (if enabled).
    async fn prepare_report_file(&mut self) -> Result<Option<File>, SwanlingError> {
        if let Some(report_file_path) = self.get_report_file_path() {
            Ok(Some(File::create(&report_file_path).await?))
        } else {
            Ok(None)
        }
    }

    // Invoke `test_start` tasks if existing.
    async fn run_test_start(&self) -> Result<(), SwanlingError> {
        // Initialize per-user states.
        if self.attack_mode != AttackMode::Worker {
            // First run global test_start_task, if defined.
            match &self.test_start_task {
                Some(t) => {
                    info!("running test_start_task");
                    // Create a one-time-use User to run the test_start_task.
                    let base_url = swanling::get_base_url(
                        self.get_configuration_host(),
                        None,
                        self.defaults.host.clone(),
                    )?;
                    let user = SwanlingUser::single(base_url, &self.configuration)?;
                    let function = &t.function;
                    let _ = function(&user).await;
                }
                // No test_start_task defined, nothing to do.
                None => (),
            }
        }

        Ok(())
    }

    // Invoke `test_stop` tasks if existing.
    async fn run_test_stop(&self) -> Result<(), SwanlingError> {
        // Initialize per-user states.
        if self.attack_mode != AttackMode::Worker {
            // First run global test_stop_task, if defined.
            match &self.test_stop_task {
                Some(t) => {
                    info!("running test_stop_task");
                    // Create a one-time-use User to run the test_stop_task.
                    let base_url = swanling::get_base_url(
                        self.get_configuration_host(),
                        None,
                        self.defaults.host.clone(),
                    )?;
                    let user = SwanlingUser::single(base_url, &self.configuration)?;
                    let function = &t.function;
                    let _ = function(&user).await;
                }
                // No test_stop_task defined, nothing to do.
                None => (),
            }
        }

        Ok(())
    }

    // Create a SwanlingAttackRunState object and do all initialization required
    // to start a [`SwanlingAttack`](./struct.SwanlingAttack.html).
    async fn initialize_attack(
        &mut self,
        socket: Option<Socket>,
    ) -> Result<SwanlingAttackRunState, SwanlingError> {
        trace!("initialize_attack");

        // Create a single channel used to send metrics from SwanlingUser threads
        // to parent thread.
        let (all_threads_metrics_tx, metrics_rx): (
            flume::Sender<SwanlingMetric>,
            flume::Receiver<SwanlingMetric>,
        ) = flume::unbounded();

        // Optionally spawn a telnet and/or Websocket Controller thread.
        let controller_channel_rx = self.setup_controllers().await;

        // Grab now() once from the standard library, used by multiple timers in
        // the run state.
        let std_now = std::time::Instant::now();

        let swanling_attack_run_state = SwanlingAttackRunState {
            spawn_user_timer: std_now,
            spawn_user_in_ms: 0,
            spawn_user_counter: 0,
            drift_timer: tokio::time::Instant::now(),
            all_threads_metrics_tx,
            metrics_rx,
            logger_handle: None,
            all_threads_logger_tx: None,
            throttle_threads_tx: None,
            parent_to_throttle_tx: None,
            controller_channel_rx,
            report_file: None,
            metrics_header_displayed: false,
            idle_status_displayed: false,
            users: Vec::new(),
            user_channels: Vec::new(),
            running_metrics_timer: std_now,
            display_running_metrics: false,
            all_users_spawned: false,
            shutdown_after_stop: !self.configuration.no_autostart,
            canceled: Arc::new(AtomicBool::new(false)),
            socket,
        };

        // Access socket to avoid errors.
        trace!("socket: {:?}", &swanling_attack_run_state.socket);

        // Catch ctrl-c to allow clean shutdown to display metrics.
        util::setup_ctrlc_handler(&swanling_attack_run_state.canceled);

        Ok(swanling_attack_run_state)
    }

    // Spawn [`SwanlingUser`](./swanling/struct.SwanlingUser.html) threads to generate a
    // [`SwanlingAttack`](./struct.SwanlingAttack.html).
    async fn spawn_attack(
        &mut self,
        swanling_attack_run_state: &mut SwanlingAttackRunState,
    ) -> Result<(), SwanlingError> {
        // If the run_timer has expired, stop spawning user threads and start stopping them
        // instead. Unwrap is safe here because load test had to start to get here.
        if util::timer_expired(self.started.unwrap(), self.run_time) {
            self.set_attack_phase(swanling_attack_run_state, AttackPhase::Stopping);
            return Ok(());
        }

        // Hatch rate is used to schedule the next user, and to ensure we don't
        // sleep too long.
        let hatch_rate = util::get_hatch_rate(self.configuration.hatch_rate.clone());

        // Determine if it's time to spawn a SwanlingUser.
        if swanling_attack_run_state.spawn_user_in_ms == 0
            || util::ms_timer_expired(
                swanling_attack_run_state.spawn_user_timer,
                swanling_attack_run_state.spawn_user_in_ms,
            )
        {
            // Reset the spawn timer.
            swanling_attack_run_state.spawn_user_timer = std::time::Instant::now();

            // To determine how long before we spawn the next SwanlingUser, start with 1,000.0
            // milliseconds and divide by the hatch_rate.
            swanling_attack_run_state.spawn_user_in_ms = (1_000.0 / hatch_rate) as usize;

            // If running on a Worker, multiple by the number of workers as each is spawning
            // SwanlingUsers at this rate.
            if self.attack_mode == AttackMode::Worker {
                swanling_attack_run_state.spawn_user_in_ms *=
                    self.configuration.expect_workers.unwrap() as usize;
            }

            // Spawn next scheduled SwanlingUser.
            let mut thread_user =
                self.weighted_users[swanling_attack_run_state.spawn_user_counter].clone();
            swanling_attack_run_state.spawn_user_counter += 1;

            // Copy weighted tasks and weighted on start tasks into the user thread.
            thread_user.weighted_tasks = self.task_sets[thread_user.task_sets_index]
                .weighted_tasks
                .clone();
            thread_user.weighted_on_start_tasks = self.task_sets[thread_user.task_sets_index]
                .weighted_on_start_tasks
                .clone();
            thread_user.weighted_on_stop_tasks = self.task_sets[thread_user.task_sets_index]
                .weighted_on_stop_tasks
                .clone();
            // Remember which task group this user is using.
            thread_user.weighted_users_index = self.metrics.users;

            // Create a per-thread channel allowing parent thread to control child threads.
            let (parent_sender, thread_receiver): (
                flume::Sender<SwanlingUserCommand>,
                flume::Receiver<SwanlingUserCommand>,
            ) = flume::unbounded();
            swanling_attack_run_state.user_channels.push(parent_sender);

            // Clone the logger_tx if enabled, otherwise is None.
            thread_user.logger = swanling_attack_run_state.all_threads_logger_tx.clone();

            // Copy the SwanlingUser-throttle receiver channel, used by all threads.
            thread_user.throttle = if self.configuration.throttle_requests > 0 {
                Some(
                    swanling_attack_run_state
                        .throttle_threads_tx
                        .clone()
                        .unwrap(),
                )
            } else {
                None
            };

            // Copy the SwanlingUser-to-parent sender channel, used by all threads.
            thread_user.channel_to_parent =
                Some(swanling_attack_run_state.all_threads_metrics_tx.clone());

            // Copy the appropriate task_set into the thread.
            let thread_task_set = self.task_sets[thread_user.task_sets_index].clone();

            // We number threads from 1 as they're human-visible (in the logs),
            // whereas metrics.users starts at 0.
            let thread_number = self.metrics.users + 1;

            let is_worker = self.attack_mode == AttackMode::Worker;

            // If running on Worker, use Worker configuration in SwanlingUser.
            if is_worker {
                thread_user.config = self.configuration.clone();
            }

            // Launch a new user.
            let user = tokio::spawn(user::user_main(
                thread_number,
                thread_task_set,
                thread_user,
                thread_receiver,
                is_worker,
            ));

            swanling_attack_run_state.users.push(user);
            self.metrics.users += 1;

            if let Some(running_metrics) = self.configuration.running_metrics {
                if self.attack_mode != AttackMode::Worker
                    && util::timer_expired(
                        swanling_attack_run_state.running_metrics_timer,
                        running_metrics,
                    )
                {
                    swanling_attack_run_state.running_metrics_timer = time::Instant::now();
                    self.metrics.print_running();
                }
            }
        } else {
            // If displaying running metrics, be sure we wake up often enough to
            // display them at the configured rate.
            let running_metrics = self.configuration.running_metrics.unwrap_or(0);

            // Otherwise, sleep until the next time something needs to happen.
            let sleep_duration = if running_metrics > 0
                && running_metrics * 1_000 < swanling_attack_run_state.spawn_user_in_ms
            {
                let sleep_delay = self.configuration.running_metrics.unwrap() * 1_000;
                swanling_attack_run_state.spawn_user_in_ms -= sleep_delay;
                tokio::time::Duration::from_millis(sleep_delay as u64)
            } else {
                tokio::time::Duration::from_millis(
                    swanling_attack_run_state.spawn_user_in_ms as u64,
                )
            };
            debug!("sleeping {:?}...", sleep_duration);
            swanling_attack_run_state.drift_timer =
                util::sleep_minus_drift(sleep_duration, swanling_attack_run_state.drift_timer)
                    .await;
        }

        // If enough users have been spawned, move onto the next attack phase.
        if self.metrics.users >= self.weighted_users.len() {
            // Pause a tenth of a second waiting for the final user to fully start up.
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            if self.attack_mode == AttackMode::Worker {
                info!(
                    "[{}] launched {} users...",
                    get_worker_id(),
                    self.metrics.users
                );
            } else {
                info!("launched {} users...", self.metrics.users);
            }

            self.reset_metrics(swanling_attack_run_state).await?;
            self.set_attack_phase(swanling_attack_run_state, AttackPhase::Running);
        }

        Ok(())
    }

    // Let the [`SwanlingAttack`](./struct.SwanlingAttack.html) run until the timer expires
    // (or the test is canceled), and then trigger a shut down.
    async fn monitor_attack(
        &mut self,
        swanling_attack_run_state: &mut SwanlingAttackRunState,
    ) -> Result<(), SwanlingError> {
        // Exit if run_time timer expires.
        if util::timer_expired(self.started.unwrap(), self.run_time) {
            self.set_attack_phase(swanling_attack_run_state, AttackPhase::Stopping);
        } else {
            // Subtract the time spent doing other things, running the main parent loop twice
            // per second.
            swanling_attack_run_state.drift_timer = util::sleep_minus_drift(
                time::Duration::from_millis(500),
                swanling_attack_run_state.drift_timer,
            )
            .await;
        }

        Ok(())
    }

    async fn stop_running_users(
        &mut self,
        swanling_attack_run_state: &mut SwanlingAttackRunState,
    ) -> Result<(), SwanlingError> {
        if self.attack_mode == AttackMode::Worker {
            info!(
                "[{}] stopping after {} seconds...",
                get_worker_id(),
                self.metrics.duration
            );

            // Load test is shutting down, update pipe handler so there is no panic
            // when the Manager goes away.
            #[cfg(feature = "gaggle")]
            {
                let manager = swanling_attack_run_state.socket.clone().unwrap();
                register_shutdown_pipe_handler(&manager);
            }
        } else {
            info!("stopping after {} seconds...", self.metrics.duration);
        }
        for (index, send_to_user) in swanling_attack_run_state.user_channels.iter().enumerate() {
            match send_to_user.send(SwanlingUserCommand::Exit) {
                Ok(_) => {
                    debug!("telling user {} to exit", index);
                }
                Err(e) => {
                    info!("failed to tell user {} to exit: {}", index, e);
                }
            }
        }
        if self.attack_mode == AttackMode::Worker {
            info!("[{}] waiting for users to exit", get_worker_id());
        } else {
            info!("waiting for users to exit");
        }

        // If throttle is enabled, tell throttle thread the load test is over.
        if let Some(throttle_tx) = swanling_attack_run_state.parent_to_throttle_tx.clone() {
            let _ = throttle_tx.send(false);
        }

        // Take the users vector out of the SwanlingAttackRunState object so it can be
        // consumed by futures::future::join_all().
        let users = std::mem::take(&mut swanling_attack_run_state.users);
        futures::future::join_all(users).await;
        debug!("all users exited");

        // If the logger thread is enabled, tell it to flush and exit.
        if swanling_attack_run_state.logger_handle.is_some() {
            if let Err(e) = swanling_attack_run_state
                .all_threads_logger_tx
                .clone()
                .unwrap()
                .send(None)
            {
                warn!("unexpected error telling logger thread to exit: {}", e);
            };
            // Take logger out of the SwanlingAttackRunState object so it can be
            // consumed by tokio::join!().
            let logger = std::mem::take(&mut swanling_attack_run_state.logger_handle);
            let _ = tokio::join!(logger.unwrap());
        }

        // If we're printing metrics, collect the final metrics received from users.
        if !self.configuration.no_metrics {
            // Set the second parameter to true, ensuring that Swanling waits until all metrics
            // are received.
            let _received_message = self
                .receive_metrics(swanling_attack_run_state, true)
                .await?;
        }

        #[cfg(feature = "gaggle")]
        {
            // As worker, push metrics up to manager.
            if self.attack_mode == AttackMode::Worker {
                worker::push_metrics_to_manager(
                    &swanling_attack_run_state.socket.clone().unwrap(),
                    vec![
                        GaggleMetrics::Requests(self.metrics.requests.clone()),
                        GaggleMetrics::Errors(self.metrics.errors.clone()),
                        GaggleMetrics::Tasks(self.metrics.tasks.clone()),
                    ],
                    true,
                );
                // No need to reset local metrics, the worker is exiting.
            }
        }

        Ok(())
    }

    // Cleanly shut down the [`SwanlingAttack`](./struct.SwanlingAttack.html).
    async fn stop_attack(&mut self) -> Result<(), SwanlingError> {
        // Run any configured test_stop() functions.
        self.run_test_stop().await?;

        // Percentile and errors are only displayed when the load test is finished.
        self.metrics.final_metrics = true;

        Ok(())
    }

    // Reset the SwanlingAttackRunState before starting a load test. This is to allow a Controller
    // to stop and start the load test multiple times, for example from a UI.
    async fn reset_run_state(
        &mut self,
        swanling_attack_run_state: &mut SwanlingAttackRunState,
    ) -> Result<(), SwanlingError> {
        // Run any configured test_start() functions.
        self.run_test_start().await.unwrap();

        // Prepare to collect metrics, if enabled.
        self.metrics = SwanlingMetrics::default();
        if !self.configuration.no_metrics {
            self.metrics
                .initialize_task_metrics(&self.task_sets, &self.configuration);
            self.metrics.display_metrics = true;
            // Only display status codes if enabled.
            self.metrics.display_status_codes = self.configuration.status_codes;
        }

        // Reset the run state.
        let std_now = std::time::Instant::now();
        swanling_attack_run_state.spawn_user_timer = std_now;
        swanling_attack_run_state.spawn_user_in_ms = 0;
        swanling_attack_run_state.spawn_user_counter = 0;
        swanling_attack_run_state.drift_timer = tokio::time::Instant::now();
        swanling_attack_run_state.metrics_header_displayed = false;
        swanling_attack_run_state.idle_status_displayed = false;
        swanling_attack_run_state.users = Vec::new();
        swanling_attack_run_state.user_channels = Vec::new();
        swanling_attack_run_state.running_metrics_timer = std_now;
        swanling_attack_run_state.display_running_metrics = false;
        swanling_attack_run_state.shutdown_after_stop = !self.configuration.no_autostart;
        swanling_attack_run_state.all_users_spawned = false;

        // If enabled, spawn a logger thread.
        let (logger_handle, all_threads_logger_tx) =
            self.configuration.setup_loggers(&self.defaults).await?;
        swanling_attack_run_state.logger_handle = logger_handle;
        swanling_attack_run_state.all_threads_logger_tx = all_threads_logger_tx;

        // If enabled, spawn a throttle thread.
        let (throttle_threads_tx, parent_to_throttle_tx) = self.setup_throttle().await;
        swanling_attack_run_state.throttle_threads_tx = throttle_threads_tx;
        swanling_attack_run_state.parent_to_throttle_tx = parent_to_throttle_tx;

        // If enabled, create an report file and confirm access.
        swanling_attack_run_state.report_file = match self.prepare_report_file().await {
            Ok(f) => f,
            Err(e) => {
                return Err(SwanlingError::InvalidOption {
                    option: "--report-file".to_string(),
                    value: self.get_report_file_path().unwrap(),
                    detail: format!("Failed to create report file: {}", e),
                })
            }
        };

        // Record when the SwanlingAttack officially started.
        self.started = Some(time::Instant::now());

        // Also record a formattable timestamp, for human readable reports.
        self.metrics.started = Some(Local::now());

        Ok(())
    }

    // Called internally in local-mode and gaggle-mode.
    async fn start_attack(
        mut self,
        socket: Option<Socket>,
    ) -> Result<SwanlingAttack, SwanlingError> {
        trace!("start_attack: socket({:?})", socket);

        // The SwanlingAttackRunState is used while spawning and running the
        // SwanlingUser threads that generate the load test.
        let mut swanling_attack_run_state = self
            .initialize_attack(socket)
            .await
            .expect("failed to initialize SwanlingAttackRunState");

        // The Swanling parent process SwanlingAttack loop runs until Swanling shuts down. Swanling enters
        // the loop in AttackPhase::Idle, and exits in AttackPhase::Shutdown.
        loop {
            match self.attack_phase {
                // In the Idle phase the Swanling configuration can be changed by a Controller,
                // and otherwise nothing happens but sleeping an checking for messages.
                AttackPhase::Idle => {
                    if self.configuration.no_autostart {
                        // Sleep then check for further instructions.
                        if swanling_attack_run_state.idle_status_displayed {
                            let sleep_duration = tokio::time::Duration::from_millis(250);
                            debug!("sleeping {:?}...", sleep_duration);
                            swanling_attack_run_state.drift_timer = util::sleep_minus_drift(
                                sleep_duration,
                                swanling_attack_run_state.drift_timer,
                            )
                            .await;
                        // Only display informational message about being idle one time.
                        } else {
                            info!("Swanling is currently idle.");
                            swanling_attack_run_state.idle_status_displayed = true;
                        }
                    } else {
                        // Prepare to start the load test, resetting timers and counters.
                        self.reset_run_state(&mut swanling_attack_run_state).await?;
                        self.set_attack_phase(
                            &mut swanling_attack_run_state,
                            AttackPhase::Starting,
                        );
                    }
                }
                // In the Start phase, Swanling launches SwanlingUser threads and starts a SwanlingAttack.
                AttackPhase::Starting => {
                    self.update_duration();
                    self.spawn_attack(&mut swanling_attack_run_state)
                        .await
                        .expect("failed to start SwanlingAttack");
                }
                // In the Running phase, Swanling maintains the configured SwanlingAttack.
                AttackPhase::Running => {
                    self.update_duration();
                    self.monitor_attack(&mut swanling_attack_run_state).await?;
                }
                // In the Stopping phase, Swanling stops all SwanlingUser threads and optionally reports
                // any collected metrics.
                AttackPhase::Stopping => {
                    // If displaying metrics, update internal state reflecting how long load test
                    // has been running.
                    self.update_duration();
                    // Tell all running SwanlingUsers to stop.
                    self.stop_running_users(&mut swanling_attack_run_state)
                        .await?;
                    // Stop any running SwanlingUser threads.
                    self.stop_attack().await?;
                    // Collect all metrics sent by SwanlingUser threads.
                    self.sync_metrics(&mut swanling_attack_run_state, true)
                        .await?;
                    // Write an html report, if enabled.
                    self.write_html_report(&mut swanling_attack_run_state)
                        .await?;
                    // Shutdown Swanling or go into an idle waiting state.
                    if swanling_attack_run_state.shutdown_after_stop {
                        self.set_attack_phase(
                            &mut swanling_attack_run_state,
                            AttackPhase::Shutdown,
                        );
                    } else {
                        // Print metrics, if enabled.
                        if !self.configuration.no_metrics {
                            println!("{}", self.metrics);
                        }
                        self.set_attack_phase(&mut swanling_attack_run_state, AttackPhase::Idle);
                    }
                }
                // By reaching the Shutdown phase, break out of the SwanlingAttack loop.
                AttackPhase::Shutdown => break,
            }
            // Regularly synchronize metrics.
            self.sync_metrics(&mut swanling_attack_run_state, false)
                .await?;

            // Check if a Controller has made a request.
            self.handle_controller_requests(&mut swanling_attack_run_state)
                .await?;

            // Gracefully exit loop if ctrl-c is caught.
            if self.attack_phase != AttackPhase::Shutdown
                && swanling_attack_run_state.canceled.load(Ordering::SeqCst)
            {
                // Shutdown after stopping as the load test was canceled.
                swanling_attack_run_state.shutdown_after_stop = true;

                // No metrics to display when sitting idle, so disable.
                if self.attack_phase == AttackPhase::Idle {
                    self.metrics.display_metrics = false;
                }

                // Cleanly stop the load test.
                self.set_attack_phase(&mut swanling_attack_run_state, AttackPhase::Stopping);
            }
        }

        Ok(self)
    }
}

/// All run-time options can optionally be configured with custom defaults.
///
/// For example, you can optionally configure a default host for the load test. This is
/// used if no per-[`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html) host is defined,
/// no `--host` CLI option is configured, and if the
/// [`SwanlingTask`](./swanling/struct.SwanlingTask.html) itself doesn't hard-code the host in
/// the base url of its request. In that case, this host is added to all requests.
///
/// For example, a load test could be configured to default to running against a local
/// development container, and the `--host` option could be used to override the host
/// value to run the load test against the production environment.
///
/// # Example
/// ```rust
/// use swanling::prelude::*;
///
/// fn main() -> Result<(), SwanlingError> {
///     SwanlingAttack::initialize()?
///         .set_default(SwanlingDefault::Host, "local.dev")?;
///
///     Ok(())
/// }
/// ```
///
/// The following run-time options can be configured with a custom default using a
/// borrowed string slice (`&str`):
///  - [SwanlingDefault::Host](../swanling/enum.SwanlingDefault.html#variant.Host)
///  - [SwanlingDefault::SwanlingLog](../swanling/enum.SwanlingDefault.html#variant.SwanlingLog)
///  - [SwanlingDefault::RequestFormat](../swanling/enum.SwanlingDefault.html#variant.RequestFormat)
///  - [SwanlingDefault::TaskLog](../swanling/enum.SwanlingDefault.html#variant.TaskLog)
///  - [SwanlingDefault::ErrorLog](../swanling/enum.SwanlingDefault.html#variant.ErrorLog)
///  - [SwanlingDefault::DebugLog](../swanling/enum.SwanlingDefault.html#variant.DebugLog)
///  - [SwanlingDefault::TelnetHost](../swanling/enum.SwanlingDefault.html#variant.TelnetHost)
///  - [SwanlingDefault::WebSocketHost](../swanling/enum.SwanlingDefault.html#variant.WebSocketHost)
///  - [SwanlingDefault::ManagerBindHost](../swanling/enum.SwanlingDefault.html#variant.ManagerBindHost)
///  - [SwanlingDefault::ManagerHost](../swanling/enum.SwanlingDefault.html#variant.ManagerHost)
///
/// The following run-time options can be configured with a custom default using a
/// `usize` integer:
///  - [SwanlingDefault::Users](../swanling/enum.SwanlingDefault.html#variant.Users)
///  - [SwanlingDefault::HatchRate](../swanling/enum.SwanlingDefault.html#variant.HatchRate)
///  - [SwanlingDefault::RunTime](../swanling/enum.SwanlingDefault.html#variant.RunTime)
///  - [SwanlingDefault::RunningMetrics](../swanling/enum.SwanlingDefault.html#variant.RunningMetrics)
///  - [SwanlingDefault::LogLevel](../swanling/enum.SwanlingDefault.html#variant.LogLevel)
///  - [SwanlingDefault::Verbose](../swanling/enum.SwanlingDefault.html#variant.Verbose)
///  - [SwanlingDefault::ThrottleRequests](../swanling/enum.SwanlingDefault.html#variant.ThrottleRequests)
///  - [SwanlingDefault::ExpectWorkers](../swanling/enum.SwanlingDefault.html#variant.ExpectWorkers)
///  - [SwanlingDefault::TelnetPort](../swanling/enum.SwanlingDefault.html#variant.TelnetPort)
///  - [SwanlingDefault::WebSocketPort](../swanling/enum.SwanlingDefault.html#variant.WebSocketPort)
///  - [SwanlingDefault::ManagerBindPort](../swanling/enum.SwanlingDefault.html#variant.ManagerBindPort)
///  - [SwanlingDefault::ManagerPort](../swanling/enum.SwanlingDefault.html#variant.ManagerPort)
///
/// The following run-time flags can be configured with a custom default using a
/// `bool` (and otherwise default to `false`).
///  - [SwanlingDefault::NoResetMetrics](../swanling/enum.SwanlingDefault.html#variant.NoResetMetrics)
///  - [SwanlingDefault::NoMetrics](../swanling/enum.SwanlingDefault.html#variant.NoMetrics)
///  - [SwanlingDefault::NoTaskMetrics](../swanling/enum.SwanlingDefault.html#variant.NoTaskMetrics)
///  - [SwanlingDefault::NoErrorSummary](../swanling/enum.SwanlingDefault.html#variant.NoErrorSummary)
///  - [SwanlingDefault::NoDebugBody](../swanling/enum.SwanlingDefault.html#variant.NoDebugBody)
///  - [SwanlingDefault::NoTelnet](../swanling/enum.SwanlingDefault.html#variant.NoTelnet)
///  - [SwanlingDefault::NoWebSocket](../swanling/enum.SwanlingDefault.html#variant.NoWebSocket)
///  - [SwanlingDefault::NoAutoStart](../swanling/enum.SwanlingDefault.html#variant.NoAutoStart)
///  - [SwanlingDefault::StatusCodes](../swanling/enum.SwanlingDefault.html#variant.StatusCodes)
///  - [SwanlingDefault::StickyFollow](../swanling/enum.SwanlingDefault.html#variant.StickyFollow)
///  - [SwanlingDefault::Manager](../swanling/enum.SwanlingDefault.html#variant.Manager)
///  - [SwanlingDefault::NoHashCheck](../swanling/enum.SwanlingDefault.html#variant.NoHashCheck)
///  - [SwanlingDefault::Worker](../swanling/enum.SwanlingDefault.html#variant.Worker)
///
/// The following run-time flags can be configured with a custom default using a
/// `SwanlingLogFormat`.
///  - [SwanlingDefault::RequestLog](../swanling/enum.SwanlingDefault.html#variant.RequestLog)
///  - [SwanlingDefault::TaskLog](../swanling/enum.SwanlingDefault.html#variant.TaskLog)
///  - [SwanlingDefault::DebugFormat](../swanling/enum.SwanlingDefault.html#variant.DebugFormat)
/// # Another Example
/// ```rust
/// use swanling::prelude::*;
///
/// fn main() -> Result<(), SwanlingError> {
///     SwanlingAttack::initialize()?
///         // Do not reset the metrics after the load test finishes starting.
///         .set_default(SwanlingDefault::NoResetMetrics, true)?
///         // Display info level logs while the test runs.
///         .set_default(SwanlingDefault::Verbose, 1)?
///         // Log all requests made during the test to `./swanling-request.log`.
///         .set_default(SwanlingDefault::RequestLog, "swanling-request.log")?;
///
///     Ok(())
/// }
/// ```
pub trait SwanlingDefaultType<T> {
    fn set_default(self, key: SwanlingDefault, value: T) -> Result<Box<Self>, SwanlingError>;
}
impl SwanlingDefaultType<&str> for SwanlingAttack {
    fn set_default(
        mut self,
        key: SwanlingDefault,
        value: &str,
    ) -> Result<Box<Self>, SwanlingError> {
        match key {
            // Set valid defaults.
            SwanlingDefault::HatchRate => self.defaults.hatch_rate = Some(value.to_string()),
            SwanlingDefault::Host => self.defaults.host = Some(value.to_string()),
            SwanlingDefault::SwanlingLog => self.defaults.swanling_log = Some(value.to_string()),
            SwanlingDefault::ReportFile => self.defaults.report_file = Some(value.to_string()),
            SwanlingDefault::RequestLog => self.defaults.request_log = Some(value.to_string()),
            SwanlingDefault::TaskLog => self.defaults.task_log = Some(value.to_string()),
            SwanlingDefault::ErrorLog => self.defaults.error_log = Some(value.to_string()),
            SwanlingDefault::DebugLog => self.defaults.debug_log = Some(value.to_string()),
            SwanlingDefault::TelnetHost => self.defaults.telnet_host = Some(value.to_string()),
            SwanlingDefault::WebSocketHost => {
                self.defaults.websocket_host = Some(value.to_string())
            }
            SwanlingDefault::ManagerBindHost => {
                self.defaults.manager_bind_host = Some(value.to_string())
            }
            SwanlingDefault::ManagerHost => self.defaults.manager_host = Some(value.to_string()),
            // Otherwise display a helpful and explicit error.
            SwanlingDefault::Users
            | SwanlingDefault::RunTime
            | SwanlingDefault::LogLevel
            | SwanlingDefault::Verbose
            | SwanlingDefault::ThrottleRequests
            | SwanlingDefault::ExpectWorkers
            | SwanlingDefault::TelnetPort
            | SwanlingDefault::WebSocketPort
            | SwanlingDefault::ManagerBindPort
            | SwanlingDefault::ManagerPort => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected usize value, received &str",
                        key, value
                    ),
                });
            }
            SwanlingDefault::RunningMetrics
            | SwanlingDefault::NoResetMetrics
            | SwanlingDefault::NoMetrics
            | SwanlingDefault::NoTaskMetrics
            | SwanlingDefault::NoErrorSummary
            | SwanlingDefault::NoDebugBody
            | SwanlingDefault::NoTelnet
            | SwanlingDefault::NoWebSocket
            | SwanlingDefault::NoAutoStart
            | SwanlingDefault::StatusCodes
            | SwanlingDefault::StickyFollow
            | SwanlingDefault::Manager
            | SwanlingDefault::NoHashCheck
            | SwanlingDefault::Worker => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected bool value, received &str",
                        key, value
                    ),
                });
            }
            SwanlingDefault::DebugFormat
            | SwanlingDefault::ErrorFormat
            | SwanlingDefault::TaskFormat
            | SwanlingDefault::RequestFormat => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected SwanlingLogFormat value, received &str",
                        key, value
                    ),
                });
            }
            SwanlingDefault::CoordinatedOmissionMitigation => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected SwanlingCoordinatedOmissionMitigation value, received &str",
                        key, value
                    ),
                });
            }
        }
        Ok(Box::new(self))
    }
}
impl SwanlingDefaultType<usize> for SwanlingAttack {
    fn set_default(
        mut self,
        key: SwanlingDefault,
        value: usize,
    ) -> Result<Box<Self>, SwanlingError> {
        match key {
            SwanlingDefault::Users => self.defaults.users = Some(value),
            SwanlingDefault::RunTime => self.defaults.run_time = Some(value),
            SwanlingDefault::RunningMetrics => self.defaults.running_metrics = Some(value),
            SwanlingDefault::LogLevel => self.defaults.log_level = Some(value as u8),
            SwanlingDefault::Verbose => self.defaults.verbose = Some(value as u8),
            SwanlingDefault::ThrottleRequests => self.defaults.throttle_requests = Some(value),
            SwanlingDefault::ExpectWorkers => self.defaults.expect_workers = Some(value as u16),
            SwanlingDefault::TelnetPort => self.defaults.telnet_port = Some(value as u16),
            SwanlingDefault::WebSocketPort => self.defaults.websocket_port = Some(value as u16),
            SwanlingDefault::ManagerBindPort => {
                self.defaults.manager_bind_port = Some(value as u16)
            }
            SwanlingDefault::ManagerPort => self.defaults.manager_port = Some(value as u16),
            // Otherwise display a helpful and explicit error.
            SwanlingDefault::Host
            | SwanlingDefault::HatchRate
            | SwanlingDefault::SwanlingLog
            | SwanlingDefault::ReportFile
            | SwanlingDefault::RequestLog
            | SwanlingDefault::TaskLog
            | SwanlingDefault::ErrorLog
            | SwanlingDefault::DebugLog
            | SwanlingDefault::TelnetHost
            | SwanlingDefault::WebSocketHost
            | SwanlingDefault::ManagerBindHost
            | SwanlingDefault::ManagerHost => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{}", value),
                    detail: format!(
                    "set_default(SwanlingDefault::{:?}, {}) expected &str value, received usize",
                    key, value
                ),
                })
            }
            SwanlingDefault::NoResetMetrics
            | SwanlingDefault::NoMetrics
            | SwanlingDefault::NoTaskMetrics
            | SwanlingDefault::NoErrorSummary
            | SwanlingDefault::NoDebugBody
            | SwanlingDefault::NoTelnet
            | SwanlingDefault::NoWebSocket
            | SwanlingDefault::NoAutoStart
            | SwanlingDefault::StatusCodes
            | SwanlingDefault::StickyFollow
            | SwanlingDefault::Manager
            | SwanlingDefault::NoHashCheck
            | SwanlingDefault::Worker => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{}", value),
                    detail: format!(
                    "set_default(SwanlingDefault::{:?}, {}) expected bool value, received usize",
                    key, value
                ),
                })
            }
            SwanlingDefault::RequestFormat
            | SwanlingDefault::DebugFormat
            | SwanlingDefault::ErrorFormat
            | SwanlingDefault::TaskFormat => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected SwanlingLogFormat value, received usize",
                        key, value
                    ),
                });
            }
            SwanlingDefault::CoordinatedOmissionMitigation => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected SwanlingCoordinatedOmissionMitigation value, received usize",
                        key, value
                    ),
                });
            }
        }
        Ok(Box::new(self))
    }
}
impl SwanlingDefaultType<bool> for SwanlingAttack {
    fn set_default(
        mut self,
        key: SwanlingDefault,
        value: bool,
    ) -> Result<Box<Self>, SwanlingError> {
        match key {
            SwanlingDefault::NoResetMetrics => self.defaults.no_reset_metrics = Some(value),
            SwanlingDefault::NoMetrics => self.defaults.no_metrics = Some(value),
            SwanlingDefault::NoTaskMetrics => self.defaults.no_task_metrics = Some(value),
            SwanlingDefault::NoErrorSummary => self.defaults.no_error_summary = Some(value),
            SwanlingDefault::NoDebugBody => self.defaults.no_debug_body = Some(value),
            SwanlingDefault::NoTelnet => self.defaults.no_telnet = Some(value),
            SwanlingDefault::NoWebSocket => self.defaults.no_websocket = Some(value),
            SwanlingDefault::NoAutoStart => self.defaults.no_autostart = Some(value),
            SwanlingDefault::StatusCodes => self.defaults.status_codes = Some(value),
            SwanlingDefault::StickyFollow => self.defaults.sticky_follow = Some(value),
            SwanlingDefault::Manager => self.defaults.manager = Some(value),
            SwanlingDefault::NoHashCheck => self.defaults.no_hash_check = Some(value),
            SwanlingDefault::Worker => self.defaults.worker = Some(value),
            // Otherwise display a helpful and explicit error.
            SwanlingDefault::Host
            | SwanlingDefault::SwanlingLog
            | SwanlingDefault::ReportFile
            | SwanlingDefault::RequestLog
            | SwanlingDefault::TaskLog
            | SwanlingDefault::RunningMetrics
            | SwanlingDefault::ErrorLog
            | SwanlingDefault::DebugLog
            | SwanlingDefault::TelnetHost
            | SwanlingDefault::WebSocketHost
            | SwanlingDefault::ManagerBindHost
            | SwanlingDefault::ManagerHost => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected &str value, received bool",
                        key, value
                    ),
                })
            }
            SwanlingDefault::Users
            | SwanlingDefault::HatchRate
            | SwanlingDefault::RunTime
            | SwanlingDefault::LogLevel
            | SwanlingDefault::Verbose
            | SwanlingDefault::ThrottleRequests
            | SwanlingDefault::ExpectWorkers
            | SwanlingDefault::TelnetPort
            | SwanlingDefault::WebSocketPort
            | SwanlingDefault::ManagerBindPort
            | SwanlingDefault::ManagerPort => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{}", value),
                    detail: format!(
                    "set_default(SwanlingDefault::{:?}, {}) expected usize value, received bool",
                    key, value
                ),
                })
            }
            SwanlingDefault::RequestFormat
            | SwanlingDefault::DebugFormat
            | SwanlingDefault::ErrorFormat
            | SwanlingDefault::TaskFormat => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected SwanlingLogFormat value, received bool",
                        key, value
                    ),
                });
            }
            SwanlingDefault::CoordinatedOmissionMitigation => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: value.to_string(),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {}) expected SwanlingCoordinatedOmissionMitigation value, received bool",
                        key, value
                    ),
                });
            }
        }
        Ok(Box::new(self))
    }
}
impl SwanlingDefaultType<SwanlingCoordinatedOmissionMitigation> for SwanlingAttack {
    fn set_default(
        mut self,
        key: SwanlingDefault,
        value: SwanlingCoordinatedOmissionMitigation,
    ) -> Result<Box<Self>, SwanlingError> {
        match key {
            SwanlingDefault::CoordinatedOmissionMitigation => self.defaults.co_mitigation = Some(value),
            // Otherwise display a helpful and explicit error.
            SwanlingDefault::NoResetMetrics
            | SwanlingDefault::NoMetrics
            | SwanlingDefault::NoTaskMetrics
            | SwanlingDefault::NoErrorSummary
            | SwanlingDefault::NoDebugBody
            | SwanlingDefault::NoTelnet
            | SwanlingDefault::NoWebSocket
            | SwanlingDefault::NoAutoStart
            | SwanlingDefault::StatusCodes
            | SwanlingDefault::StickyFollow
            | SwanlingDefault::Manager
            | SwanlingDefault::NoHashCheck
            | SwanlingDefault::Worker => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected bool value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
            SwanlingDefault::Host
            | SwanlingDefault::SwanlingLog
            | SwanlingDefault::ReportFile
            | SwanlingDefault::RequestLog
            | SwanlingDefault::TaskLog
            | SwanlingDefault::RunningMetrics
            | SwanlingDefault::ErrorLog
            | SwanlingDefault::DebugLog
            | SwanlingDefault::TelnetHost
            | SwanlingDefault::WebSocketHost
            | SwanlingDefault::ManagerBindHost
            | SwanlingDefault::ManagerHost => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected &str value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
            SwanlingDefault::Users
            | SwanlingDefault::HatchRate
            | SwanlingDefault::RunTime
            | SwanlingDefault::LogLevel
            | SwanlingDefault::Verbose
            | SwanlingDefault::ThrottleRequests
            | SwanlingDefault::ExpectWorkers
            | SwanlingDefault::TelnetPort
            | SwanlingDefault::WebSocketPort
            | SwanlingDefault::ManagerBindPort
            | SwanlingDefault::ManagerPort => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected usize value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
            SwanlingDefault::RequestFormat
            | SwanlingDefault::DebugFormat
            | SwanlingDefault::ErrorFormat
            | SwanlingDefault::TaskFormat => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected SwanlingLogFormat value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
        }
        Ok(Box::new(self))
    }
}
impl SwanlingDefaultType<SwanlingLogFormat> for SwanlingAttack {
    fn set_default(
        mut self,
        key: SwanlingDefault,
        value: SwanlingLogFormat,
    ) -> Result<Box<Self>, SwanlingError> {
        match key {
            SwanlingDefault::RequestFormat => self.defaults.request_format = Some(value),
            SwanlingDefault::DebugFormat => self.defaults.debug_format = Some(value),
            SwanlingDefault::ErrorFormat => self.defaults.error_format = Some(value),
            SwanlingDefault::TaskFormat => self.defaults.task_format = Some(value),
            // Otherwise display a helpful and explicit error.
            SwanlingDefault::NoResetMetrics
            | SwanlingDefault::NoMetrics
            | SwanlingDefault::NoTaskMetrics
            | SwanlingDefault::NoErrorSummary
            | SwanlingDefault::NoDebugBody
            | SwanlingDefault::NoTelnet
            | SwanlingDefault::NoWebSocket
            | SwanlingDefault::NoAutoStart
            | SwanlingDefault::StatusCodes
            | SwanlingDefault::StickyFollow
            | SwanlingDefault::Manager
            | SwanlingDefault::NoHashCheck
            | SwanlingDefault::Worker => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected bool value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
            SwanlingDefault::Host
            | SwanlingDefault::SwanlingLog
            | SwanlingDefault::ReportFile
            | SwanlingDefault::RequestLog
            | SwanlingDefault::TaskLog
            | SwanlingDefault::RunningMetrics
            | SwanlingDefault::ErrorLog
            | SwanlingDefault::DebugLog
            | SwanlingDefault::TelnetHost
            | SwanlingDefault::WebSocketHost
            | SwanlingDefault::ManagerBindHost
            | SwanlingDefault::ManagerHost => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected &str value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
            SwanlingDefault::Users
            | SwanlingDefault::HatchRate
            | SwanlingDefault::RunTime
            | SwanlingDefault::LogLevel
            | SwanlingDefault::Verbose
            | SwanlingDefault::ThrottleRequests
            | SwanlingDefault::ExpectWorkers
            | SwanlingDefault::TelnetPort
            | SwanlingDefault::WebSocketPort
            | SwanlingDefault::ManagerBindPort
            | SwanlingDefault::ManagerPort => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected usize value, received SwanlingCoordinatedOmissionMitigation",
                        key, value
                    ),
                })
            }
            SwanlingDefault::CoordinatedOmissionMitigation => {
                return Err(SwanlingError::InvalidOption {
                    option: format!("SwanlingDefault::{:?}", key),
                    value: format!("{:?}", value),
                    detail: format!(
                        "set_default(SwanlingDefault::{:?}, {:?}) expected SwanlingCoordinatedOmissionMitigation value, received SwanlingLogFormat",
                        key, value
                    ),
                })

            }
        }
        Ok(Box::new(self))
    }
}

/// Options available when launching a Swanling load test.
#[derive(Options, Debug, Clone, Serialize, Deserialize)]
pub struct SwanlingConfiguration {
    /// Displays this help
    #[options(short = "h")]
    pub help: bool,
    /// Prints version information
    #[options(short = "V")]
    pub version: bool,
    // Add a blank line after this option
    #[options(short = "l", help = "Lists all tasks and exits\n")]
    pub list: bool,

    /// Defines host to load test (ie http://10.21.32.33)
    #[options(short = "H")]
    pub host: String,
    /// Sets concurrent users (default: number of CPUs)
    #[options(short = "u")]
    pub users: Option<usize>,
    /// Sets per-second user hatch rate (default: 1)
    #[options(short = "r", meta = "RATE")]
    pub hatch_rate: Option<String>,
    /// Stops after (30s, 20m, 3h, 1h30m, etc)
    #[options(short = "t", meta = "TIME")]
    pub run_time: String,
    /// Enables Swanling log file and sets name
    #[options(short = "G", meta = "NAME")]
    pub swanling_log: String,
    /// Sets Swanling log level (-g, -gg, etc)
    #[options(short = "g", count)]
    pub log_level: u8,
    #[options(
        count,
        short = "v",
        // Add a blank line and then a 'Metrics:' header after this option
        help = "Sets Swanling verbosity (-v, -vv, etc)\n\nMetrics:"
    )]
    pub verbose: u8,

    /// How often to optionally print running metrics
    #[options(no_short, meta = "TIME")]
    pub running_metrics: Option<usize>,
    /// Doesn't reset metrics after all users have started
    #[options(no_short)]
    pub no_reset_metrics: bool,
    /// Doesn't track metrics
    #[options(no_short)]
    pub no_metrics: bool,
    /// Doesn't track task metrics
    #[options(no_short)]
    pub no_task_metrics: bool,
    /// Doesn't display an error summary
    #[options(no_short)]
    pub no_error_summary: bool,
    /// Create an html-formatted report
    #[options(no_short, meta = "NAME")]
    pub report_file: String,
    /// Sets request log file name
    #[options(short = "R", meta = "NAME")]
    pub request_log: String,
    /// Sets request log format (csv, json, raw)
    #[options(no_short, meta = "FORMAT")]
    pub request_format: Option<SwanlingLogFormat>,
    /// Sets task log file name
    #[options(short = "T", meta = "NAME")]
    pub task_log: String,
    /// Sets task log format (csv, json, raw)
    #[options(no_short, meta = "FORMAT")]
    pub task_format: Option<SwanlingLogFormat>,
    /// Sets error log file name
    #[options(short = "E", meta = "NAME")]
    pub error_log: String,
    /// Sets error log format (csv, json, raw)
    #[options(no_short, meta = "FORMAT")]
    pub error_format: Option<SwanlingLogFormat>,
    /// Sets debug log file name
    #[options(short = "D", meta = "NAME")]
    pub debug_log: String,
    /// Sets debug log format (csv, json, raw)
    #[options(no_short, meta = "FORMAT")]
    pub debug_format: Option<SwanlingLogFormat>,
    /// Do not include the response body in the debug log
    #[options(no_short)]
    pub no_debug_body: bool,
    // Add a blank line and then an Advanced: header after this option
    #[options(no_short, help = "Tracks additional status code metrics\n\nAdvanced:")]
    pub status_codes: bool,

    /// Doesn't enable telnet Controller
    #[options(no_short)]
    pub no_telnet: bool,
    /// Sets telnet Controller host (default: 0.0.0.0)
    #[options(no_short, meta = "HOST")]
    pub telnet_host: String,
    /// Sets telnet Controller TCP port (default: 5116)
    #[options(no_short, meta = "PORT")]
    pub telnet_port: u16,
    /// Doesn't enable WebSocket Controller
    #[options(no_short)]
    pub no_websocket: bool,
    /// Sets WebSocket Controller host (default: 0.0.0.0)
    #[options(no_short, meta = "HOST")]
    pub websocket_host: String,
    /// Sets WebSocket Controller TCP port (default: 5117)
    #[options(no_short, meta = "PORT")]
    pub websocket_port: u16,
    /// Doesn't automatically start load test
    #[options(no_short)]
    pub no_autostart: bool,
    /// Sets coordinated omission mitigation strategy
    #[options(no_short, meta = "STRATEGY")]
    pub co_mitigation: Option<SwanlingCoordinatedOmissionMitigation>,
    /// Sets maximum requests per second
    #[options(no_short, meta = "VALUE")]
    pub throttle_requests: usize,
    #[options(
        no_short,
        help = "Follows base_url redirect with subsequent requests\n\nGaggle:"
    )]
    pub sticky_follow: bool,

    /// Enables distributed load test Manager mode
    #[options(no_short)]
    pub manager: bool,
    /// Sets number of Workers to expect
    #[options(no_short, meta = "VALUE")]
    pub expect_workers: Option<u16>,
    /// Tells Manager to ignore load test checksum
    #[options(no_short)]
    pub no_hash_check: bool,
    /// Sets host Manager listens on (default: 0.0.0.0)
    #[options(no_short, meta = "HOST")]
    pub manager_bind_host: String,
    /// Sets port Manager listens on (default: 5115)
    #[options(no_short, meta = "PORT")]
    pub manager_bind_port: u16,
    /// Enables distributed load test Worker mode
    #[options(no_short)]
    pub worker: bool,
    /// Sets host Worker connects to (default: 127.0.0.1)
    #[options(no_short, meta = "HOST")]
    pub manager_host: String,
    /// Sets port Worker connects to (default: 5115)
    #[options(no_short, meta = "PORT")]
    pub manager_port: u16,
}

/// Use the configured SwanlingScheduler to allocate all [`SwanlingTask`](./swanling/struct.SwanlingTask.html)s
/// within the [`SwanlingTaskSet`](./swanling/struct.SwanlingTaskSet.html) in the appropriate order. Returns
/// three set of ordered tasks: /// `on_start_tasks`, `tasks`, and `on_stop_tasks`. The
/// `on_start_tasks` are only run once when the [`SwanlingAttack`](./struct.SwanlingAttack.html) first
/// starts. Normal `tasks` are then run for the duration of the
/// [`SwanlingAttack`](./struct.SwanlingAttack.html). The `on_stop_tasks` finally are only run once when
/// the [`SwanlingAttack`](./struct.SwanlingAttack.html) stops.
fn allocate_tasks(
    task_set: &SwanlingTaskSet,
    scheduler: &SwanlingScheduler,
) -> (
    WeightedSwanlingTasks,
    WeightedSwanlingTasks,
    WeightedSwanlingTasks,
) {
    debug!(
        "allocating SwanlingTasks on SwanlingUsers with {:?} scheduler",
        scheduler
    );

    // A BTreeMap of Vectors allows us to group and sort tasks per sequence value.
    let mut sequenced_tasks: SequencedSwanlingTasks = BTreeMap::new();
    let mut sequenced_on_start_tasks: SequencedSwanlingTasks = BTreeMap::new();
    let mut sequenced_on_stop_tasks: SequencedSwanlingTasks = BTreeMap::new();
    let mut unsequenced_tasks: UnsequencedSwanlingTasks = Vec::new();
    let mut unsequenced_on_start_tasks: UnsequencedSwanlingTasks = Vec::new();
    let mut unsequenced_on_stop_tasks: UnsequencedSwanlingTasks = Vec::new();
    let mut u: usize = 0;
    let mut v: usize;

    // Find the greatest common divisor of all tasks in the task_set.
    for task in &task_set.tasks {
        if task.sequence > 0 {
            if task.on_start {
                if let Some(sequence) = sequenced_on_start_tasks.get_mut(&task.sequence) {
                    // This is another task with this order value.
                    sequence.push(task.clone());
                } else {
                    // This is the first task with this order value.
                    sequenced_on_start_tasks.insert(task.sequence, vec![task.clone()]);
                }
            }
            // Allow a task to be both on_start and on_stop.
            if task.on_stop {
                if let Some(sequence) = sequenced_on_stop_tasks.get_mut(&task.sequence) {
                    // This is another task with this order value.
                    sequence.push(task.clone());
                } else {
                    // This is the first task with this order value.
                    sequenced_on_stop_tasks.insert(task.sequence, vec![task.clone()]);
                }
            }
            if !task.on_start && !task.on_stop {
                if let Some(sequence) = sequenced_tasks.get_mut(&task.sequence) {
                    // This is another task with this order value.
                    sequence.push(task.clone());
                } else {
                    // This is the first task with this order value.
                    sequenced_tasks.insert(task.sequence, vec![task.clone()]);
                }
            }
        } else {
            if task.on_start {
                unsequenced_on_start_tasks.push(task.clone());
            }
            if task.on_stop {
                unsequenced_on_stop_tasks.push(task.clone());
            }
            if !task.on_start && !task.on_stop {
                unsequenced_tasks.push(task.clone());
            }
        }
        // Look for lowest common divisor amongst all tasks of any weight.
        if u == 0 {
            u = task.weight;
        } else {
            v = task.weight;
            trace!("calculating greatest common denominator of {} and {}", u, v);
            u = util::gcd(u, v);
            trace!("inner gcd: {}", u);
        }
    }
    // 'u' will always be the greatest common divisor
    debug!("gcd: {}", u);

    // Apply weights to sequenced tasks.
    let weighted_sequenced_on_start_tasks = weight_sequenced_tasks(&sequenced_on_start_tasks, u);
    let weighted_sequenced_tasks = weight_sequenced_tasks(&sequenced_tasks, u);
    let weighted_sequenced_on_stop_tasks = weight_sequenced_tasks(&sequenced_on_stop_tasks, u);

    // Apply weights to unsequenced tasks.
    let (weighted_unsequenced_on_start_tasks, total_unsequenced_on_start_tasks) =
        weight_unsequenced_tasks(&unsequenced_on_start_tasks, u);
    let (weighted_unsequenced_tasks, total_unsequenced_tasks) =
        weight_unsequenced_tasks(&unsequenced_tasks, u);
    let (weighted_unsequenced_on_stop_tasks, total_unsequenced_on_stop_tasks) =
        weight_unsequenced_tasks(&unsequenced_on_stop_tasks, u);

    // Schedule sequenced tasks.
    let scheduled_sequenced_on_start_tasks =
        schedule_sequenced_tasks(&weighted_sequenced_on_start_tasks, scheduler);
    let scheduled_sequenced_tasks = schedule_sequenced_tasks(&weighted_sequenced_tasks, scheduler);
    let scheduled_sequenced_on_stop_tasks =
        schedule_sequenced_tasks(&weighted_sequenced_on_stop_tasks, scheduler);

    // Schedule unsequenced tasks.
    let scheduled_unsequenced_on_start_tasks = schedule_unsequenced_tasks(
        &weighted_unsequenced_on_start_tasks,
        total_unsequenced_on_start_tasks,
        scheduler,
    );
    let scheduled_unsequenced_tasks = schedule_unsequenced_tasks(
        &weighted_unsequenced_tasks,
        total_unsequenced_tasks,
        scheduler,
    );
    let scheduled_unsequenced_on_stop_tasks = schedule_unsequenced_tasks(
        &weighted_unsequenced_on_stop_tasks,
        total_unsequenced_on_stop_tasks,
        scheduler,
    );

    // Finally build a Vector of tuples: (task id, task name)
    let mut on_start_tasks = Vec::new();
    let mut tasks = Vec::new();
    let mut on_stop_tasks = Vec::new();

    // Sequenced tasks come first.
    for task in scheduled_sequenced_on_start_tasks.iter() {
        on_start_tasks.extend(vec![(*task, task_set.tasks[*task].name.to_string())])
    }
    for task in scheduled_sequenced_tasks.iter() {
        tasks.extend(vec![(*task, task_set.tasks[*task].name.to_string())])
    }
    for task in scheduled_sequenced_on_stop_tasks.iter() {
        on_stop_tasks.extend(vec![(*task, task_set.tasks[*task].name.to_string())])
    }

    // Unsequenced tasks come last.
    for task in scheduled_unsequenced_on_start_tasks.iter() {
        on_start_tasks.extend(vec![(*task, task_set.tasks[*task].name.to_string())])
    }
    for task in scheduled_unsequenced_tasks.iter() {
        tasks.extend(vec![(*task, task_set.tasks[*task].name.to_string())])
    }
    for task in scheduled_unsequenced_on_stop_tasks.iter() {
        on_stop_tasks.extend(vec![(*task, task_set.tasks[*task].name.to_string())])
    }

    // Return sequenced buckets of weighted usize pointers to and names of Swanling Tasks
    (on_start_tasks, tasks, on_stop_tasks)
}

/// Build a weighted vector of vectors of unsequenced SwanlingTasks.
fn weight_unsequenced_tasks(
    unsequenced_tasks: &[SwanlingTask],
    u: usize,
) -> (Vec<Vec<usize>>, usize) {
    // Build a vector of vectors to be used to schedule users.
    let mut available_unsequenced_tasks = Vec::with_capacity(unsequenced_tasks.len());
    let mut total_tasks = 0;
    for task in unsequenced_tasks.iter() {
        // divide by greatest common divisor so vector is as short as possible
        let weight = task.weight / u;
        trace!(
            "{}: {} has weight of {} (reduced with gcd to {})",
            task.tasks_index,
            task.name,
            task.weight,
            weight
        );
        let weighted_tasks = vec![task.tasks_index; weight];
        available_unsequenced_tasks.push(weighted_tasks);
        total_tasks += weight;
    }
    (available_unsequenced_tasks, total_tasks)
}

/// Build a weighted vector of vectors of sequenced SwanlingTasks.
fn weight_sequenced_tasks(
    sequenced_tasks: &SequencedSwanlingTasks,
    u: usize,
) -> BTreeMap<usize, Vec<Vec<usize>>> {
    // Build a sequenced BTreeMap containing weighted vectors of SwanlingTasks.
    let mut available_sequenced_tasks = BTreeMap::new();
    // Step through sequences, each containing a bucket of all SwanlingTasks with the same
    // sequence value, allowing actual weighting to be done by weight_unsequenced_tasks().
    for (sequence, unsequenced_tasks) in sequenced_tasks.iter() {
        let (weighted_tasks, _total_weighted_tasks) =
            weight_unsequenced_tasks(&unsequenced_tasks, u);
        available_sequenced_tasks.insert(*sequence, weighted_tasks);
    }

    available_sequenced_tasks
}

fn schedule_sequenced_tasks(
    available_sequenced_tasks: &BTreeMap<usize, Vec<Vec<usize>>>,
    scheduler: &SwanlingScheduler,
) -> Vec<usize> {
    let mut weighted_tasks: Vec<usize> = Vec::new();

    for (_sequence, tasks) in available_sequenced_tasks.iter() {
        let scheduled_tasks = schedule_unsequenced_tasks(tasks, tasks[0].len(), scheduler);
        weighted_tasks.extend(scheduled_tasks);
    }

    weighted_tasks
}

// Return a list of tasks in the order to be run.
fn schedule_unsequenced_tasks(
    available_unsequenced_tasks: &[Vec<usize>],
    total_tasks: usize,
    scheduler: &SwanlingScheduler,
) -> Vec<usize> {
    // Now build the weighted list with the appropriate scheduler.
    let mut weighted_tasks = Vec::new();

    match scheduler {
        SwanlingScheduler::RoundRobin => {
            // Allocate task sets round robin.
            let tasks_len = available_unsequenced_tasks.len();
            let mut available_tasks = available_unsequenced_tasks.to_owned();
            loop {
                // Tasks are contained in a vector of vectors. The outer vectors each
                // contain a different SwanlingTask, and the inner vectors contain each
                // instance of that specific SwanlingTask.
                for (task_index, tasks) in available_tasks.iter_mut().enumerate().take(tasks_len) {
                    if let Some(task) = tasks.pop() {
                        debug!("allocating task from Task {}", task_index);
                        weighted_tasks.push(task);
                    }
                }
                if weighted_tasks.len() >= total_tasks {
                    break;
                }
            }
        }
        SwanlingScheduler::Serial | SwanlingScheduler::Random => {
            // Allocate task sets serially in the weighted order defined. If the Random
            // scheduler is being used, tasks will get shuffled later.
            for (task_index, tasks) in available_unsequenced_tasks.iter().enumerate() {
                debug!(
                    "allocating all {} tasks from Task {}",
                    tasks.len(),
                    task_index
                );

                let mut tasks_clone = tasks.clone();
                if scheduler == &SwanlingScheduler::Random {
                    tasks_clone.shuffle(&mut thread_rng());
                }
                weighted_tasks.append(&mut tasks_clone);
            }
        }
    }

    weighted_tasks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_defaults() {
        let host = "http://example.com/".to_string();
        let users: usize = 10;
        let run_time: usize = 10;
        let hatch_rate = "2".to_string();
        let log_level: usize = 1;
        let swanling_log = "custom-swanling.log".to_string();
        let verbose: usize = 0;
        let report_file = "custom-swanling-report.html".to_string();
        let request_log = "custom-swanling-request.log".to_string();
        let task_log = "custom-swanling-task.log".to_string();
        let debug_log = "custom-swanling-debug.log".to_string();
        let error_log = "custom-swanling-error.log".to_string();
        let throttle_requests: usize = 25;
        let expect_workers: usize = 5;
        let manager_bind_host = "127.0.0.1".to_string();
        let manager_bind_port: usize = 1221;
        let manager_host = "127.0.0.1".to_string();
        let manager_port: usize = 1221;

        let swanling_attack = SwanlingAttack::initialize()
            .unwrap()
            .set_default(SwanlingDefault::Host, host.as_str())
            .unwrap()
            .set_default(SwanlingDefault::Users, users)
            .unwrap()
            .set_default(SwanlingDefault::RunTime, run_time)
            .unwrap()
            .set_default(SwanlingDefault::HatchRate, hatch_rate.as_str())
            .unwrap()
            .set_default(SwanlingDefault::LogLevel, log_level)
            .unwrap()
            .set_default(SwanlingDefault::SwanlingLog, swanling_log.as_str())
            .unwrap()
            .set_default(SwanlingDefault::Verbose, verbose)
            .unwrap()
            .set_default(SwanlingDefault::RunningMetrics, 15)
            .unwrap()
            .set_default(SwanlingDefault::NoResetMetrics, true)
            .unwrap()
            .set_default(SwanlingDefault::NoMetrics, true)
            .unwrap()
            .set_default(SwanlingDefault::NoTaskMetrics, true)
            .unwrap()
            .set_default(SwanlingDefault::NoErrorSummary, true)
            .unwrap()
            .set_default(SwanlingDefault::NoTelnet, true)
            .unwrap()
            .set_default(SwanlingDefault::NoWebSocket, true)
            .unwrap()
            .set_default(SwanlingDefault::NoAutoStart, true)
            .unwrap()
            .set_default(SwanlingDefault::ReportFile, report_file.as_str())
            .unwrap()
            .set_default(SwanlingDefault::RequestLog, request_log.as_str())
            .unwrap()
            .set_default(SwanlingDefault::RequestFormat, SwanlingLogFormat::Raw)
            .unwrap()
            .set_default(SwanlingDefault::TaskLog, task_log.as_str())
            .unwrap()
            .set_default(SwanlingDefault::TaskFormat, SwanlingLogFormat::Raw)
            .unwrap()
            .set_default(SwanlingDefault::ErrorLog, error_log.as_str())
            .unwrap()
            .set_default(SwanlingDefault::ErrorFormat, SwanlingLogFormat::Csv)
            .unwrap()
            .set_default(SwanlingDefault::DebugLog, debug_log.as_str())
            .unwrap()
            .set_default(SwanlingDefault::DebugFormat, SwanlingLogFormat::Csv)
            .unwrap()
            .set_default(SwanlingDefault::NoDebugBody, true)
            .unwrap()
            .set_default(SwanlingDefault::StatusCodes, true)
            .unwrap()
            .set_default(
                SwanlingDefault::CoordinatedOmissionMitigation,
                SwanlingCoordinatedOmissionMitigation::Disabled,
            )
            .unwrap()
            .set_default(SwanlingDefault::ThrottleRequests, throttle_requests)
            .unwrap()
            .set_default(SwanlingDefault::StickyFollow, true)
            .unwrap()
            .set_default(SwanlingDefault::Manager, true)
            .unwrap()
            .set_default(SwanlingDefault::ExpectWorkers, expect_workers)
            .unwrap()
            .set_default(SwanlingDefault::NoHashCheck, true)
            .unwrap()
            .set_default(SwanlingDefault::ManagerBindHost, manager_bind_host.as_str())
            .unwrap()
            .set_default(SwanlingDefault::ManagerBindPort, manager_bind_port)
            .unwrap()
            .set_default(SwanlingDefault::Worker, true)
            .unwrap()
            .set_default(SwanlingDefault::ManagerHost, manager_host.as_str())
            .unwrap()
            .set_default(SwanlingDefault::ManagerPort, manager_port)
            .unwrap();

        assert!(swanling_attack.defaults.host == Some(host));
        assert!(swanling_attack.defaults.users == Some(users));
        assert!(swanling_attack.defaults.run_time == Some(run_time));
        assert!(swanling_attack.defaults.hatch_rate == Some(hatch_rate));
        assert!(swanling_attack.defaults.log_level == Some(log_level as u8));
        assert!(swanling_attack.defaults.swanling_log == Some(swanling_log));
        assert!(swanling_attack.defaults.no_debug_body == Some(true));
        assert!(swanling_attack.defaults.verbose == Some(verbose as u8));
        assert!(swanling_attack.defaults.running_metrics == Some(15));
        assert!(swanling_attack.defaults.no_reset_metrics == Some(true));
        assert!(swanling_attack.defaults.no_metrics == Some(true));
        assert!(swanling_attack.defaults.no_task_metrics == Some(true));
        assert!(swanling_attack.defaults.no_error_summary == Some(true));
        assert!(swanling_attack.defaults.no_telnet == Some(true));
        assert!(swanling_attack.defaults.no_websocket == Some(true));
        assert!(swanling_attack.defaults.no_autostart == Some(true));
        assert!(swanling_attack.defaults.report_file == Some(report_file));
        assert!(swanling_attack.defaults.request_log == Some(request_log));
        assert!(swanling_attack.defaults.request_format == Some(SwanlingLogFormat::Raw));
        assert!(swanling_attack.defaults.error_log == Some(error_log));
        assert!(swanling_attack.defaults.error_format == Some(SwanlingLogFormat::Csv));
        assert!(swanling_attack.defaults.debug_log == Some(debug_log));
        assert!(swanling_attack.defaults.debug_format == Some(SwanlingLogFormat::Csv));
        assert!(swanling_attack.defaults.status_codes == Some(true));
        assert!(
            swanling_attack.defaults.co_mitigation
                == Some(SwanlingCoordinatedOmissionMitigation::Disabled)
        );
        assert!(swanling_attack.defaults.throttle_requests == Some(throttle_requests));
        assert!(swanling_attack.defaults.sticky_follow == Some(true));
        assert!(swanling_attack.defaults.manager == Some(true));
        assert!(swanling_attack.defaults.expect_workers == Some(expect_workers as u16));
        assert!(swanling_attack.defaults.no_hash_check == Some(true));
        assert!(swanling_attack.defaults.manager_bind_host == Some(manager_bind_host));
        assert!(swanling_attack.defaults.manager_bind_port == Some(manager_bind_port as u16));
        assert!(swanling_attack.defaults.worker == Some(true));
        assert!(swanling_attack.defaults.manager_host == Some(manager_host));
        assert!(swanling_attack.defaults.manager_port == Some(manager_port as u16));
    }
}
