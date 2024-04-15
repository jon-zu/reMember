#[macro_export]
macro_rules! handler {
    ($op:ident, $ctx:ident, $pr:ident, $this:ident, $default:ident, $($req:ty => $handler:ident),*) => {
        match $op {
            $(
                <$req>::OPCODE => $this.$handler($ctx, <$req>::decode_packet(&mut $pr)?).await,
            )*
            _ => $this.$default($ctx, $op, $pr).await
        }
    };
}