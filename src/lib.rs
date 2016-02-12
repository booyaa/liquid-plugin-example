extern crate liquid;
use liquid::Renderable;
use liquid::Block;
use liquid::Context;
use liquid::Template;
use liquid::LiquidOptions;
use liquid::lexer::Token;
use liquid::lexer::Element;
use liquid::Error;
use liquid::parser;

struct Shout<'a> {
    inner: Template<'a>,
}

impl<'a> Renderable for Shout<'a> {
    fn render(&self, context: &mut Context) -> Result<Option<String>, Error> {
        Ok(try!(self.inner.render(context)).map(|content| content.to_uppercase()))
    }
}

pub struct ShoutBlock;
impl Block for ShoutBlock {
    fn initialize<'a>(&'a self,
                      _tag_name: &str,
                      _arguments: &[Token],
                      tokens: Vec<Element>,
                      options: &'a LiquidOptions)
                      -> Result<Box<Renderable + 'a>, Error> {
        Ok(Box::new(Shout {
            inner: Template::new(try!(parser::parse(&tokens, options))),
        }) as Box<Renderable>)
    }
}

#[test]
fn it_works() {
    use std::default::Default;

    let mut options: LiquidOptions = Default::default();
    options.blocks.insert("shout".to_string(), Box::new(ShoutBlock) as Box<Block + 'static>);
    let template = liquid::parse("{% shout %}Liquid!{% endshout %}", &mut options).unwrap();
    let mut data = Context::new();
    let output = template.render(&mut data);
    assert_eq!(output.unwrap(), Some("LIQUID!".to_string()));
}
