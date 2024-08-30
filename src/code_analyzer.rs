use tree_sitter::{Language, Parser, Query, QueryCursor};

fn get_language(file_name: &str) -> Language {
    if file_name.ends_with(".tsx") {
        tree_sitter_typescript::language_tsx()
    } else {
        tree_sitter_typescript::language_typescript()
    }
}

pub fn extract_signatures(contents: &[(String, String)]) -> Vec<(String, Vec<String>)> {
    let mut parser = Parser::new();

    let query_string = r#"
        (function_declaration
          name: (identifier) @func_name
        ) @function

        (variable_declarator
          name: (identifier) @var_name
          value: (arrow_function)
        ) @component

        (export_statement
          declaration: [
            (function_declaration
              name: (identifier) @export_func_name
            )
            (variable_declaration
              (variable_declarator
                name: (identifier) @export_var_name
              )
            )
          ]
        ) @export

        (lexical_declaration
          (variable_declarator
            name: (identifier) @const_name
            value: (arrow_function)
          )
        ) @const_component
    "#;

    contents
        .iter()
        .map(|(file, content)| {
            let language = get_language(file);
            parser
                .set_language(&language)
                .expect("Error loading TypeScript/TSX grammar");

            let query = Query::new(&language, query_string).expect("Error creating query");

            let tree = parser.parse(content, None).expect("Error parsing file");
            let mut cursor = QueryCursor::new();
            let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

            let mut signatures: Vec<String> = Vec::new();
            for m in matches {
                let capture_name = query.capture_names()[m.pattern_index];
                match capture_name {
                    "func_name" | "export_func_name" => {
                        if let Some(node) = m.nodes_for_capture_index(0).next() {
                            let name = &content[node.start_byte()..node.end_byte()];
                            signatures.push(format!("function {}", name));
                        }
                    }
                    "var_name" | "export_var_name" | "const_name" => {
                        if let Some(node) = m.nodes_for_capture_index(0).next() {
                            let name = &content[node.start_byte()..node.end_byte()];
                            signatures.push(format!("const {} = () => {{}}", name));
                        }
                    }
                    _ => {}
                }
            }

            (file.clone(), signatures)
        })
        .collect()
}
