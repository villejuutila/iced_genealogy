use graph::{
    node::{
        genealogical_node::{GenealogicalNode, Sex},
        GraphNodeTrait,
    },
    GraphMessage,
};
use iced::{
    widget::{button, checkbox, column, container, text, text_input, Column, Container},
    Background, Border, Color,
    Length::Fill,
    Shadow,
};

use crate::Message;

pub fn side_panel<'a>(selected_node: Option<&'a GenealogicalNode>) -> Container<'a, Message> {
    let mut root = column![
        text("Graph").color(Color::BLACK),
        button("Add new")
            .width(Fill)
            .on_press(Message::Graph(GraphMessage::InsertNode(None)))
    ];

    if let Some(selected_node) = selected_node {
        root = select_node_content(root, selected_node);
    }

    let root = container(root)
        .width(300)
        .padding(10)
        .height(Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(iced::Color::WHITE)),
            shadow: Shadow { ..Default::default() },
            border: Border {
                radius: 5.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    root
}

fn select_node_content<'a>(mut root: Column<'a, Message>, node: &'a GenealogicalNode) -> Column<'a, Message> {
    let selected_node_widgets: Column<'a, Message> = column![
        text("Selected node").color(Color::BLACK),
        text_input("Input persons name", &node.first_name().unwrap_or("".to_string()))
            .on_input(move |input| Message::UpdateNodeName((node.id(), input)))
            .padding(10)
            .size(20)
            .width(Fill),
        text("Sex").color(Color::BLACK),
        checkbox("Male", node.sex().map_or(false, |sex| sex == Sex::Male))
            .on_toggle(|checked| { Message::SetNodeSex((node.id(), if checked { Sex::Male } else { Sex::Female })) }),
        checkbox("Female", node.sex().map_or(false, |sex| sex == Sex::Female))
            .on_toggle(|checked| { Message::SetNodeSex((node.id(), if checked { Sex::Female } else { Sex::Male })) }),
        button("Add offspring")
            .width(Fill)
            .on_press(Message::Graph(GraphMessage::InsertNode(Some(node.id())))),
    ];
    root = root.push(selected_node_widgets);
    root
}
