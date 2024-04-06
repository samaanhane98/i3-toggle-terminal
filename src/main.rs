use std::rc::Rc;
use tokio_i3ipc::{reply::Workspace, I3};

mod error;
use error::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut i3 = I3::connect().await?;

    let msg_body = format!("[title=\"^dropdown\"] scratchpad show; move absolute position 0 1 ppt; resize set height 35 ppt");
    i3.send_msg_body(tokio_i3ipc::msg::Msg::RunCommand, msg_body)
        .await?;

    Ok(())
}
