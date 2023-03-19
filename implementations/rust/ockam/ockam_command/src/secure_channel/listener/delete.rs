use clap::Args;

use ockam::Context;
use ockam_api::nodes::models::secure_channel::DeleteSecureChannelListenerRequest;
use ockam_core::api::Request;
use ockam_core::Address;

use super::common::SecureChannelListenerNodeOpts;
use crate::secure_channel::HELP_DETAIL;
use crate::util::{extract_address_value, node_rpc, Rpc};
use crate::{help, CommandGlobalOpts};

/// Create Secure Channel Listeners
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help = true, after_long_help = help::template(HELP_DETAIL))]
pub struct DeleteCommand {
    /// Address at which the channel listener to be deleted is running (required)
    address: Address,

    #[command(flatten)]
    node_opts: SecureChannelListenerNodeOpts,
}

impl DeleteCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        node_rpc(rpc, (opts, self));
    }
}

async fn rpc(ctx: Context, (opts, cmd): (CommandGlobalOpts, DeleteCommand)) -> crate::Result<()> {
    run_impl(&ctx, (opts, cmd)).await
}

async fn run_impl(
    ctx: &Context,
    (opts, cmd): (CommandGlobalOpts, DeleteCommand),
) -> crate::Result<()> {
    let at = &cmd.node_opts.at;

    let node = extract_address_value(at)?;
    let mut rpc = Rpc::background(ctx, &opts, &node)?;
    let req = Request::delete(
        format!("/node/secure_channel_listener/{}", &node));
    rpc.request(req).await?;
    rpc.is_ok()?;

    println!(
        "Deleted secure-channel listener with address '/service/{}' on node '{node}'",
        cmd.address.address()
    );
    Ok(())
}
