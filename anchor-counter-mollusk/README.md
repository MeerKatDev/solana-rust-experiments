# Anchor + Mollusk

This was much more complex than the others, due to the structure of Anchor.

When using 
```
 ...
pub struct Initialize<'info> {
    #[account(init,
        payer = user,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
 ...
```
and we try to initialize, we will enter a conflict since Anchor is trying to initialize too, resulting in
```
[2025-07-16T12:55:52.018618000Z DEBUG solana_runtime::message_processor::stable_log] Program GrUZEe4fs4dWW4bm9xGe9YuTJT2xESx29BksT1AX6ueM failed: Unsupported program id
```
because they will deploy two different program ids (or the same?).

When we want to Initialize manually from tests, it's instead more proper to use `init_if_needed`, because anchor won't initialize if it's already been initialized.
```
 ...
pub struct Initialize<'info> {
    #[account(init_if_needed, // this changes
        payer = user,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
 ...
```
and we should see
```
[2025-07-16T13:46:56.131333000Z DEBUG solana_runtime::message_processor::stable_log] Program log: Counter initialized at 0
[2025-07-16T13:46:56.131504000Z DEBUG solana_runtime::message_processor::stable_log] Program GrUZEe4fs4dWW4bm9xGe9YuTJT2xESx29BksT1AX6ueM consumed 4723 of 1400000 compute units
[2025-07-16T13:46:56.131547000Z DEBUG solana_runtime::message_processor::stable_log] Program GrUZEe4fs4dWW4bm9xGe9YuTJT2xESx29BksT1AX6ueM success
 ...
```