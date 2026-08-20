## Known: the SSH host listens on every interface, not only the trusted one

Recorded so it stays findable, and so a future review does not report it again as
a new finding. **Not planned** — see the weighing below.

### What it is

`set_enabled` opens the listener when `netgate::on_trusted_network()` says *any*
current network is trusted, and then binds all of them:

```rust
tokio::net::TcpListener::bind(("0.0.0.0", port))   // sshhost.rs
```

`netgate::trusted_ipv4()` already resolves the address of the trusted adapter —
`discovery.rs` uses it to decide what to advertise over mDNS — but the socket
itself does not use it. So a laptop on a trusted Ethernet *and* a public hotspot
is reachable on port 8287 from the hotspot too, even though it is never announced
there.

### Why it is not urgent

Nothing gets in unattended. An unknown key hits the pairing popup in
`auth_publickey` and a human has to approve it; a blocked fingerprint is rejected
without a popup. So this is defence in depth, not an open door.

What does leak is the banner: `SSH-2.0-Taurus_<version>` goes to anyone who can
reach the port. On a public network that announces what runs here and in which
version, which is what makes a version-specific weakness targetable in the first
place.

### Why it is not a one-liner

- `trusted_ipv4()` returns only the *first* trusted address. Bind to that and a
  second trusted adapter becomes unreachable, so it needs one listener per
  trusted address.
- A DHCP renewal changes the address under a bound socket. The 15-second
  `watch_network` poll would have to compare the bound addresses against the
  current trusted ones and rebind, not just open and close on trust changes.

Both are doable; neither is free, and the gain is narrow. Picked up when the
listener is touched for another reason.

Found by a code review (`input/review.md`, 19 Aug 2026) as point 1.3. The other
valid points from that review are in #179.
