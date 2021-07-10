# Coordinated Omission Mitigation

https://www.scylladb.com/2021/04/22/on-coordinated-omission/

THIS IS AN EXPERIMENTAL FEATURE THAT IS DISABLED BY DEFAULT. The following documentation is a work in progress, and may currently contain inaccuracies.

When enabled, Swanling attempts to mitigate the loss of metrics data (Coordinated Omission) caused by an abnormally lengthy response to a request.

## Definition

To understand Coordinated Omission and how Swanling attempts to mitigate it, it's necessary to understand how Swanling is scheduling requests. Swanling launches one thread per `SwanlingUser`. Each `SwanlingUser` is assigned a single `SwanlingTaskSet`. Each of these `SwanlingUser` threads then loop repeatedly through all of the `SwanlingTasks` defined in the assigned `SwanlingTaskSet`, each of which can involve any number of individual requests. However, at any given time, each `SwanlingUser` is only making a single request and then asynchronously waiting for the response.

If something causes the response to a request to take abnormally long, raw Swanling metrics only see this slowdown as affecting a specific request (or one request per `SwanlingUser`). The slowdown can be caused by a variety of issues, such as a resource bottleneck (on the Swanling client or the web server), garbage collection, a cache stampede, or even a network issue. A real user loading the same web page would see a much larger effect, as all requests to the affected server would stall. Even static assets such as images and scripts hosted on a reliable and fast CDN can be affected, as the web browser won't know to load them until it first loads the HTML from the affected web server. Because Swanling is only making one request at a time per `SwanlingUser`, it may only see one or very few slow requests and then all other requests resume at normal speed. This results in a bias in the metrics to "ignore" or "hide" the true effect of a slowdown, commonly referred to as Coordinated Omission.

## Mitigation

Swanling attempts to mitigate Coordinated Omission by back-filling the metrics with the statistically expected requests. To do this, it tracks the normal "cadence" of each `SwanlingUser`, timing how long it takes to loop through all `SwanlingTasks` in the assigned `SwanlingTaskSet`. By default, Swanling will trigger Coordinated Omission Mitigation if the time to loop through a `SwanlingTaskSet` takes more than twice as long as the average time of all previous loops. In this case, on the next loop through the `SwanlingTaskSet` when tracking the actual metrics for each subsequent request in all `SwanlingTasks` it will also add in statistically generated "requests" with a `response_time` starting at the unexpectedly long request time, then again with that `response_time` minus the normal "cadence", continuing to generate a metric then subtract the normal "cadence" until arriving at the expected `response_time`. In this way, Swanling is able to estimate the actual effect of a slowdown.

When Swanling detects an abnormally slow request (one in which the individual request takes longer than the normal `user_cadence`), it will generate an INFO level message (which will be visible if Swanling was started with the `-v` run time flag, or written to the log if started with the `-g` run time flag and `--swanling-log` is configured). For example:

```
13:10:30 [INFO] 11.401s into swanling attack: "GET http://apache/node/1557" [200] took abnormally long (1814 ms), task name: "(Anon) node page"
13:10:30 [INFO] 11.450s into swanling attack: "GET http://apache/node/5016" [200] took abnormally long (1769 ms), task name: "(Anon) node page"
```

If the `--request-log` is enabled, you can get more details, in this case by looking for elapsed times matching the above messages, specifically 1814 and 1769 respectively:

```
{"coordinated_omission_elapsed":0,"elapsed":11401,"error":"","final_url":"http://apache/node/1557","method":"Get","name":"(Anon) node page","redirected":false,"response_time":1814,"status_code":200,"success":true,"update":false,"url":"http://apache/node/1557","user":2,"user_cadence":1727}
{"coordinated_omission_elapsed":0,"elapsed":11450,"error":"","final_url":"http://apache/node/5016","method":"Get","name":"(Anon) node page","redirected":false,"response_time":1769,"status_code":200,"success":true,"update":false,"url":"http://apache/node/5016","user":0,"user_cadence":1422}
```

In the requests file, you can see that two different user threads triggered Coordinated Omission Mitigation, specifically threads 2 and 0. Both `SwanlingUser` threads were loading the same `SwanlingTask` as due to task weighting this is the task loaded the most frequently. Both `SwanlingUser` threads loop through all `SwanlingTasks` in a similar amount of time: thread 2 takes on average 1.727 seconds, thread 0 takes on average 1.422 seconds.

Also if the `--request-log` is enabled, requests back-filled by Coordinated Omission Mitigation show up in the generated log file, even though they were not actually sent to the server. Normal requests not generated by Coordinated Omission Mitigation have a `coordinated_omission_elapsed` of 0.

Coordinated Omission Mitigation is disabled by default. This experimental feature can be enabled by enabling the `--co-mitigation` run time option when starting Swanling. It can be configured to use the `average`, `minimum`, or `maximum` `GoouseUser` cadence when backfilling statistics.

## Metrics

When Coordinated Omission Mitigation kicks in, Swanling tracks both the "raw" metrics and the "adjusted" metrics. It shows both together when displaying metrics, first the "raw" (actually seen) metrics, followed by the "adjusted" metrics. As the minimum response time is never changed by Coordinated Omission Mitigation, this column is replacd with the "standard deviation" between the average "raw" response time, and the average "adjusted" response time.

