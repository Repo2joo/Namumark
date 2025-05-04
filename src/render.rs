use crate::parser::{Heading, Objects};

pub fn render_raw(vector:Vec<Objects>) -> String {
    let mut string = String::new();
    for object in vector {
        match object {
            Objects::RenderObject(render_object) => match render_object {
                crate::parser::RenderObject::Heading(heading) => {
                    let lvl = heading.get_lvl();
                    string.push_str(&format!("{}{}{}", "=".repeat(lvl.into()), render_raw(heading.get_render_objects()), "=".repeat(lvl.into())));
                },
                crate::parser::RenderObject::Literal(literal) => string.push_str(&literal.literal),
            },
            Objects::Tokens(tokens) => string.push_str(&tokens.to_string()),
        }
    }
    return string;
}