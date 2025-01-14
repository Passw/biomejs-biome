use crate::parser::{
    directive::DirectiveList, parse_description, parse_error::expected_name, parse_name,
    GraphqlParser,
};
use biome_graphql_syntax::{GraphqlSyntaxKind::*, T};
use biome_parser::{
    parse_lists::ParseNodeList, parsed_syntax::ParsedSyntax, prelude::ParsedSyntax::*, Parser,
};

#[inline]
pub(crate) fn parse_scalar_type_definition(p: &mut GraphqlParser) -> ParsedSyntax {
    let m = p.start();
    // description is optional
    parse_description(p).ok();

    p.bump(T![scalar]);

    parse_name(p).or_add_diagnostic(p, expected_name);
    DirectiveList.parse_list(p);

    Present(m.complete(p, GRAPHQL_SCALAR_TYPE_DEFINITION))
}
