//! Simple Swanling load test example using closures.
//!
//! ## License
//!
//! Copyright 2020 Fabian Franz
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

use std::boxed::Box;
use std::sync::Arc;

fn main() -> Result<(), SwanlingError> {
    let mut taskset = taskset!("WebsiteUser")
        // After each task runs, sleep randomly from 5 to 15 seconds.
        .set_wait_time(5, 15)?;

    let paths = vec!["/", "/about", "/our-team"];
    for request_path in paths {
        let path = request_path;

        let closure: SwanlingTaskFunction = Arc::new(move |user| {
            Box::pin(async move {
                let _swanling = user.get(path).await?;

                Ok(())
            })
        });

        let task = SwanlingTask::new(closure);
        // We need to do the variable dance as taskset.register_task returns self and hence moves
        // self out of `taskset`. By storing it in a new local variable and then moving it over
        // we can avoid that error.
        let new_taskset = taskset.register_task(task);
        taskset = new_taskset;
    }

    SwanlingAttack::initialize()?
        // In this example, we only create a single taskset, named "WebsiteUser".
        .register_taskset(taskset)
        .execute()?
        .print();

    Ok(())
}
