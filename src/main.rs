use swayipc::{reply::Node, Connection, Fallible};

fn main() -> Fallible<()> {
    fn find_focused_self(node: &mut Node) -> Option<&mut Node> {
        if node.focused {
            Some(node)
        } else {
            if let Some(want) = node.focus.get(0) {
                let child = node.nodes.iter_mut().find(|n| *want == n.id).unwrap();
                find_focused_self(child)
            } else {
                None
            }
        }
    }
    let mut connection = Connection::new()?;
    if find_focused_self(&mut connection.get_tree().unwrap())
        .unwrap()
        .border
        == swayipc::reply::NodeBorder::Normal
    {
        //println!("{:?}", connection.run_command("border none"));
        connection.run_command("border none")?;
    } else {
        //println!("{:?}", connection.run_command("border normal"));
        connection.run_command("border normal")?;
    };
    Ok(())
}
