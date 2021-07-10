//! Swanling complete load test example.
//!
//! ## License
//!
//! Copyright 2021 Begley Brothers Ltd
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use swanling::prelude::*;

fn main() -> Result<(), SwanlingError> {
    SwanlingAttack::initialize()?
        // We only create the "TheInternetUser" taskset.
        .register_taskset(
            taskset!("TheInternetUser")
                // Load when the Swanling load test first starts.
                .register_task(task!(website_login).set_on_start())
                // These tasks repeat, until the Swanling load test ends.
                .register_task(task!(website_slow))
                .register_task(task!(website_redirect)),
        )
        .execute()?
        .print();

    Ok(())
}

/// Demonstrates how to log in when a user starts.
/// This task is set as an on_start task when registering it above.
/// This means it only runs when the "user" thread starts.
async fn website_login(user: &SwanlingUser) -> SwanlingTaskResult {
    let request_builder = user.swanling_post("/login").await?;
    // https://docs.rs/reqwest/*/reqwest/blocking/struct.RequestBuilder.html#method.form
    let params = [("username", "tomsmith"), ("password", "SuperSecretPassword!")];
    let _swanling = user.swanling_send(request_builder.form(&params), None).await?;

    Ok(())
}

/// A task to load the slow resources page.
async fn website_slow(user: &SwanlingUser) -> SwanlingTaskResult {
    let _swanling = user.get("/slow").await?;

    Ok(())
}

/// A task to load the redirect page.
async fn website_redirect(user: &SwanlingUser) -> SwanlingTaskResult {
    let _swanling = user.get("/redirect").await?;

    Ok(())
}
