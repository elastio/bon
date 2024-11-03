# Typestate API

This section teaches you the builder's typestate API. It describes the underlying component traits and types of the builder type and how to use them.

Reading this is optional. The typestate API is private by default. The users of your builder can't denote its type unless you enable [`#[builder(state_mod(vis = "pub"))]`](../reference/builder/top-level/state_mod).

It is more of an advanced concept that you'll rarely need. However, "advanced" doesn't necessarily mean "complicated" in this case. So feel free to study this section if you feel like it.