The following example was "contrived". The `drupal_loadtest` example was run for 15 seconds, and after 10 seconds the upstream Apache server was manually "paused" for 3 seconds, forcing some abnormally slow queries. (More specifically, the apache web server was started by running `. /etc/apache2/envvars && /usr/sbin/apache2 -DFOREGROUND`, it was "paused" by pressing `ctrl-z`, and it was resumed three seconds later by typing `fg`.) In the "PER REQUEST METRICS" Swanling shows first the "raw" metrics", followed by the "adjusted" metrics:

```
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 GET (Anon) front page    |       11.73 |          3 |          81 |         12
 GET (Anon) node page     |       81.76 |          5 |       3,390 |         37
 GET (Anon) user page     |       27.53 |         16 |          94 |         26
 GET (Auth) comment form  |       35.27 |         24 |          50 |         35
 GET (Auth) front page    |       30.68 |         20 |         111 |         26
 GET (Auth) node page     |       97.79 |         23 |       3,326 |         35
 GET (Auth) user page     |       25.20 |         21 |          30 |         25
 GET static asset         |        9.27 |          2 |          98 |          6
 POST (Auth) comment form |       52.47 |         43 |          59 |         52
 -------------------------+-------------+------------+-------------+-----------
 Aggregated               |       17.04 |          2 |       3,390 |          8
 ------------------------------------------------------------------------------
 Adjusted for Coordinated Omission:
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |    Std Dev |         Max |     Median
 ------------------------------------------------------------------------------
 GET (Anon) front page    |      419.82 |     288.56 |       3,153 |         14
 GET (Anon) node page     |      464.72 |     270.80 |       3,390 |         40
 GET (Anon) user page     |      420.48 |     277.86 |       3,133 |         27
 GET (Auth) comment form  |      503.38 |     331.01 |       2,951 |         37
 GET (Auth) front page    |      489.99 |     324.78 |       2,960 |         33
 GET (Auth) node page     |      530.29 |     305.82 |       3,326 |         37
 GET (Auth) user page     |      500.67 |     336.21 |       2,959 |         27
 GET static asset         |      427.70 |     295.87 |       3,154 |          9
 POST (Auth) comment form |      512.14 |     325.04 |       2,932 |         55
 -------------------------+-------------+------------+-------------+-----------
 Aggregated               |      432.98 |     294.11 |       3,390 |         14
 ```

From these two tables, it is clear that there was a statistically significant event affecting the load testing metrics. In particular, note that the standard deviation between the "raw" average and the "adjusted" average is considerably larger than the "raw" average, calling into questing whether or not your load test was "valid". (The answer to that question depends very much on your specific goals and load test.)

Swanling also shows multiple percentile graphs, again showing first the "raw" metrics followed by the "adjusted" metrics. The "raw" graph would suggest that less than 1% of the requests for the `GET (Anon) node page` were slow, and less than 0.1% of the requests for the `GET (Auth) node page` were slow. However, through Coordinated Omission Mitigation we can see that statistically this would have actually affected all requests, and for authenticated users the impact is visible on >25% of the requests.

```
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 GET (Anon) front page    |     12 |     15 |     25 |     27 |     81 |     81
 GET (Anon) node page     |     37 |     43 |     60 |  3,000 |  3,000 |  3,000
 GET (Anon) user page     |     26 |     28 |     34 |     93 |     94 |     94
 GET (Auth) comment form  |     35 |     37 |     50 |     50 |     50 |     50
 GET (Auth) front page    |     26 |     34 |     45 |     88 |    110 |    110
 GET (Auth) node page     |     35 |     38 |     58 |     58 |  3,000 |  3,000
 GET (Auth) user page     |     25 |     27 |     30 |     30 |     30 |     30
 GET static asset         |      6 |     14 |     21 |     22 |     81 |     98
 POST (Auth) comment form |     52 |     55 |     59 |     59 |     59 |     59
 -------------------------+--------+--------+--------+--------+--------+-------
 Aggregated               |      8 |     16 |     47 |     53 |  3,000 |  3,000
 ------------------------------------------------------------------------------
 Adjusted for Coordinated Omission:
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 GET (Anon) front page    |     14 |     21 |  3,000 |  3,000 |  3,000 |  3,000
 GET (Anon) node page     |     40 |     55 |  3,000 |  3,000 |  3,000 |  3,000
 GET (Anon) user page     |     27 |     32 |  3,000 |  3,000 |  3,000 |  3,000
 GET (Auth) comment form  |     37 |    400 |  2,951 |  2,951 |  2,951 |  2,951
 GET (Auth) front page    |     33 |    410 |  2,960 |  2,960 |  2,960 |  2,960
 GET (Auth) node page     |     37 |    410 |  3,000 |  3,000 |  3,000 |  3,000
 GET (Auth) user page     |     27 |    420 |  2,959 |  2,959 |  2,959 |  2,959
 GET static asset         |      9 |     20 |  3,000 |  3,000 |  3,000 |  3,000
 POST (Auth) comment form |     55 |    390 |  2,932 |  2,932 |  2,932 |  2,932
 -------------------------+--------+--------+--------+--------+--------+-------
 Aggregated               |     14 |     42 |  3,000 |  3,000 |  3,000 |  3,000
 ```

 The Coordinated Omission metrics will also show up in the HTML report generated when Swanling is started with the `--report-file` run-time option. If Coordinated Omission mitigation kicked in, the HTML report will include both the "raw" metrics and the "adjusted" metrics.
