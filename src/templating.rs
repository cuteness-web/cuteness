use handlebars::{Handlebars, TemplateError};

#[inline(always)]
pub(crate) fn routing_template(reg: &mut Handlebars, ) -> Result<(), TemplateError> {
	reg.register_template_file("routing_template", "./serverside/routing.hbs")
}