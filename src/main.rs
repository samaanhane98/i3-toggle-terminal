use tokio_i3ipc::{reply::Workspace, I3};

mod error;
use error::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut i3 = I3::connect().await?;
    let workspaces = i3.get_workspaces().await?;

    let active_workspace = workspaces
        .clone()
        .into_iter()
        .find(|workspace| workspace.focused)
        .ok_or("I3 error, no focussed workspace")?;

    let Workspace { rect, .. } = active_workspace;

    let mut msg_body = format!("[title=\"^dropdown\"] scratchpad show;");
    i3.send_msg_body(tokio_i3ipc::msg::Msg::RunCommand, msg_body)
        .await?;

    msg_body = format!(
        "[title=\"^dropdown\"] move absolute position {} 0 ppt",
        rect.x
    );
    i3.send_msg_body(tokio_i3ipc::msg::Msg::RunCommand, msg_body)
        .await?;

    msg_body = format!("[title=\"^dropdown\"] resize set {} 500", rect.width);
    i3.send_msg_body(tokio_i3ipc::msg::Msg::RunCommand, msg_body)
        .await?;

    Ok(())
}
