# 0.18.0
## Overview
This release focuses on fixes and improvements, with addition of ordered Push Consumer.

## Breaking Changes
* Refactor callbacks by @Jarema in https://github.com/nats-io/nats.rs/pull/595

## Added
* Add `get_last_raw_message_by_subject` to `Stream` by @caspervonb in https://github.com/nats-io/nats.rs/pull/584
* Add `ClusterInfo` and `PeerInfo` by @Jarema in https://github.com/nats-io/nats.rs/pull/572
* Add ordered  push consumer by @Jarema in https://github.com/nats-io/nats.rs/pull/574
* Add concurrent example by @Jarema in https://github.com/nats-io/nats.rs/pull/580
* Add delete message from stream by @Jarema in https://github.com/nats-io/nats.rs/pull/588
* Add Nkey authorization support by @neogenie  https://github.com/nats-io/nats.rs/pull/593

## Fixed
* Fix ordered consumer after discard policy hit by @Jarema in https://github.com/nats-io/nats.rs/pull/585
* Fix pull consumer stream method when batch is set to 1 by @Jarema in https://github.com/nats-io/nats.rs/pull/590
* Fix reconnect auth deadlock by @caspervonb in https://github.com/nats-io/nats.rs/pull/578

# 0.17.0
## Overview
This release focuses on two main things:
* Refactor of JetStream API
* Fix of slow connect (thanks @brooksmtownsend for reporting this!)

The changes in JetStream API make usage of builder more intuitive and seamless.
Before, you had to call
```rust
// before changes
let messages = consumer.stream().await?;
// or use a shortcut
let messages = consumer.messages().await?;

// after changes
let messages = consumer.stream().messages().await?;
// or with options
let messages = consumer.stream().max_bytes_per_bytes(1024).messages().await?;
```

## Changed
* Rename push consumer `Stream` iterator to `Messages` by @caspervonb in https://github.com/nats-io/nats.rs/pull/566
* Add pull builder for Fetch and Batch by @Jarema in https://github.com/nats-io/nats.rs/pull/565

## Fixed
* Fix slow connect in no-auth scenarios by @Jarema in https://github.com/nats-io/nats.rs/pull/568

## Other
* Fix license headers by @Jarema in https://github.com/nats-io/nats.rs/pull/564
* Add missing module docs headers by @Jarema in https://github.com/nats-io/nats.rs/pull/563
* Remove fault injection run from workflow by @caspervonb in https://github.com/nats-io/nats.rs/pull/567


**Full Changelog**: https://github.com/nats-io/nats.rs/compare/async-nats/v0.16.0...async-nats/v0.17.0

# 0.16.0

This release features a lot of improvements and additions to `JetStream` API and adds `Push Consumer`.

## Added
* Add `query_account` to `jetstream::Context` by @caspervonb in https://github.com/nats-io/nats.rs/pull/528
* Add streams to push consumers by @caspervonb in https://github.com/nats-io/nats.rs/pull/527
* Add no_echo option by @Jarema in https://github.com/nats-io/nats.rs/pull/560
* Add `jetstream::Stream::get_raw_message` by @caspervonb in https://github.com/nats-io/nats.rs/pull/484
* Add Pull Consumer builder by @Jarema in https://github.com/nats-io/nats.rs/pull/541

## Changed
* Allow unknown directives to be skipped when parsing by @caspervonb in https://github.com/nats-io/nats.rs/pull/514
* Narrow error type returned from client publishing by @caspervonb in https://github.com/nats-io/nats.rs/pull/525
* Change `create_consumer` to return `Consumer` by @Jarema in https://github.com/nats-io/nats.rs/pull/544
* Switch webpki to rustls-native-certs by @Jarema in https://github.com/nats-io/nats.rs/pull/558
* Normalize error type used in subscribe methods by @caspervonb in https://github.com/nats-io/nats.rs/pull/524
* Optimize `jetstream::consumer::pull::Consumer::stream` method. by @Jarema in https://github.com/nats-io/nats.rs/pull/529
* Make `deliver_subject` required for `push::Config` by @caspervonb in https://github.com/nats-io/nats.rs/pull/531

## Fixed
* Handle missing error cases in Stream by @Jarema in https://github.com/nats-io/nats.rs/pull/542
* Handle connecting to ipv6 addresses correctly by @jszwedko in https://github.com/nats-io/nats.rs/pull/386

## Other
* Move `Client` into its own source file by @caspervonb in https://github.com/nats-io/nats.rs/pull/523
* Extract `jetstream::Message` into its own module by @caspervonb in https://github.com/nats-io/nats.rs/pull/534
* Normalize introduction example by @caspervonb in https://github.com/nats-io/nats.rs/pull/540
* Fix documentation links by @Jarema in https://github.com/nats-io/nats.rs/pull/547
* Add more documentation to Pull Consumer by @Jarema in https://github.com/nats-io/nats.rs/pull/546
* Add Push Consumer stream docs by @Jarema in https://github.com/nats-io/nats.rs/pull/559
* Fix ack test race by @Jarema in https://github.com/nats-io/nats.rs/pull/555
* Add Message and Headers docs by @Jarema in https://github.com/nats-io/nats.rs/pull/548
* Remove trace and debug from nats-server wrapper by @Jarema in https://github.com/nats-io/nats.rs/pull/550

# 0.15.0

This release is the first `JetStream` 🍾  feature set for `async-nats`!

**It includes:**
* New simplified JetStream API approach
* JetStream Publish
* Streams management
* Consumers Management
* Pull Consumers implementation
* Ack's

Warning: JetStream support is experimental and may change

