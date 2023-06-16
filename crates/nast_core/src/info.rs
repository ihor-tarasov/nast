use std::borrow::Cow;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ContentKind {
    Empty,
    Number,
    Identifier,
}

#[derive(Clone, Serialize)]
pub struct Info {
    pub name: Cow<'static, str>,
    pub trigger: bool,
    pub output: bool,
    pub flows: Cow<'static, [Cow<'static, str>]>,
    pub inputs: Cow<'static, [Cow<'static, str>]>,
    pub content: ContentKind,
    pub description: Cow<'static, str>,
}

pub const STD_INFOS: &'static [Info] = &[
    Info {
        name: Cow::Borrowed("Argument"),
        trigger: false,
        output: true,
        flows: Cow::Borrowed(&[]),
        inputs: Cow::Borrowed(&[]),
        content: ContentKind::Identifier,
        description: Cow::Borrowed("Returns value of function argument."),
    },
    Info {
        name: Cow::Borrowed("Subtract"),
        trigger: false,
        output: true,
        flows: Cow::Borrowed(&[]),
        inputs: Cow::Borrowed(&[Cow::Borrowed("left"), Cow::Borrowed("right")]),
        content: ContentKind::Empty,
        description: Cow::Borrowed("Perform 'left' - 'right' binary operation."),
    },
    Info {
        name: Cow::Borrowed("Multiply"),
        trigger: false,
        output: true,
        flows: Cow::Borrowed(&[]),
        inputs: Cow::Borrowed(&[Cow::Borrowed("left"), Cow::Borrowed("right")]),
        content: ContentKind::Empty,
        description: Cow::Borrowed("Perform 'left' * 'right' binary operation."),
    },
    Info {
        name: Cow::Borrowed("LessEquals"),
        trigger: false,
        output: true,
        flows: Cow::Borrowed(&[]),
        inputs: Cow::Borrowed(&[Cow::Borrowed("left"), Cow::Borrowed("right")]),
        content: ContentKind::Empty,
        description: Cow::Borrowed("Perform 'left' <= 'right' binary operation."),
    },
    Info {
        name: Cow::Borrowed("If"),
        trigger: true,
        output: false,
        flows: Cow::Borrowed(&[Cow::Borrowed("then"), Cow::Borrowed("else")]),
        inputs: Cow::Borrowed(&[Cow::Borrowed("condition")]),
        content: ContentKind::Empty,
        description: Cow::Borrowed(
            "If 'condition' returns 'true' execute 'then', execute 'else' in other case.",
        ),
    },
    Info {
        name: Cow::Borrowed("Number"),
        trigger: false,
        output: true,
        flows: Cow::Borrowed(&[]),
        inputs: Cow::Borrowed(&[]),
        content: ContentKind::Number,
        description: Cow::Borrowed("Returns number constant."),
    },
    Info {
        name: Cow::Borrowed("Return"),
        trigger: true,
        output: false,
        flows: Cow::Borrowed(&[]),
        inputs: Cow::Borrowed(&[Cow::Borrowed("result")]),
        content: ContentKind::Empty,
        description: Cow::Borrowed("Returns from function."),
    },
    Info {
        name: Cow::Borrowed("Start"),
        trigger: false,
        output: false,
        flows: Cow::Borrowed(&[Cow::Borrowed("flow")]),
        inputs: Cow::Borrowed(&[]),
        content: ContentKind::Empty,
        description: Cow::Borrowed("Entry point of function."),
    },
];
