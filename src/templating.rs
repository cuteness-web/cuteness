use handlebars::{Handlebars, TemplateError};

#[inline(always)]
pub(crate) fn routing_template(reg: &mut Handlebars) -> Result<(), Box<TemplateError>> {
    Ok(reg.register_template_file("routing_template", "./serverside/routing.hbs")?)
}