## Added
* Add JetStream types and basics by @Jarema in https://github.com/nats-io/nats.rs/pull/457
* Add get stream by @Jarema in https://github.com/nats-io/nats.rs/pull/458
* Add jetstream stream delete and stream update by @Jarema in https://github.com/nats-io/nats.rs/pull/459
* Add `async_nats::jetstream::Context::publish` by @caspervonb in https://github.com/nats-io/nats.rs/pull/460
* Add get_or_create JetStream management API by @Jarema in https://github.com/nats-io/nats.rs/pull/467
* Add domain and prefix by @Jarema in https://github.com/nats-io/nats.rs/pull/490
* Add error codes to `Response::Error` variant by @caspervonb in https://github.com/nats-io/nats.rs/pull/496
* Add JetStream ACK by @Jarema in https://github.com/nats-io/nats.rs/pull/515
* Add convinience methods to Consumer management by @Jarema in https://github.com/nats-io/nats.rs/pull/481
* Add Pull Consumer by @Jarema in https://github.com/nats-io/nats.rs/pull/479
* Add create consumer by @Jarema in https://github.com/nats-io/nats.rs/pull/471
* Add stream to pull consumers by @caspervonb in https://github.com/nats-io/nats.rs/pull/518
* Add Consumer::info and Consumer::cached_info by @Jarema in https://github.com/nats-io/nats.rs/pull/510
* Introduce a `StatusCode` type to represent statuses by @caspervonb in https://github.com/nats-io/nats.rs/pull/474
* Add example for multiple pub/subs in tasks by @Jarema in https://github.com/nats-io/nats.rs/pull/453
* Implement jetstream requests by @caspervonb in https://github.com/nats-io/nats.rs/pull/435
* Add `async_nats::jetstream::Context::publish_with_headers` by @caspervonb in https://github.com/nats-io/nats.rs/pull/462
* Implement `From<jetstream::Message>` for `Message` by @caspervonb in https://github.com/nats-io/nats.rs/pull/512
* Add get_or_create_consumer and delete_consumer by @Jarema in https://github.com/nats-io/nats.rs/pull/475
* Have No async-stream dependant implementation for Pull Consumers  by @caspervonb in https://github.com/nats-io/nats.rs/pull/499

## Changed
* Do not flush in write calls by @caspervonb in https://github.com/nats-io/nats.rs/pull/423
* Only retain non-closed subscriptions on reconnect by @caspervonb in https://github.com/nats-io/nats.rs/pull/454

## Fixed
* Fix off by one error that can occur parsing "HMSG" by @caspervonb in https://github.com/nats-io/nats.rs/pull/513
* Removed attempt to connect to server info host when TLS is enabled by @brooksmtownsend in https://github.com/nats-io/nats.rs/pull/500

# 0.14.0
## Added
* Add no responders handling by @Jarema in https://github.com/nats-io/nats.rs/pull/450
* Add client jwt authentication by @stevelr  in https://github.com/nats-io/nats.rs/pull/433
* Add lame duck mode support by @Jarema in https://github.com/nats-io/nats.rs/pull/438
* Add slow consumers support by @Jarema in https://github.com/nats-io/nats.rs/pull/444
* Add tracking maximum number of pending pings by @caspervonb  https://github.com/nats-io/nats.rs/pull/419

## Changed
* `Client` doesn't need to be mutable self by @stevelr  in https://github.com/nats-io/nats.rs/pull/434
* Make send buffer configurable by @Jarema in https://github.com/nats-io/nats.rs/pull/437

# 0.13.0
## Added
* Add Auth - username/password & token by @Jarema in https://github.com/nats-io/nats.rs/pull/408
* Support sending and receiving messages with headers by @caspervonb in https://github.com/nats-io/nats.rs/pull/402 
* Add async server errors callbacks by @Jarema in https://github.com/nats-io/nats.rs/pull/397
* Discover additional servers via INFO by @caspervonb in https://github.com/nats-io/nats.rs/pull/403
* Resolve socket addresses during connect by @caspervonb in https://github.com/nats-io/nats.rs/pull/403

## Changed
* Wait between reconnection attempts by @caspervonb in https://github.com/nats-io/nats.rs/pull/407
* Limit connection attempts by @caspervonb in https://github.com/nats-io/nats.rs/pull/400

## Other
* Remove redundant doctests by @Jarema in https://github.com/nats-io/nats.rs/pull/412
* Fix connection callback tests by @Jarema in https://github.com/nats-io/nats.rs/pull/420

# 0.12.0
## Added
* Add more examples and docs by @Jarema in https://github.com/nats-io/nats.rs/pull/372
* Add unsubscribe by @Jarema in https://github.com/nats-io/nats.rs/pull/363
* Add unsubscribe_after by @Jarema in https://github.com/nats-io/nats.rs/pull/385
* Add queue subscriber and unit test by @stevelr in https://github.com/nats-io/nats.rs/pull/388
* Implement reconnect by @caspervonb in https://github.com/nats-io/nats.rs/pull/382

## Other
* Fix test linter warnings by @caspervonb in https://github.com/nats-io/nats.rs/pull/379
* Fix tests failing with nats-server 2.8.0 by @Jarema in https://github.com/nats-io/nats.rs/pull/380
* Use local server for documentation tests by @caspervonb in https://github.com/nats-io/nats.rs/pull/377
* Improve workflow caching by @caspervonb in https://github.com/nats-io/nats.rs/pull/381
* Fix typo in README.md by @mgrachev in https://github.com/nats-io/nats.rs/pull/384
* Internal Architecture overhaul by @caspervonb and @Jarema 

# 0.11.0
Initial release of async NATS client rewrite.
The versioning starts from v0.11.0, as the Crate was used a long time ago by NATS.io org for some former work around async client.
