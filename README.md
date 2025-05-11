## use paste crate to resolve enum_dispatch inner scope issue

The error "cannot find value `inner` in this scope" occurs because enum_dispatch
creates an inner identifier during macro expansion, but it's not properly scoped.
The paste crate helps by creating hygienic identifiers during macro expansion,
ensuring the inner value is available in the correct scope.

This is a more robust solution than relying on trait definition order, as it
properly handles macro hygiene and identifier scoping.
