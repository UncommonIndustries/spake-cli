use fancy_regex::Regex;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::{HashMap, VecDeque};
use std::fs;

use htmlstream;

use crate::file::{get_json, Key};

pub fn replace_raw_strings_in_file(file_path: &str, strings_file_path: &str) -> String {
    let file_data = fs::read_to_string(file_path).unwrap();

    let components = extract_components(&file_data);

    let mut new_src = file_data.clone().to_string();
    let mut component_map: HashMap<String, String> = HashMap::new();

    for component in components {
        let result = extract_returned_jsx_from_component(component);

        if let Some(return_expression) = result {
            // get just the JSX portion of the return expression
            let copy = return_expression.to_string().clone();
            let tree = build_parse_tree(copy);

            let mut new_component_tree: Vec<(htmlstream::Position, htmlstream::HTMLTag)> =
                Vec::new();

            for (position, mut tag) in tree {
                if tag.state == htmlstream::HTMLTagState::Text {
                    let cleaned_text = tag.html.trim();
                    if cleaned_text.starts_with("{") || cleaned_text.ends_with("}") {
                        new_component_tree.push((position, tag));
                        continue;
                    }
                    let new_key = generate_synthetic_key_name(file_path, cleaned_text);

                    // store the key and the old string in the file here
                    add_key_to_strings_file(new_key.clone(), tag.html.clone(), strings_file_path);

                    // replace the string with the key in the tag element itself.✅
                    tag.html = create_evaluated_jsx_string(new_key);
                }
                new_component_tree.push((position, tag));
            }

            // rebuild the return statement.
            let altered_jsx = &position_tag_iterator_to_string(new_component_tree);
            let rebuilt_return_statement = format!("{}{}{}", "return (\n", altered_jsx, "\n)");
            // rebuild the component from the return statement
            let new_component = component.replace(return_expression, &rebuilt_return_statement);
            component_map.insert(component.to_string(), new_component);
        }
    }

    for (k, v) in component_map.iter() {
        new_src = new_src.replace(k, v);
    }
    fs::write(file_path, new_src);
    // rebuild the file from the components.
    return "new_src".to_string();
}

fn add_key_to_strings_file(new_key: String, text_data: String, strings_file_path: &str) {
    let keys = get_json(strings_file_path.to_string());
    let mut keys = match keys {
        Ok(k) => k,
        Err(_) => {
            println!("There was anerror reading in the keys file");
            return;
        }
    };
    if keys.contains_key(&new_key) {
        println!("This key is already in the thing.")
        // return an error and exit
    }
    let new_key_object = Key {
        string: text_data,
        example_keys: None,
        translate: None,
    };
    keys.insert(new_key, new_key_object);
    let json = match serde_json::to_string_pretty(&keys) {
        Ok(json) => json,
        Err(error) => {
            println!("Error converting json to string: {}", error);
            return;
        }
    };
    fs::write(strings_file_path, json);
}

fn position_tag_iterator_to_string(
    tree: Vec<(htmlstream::Position, htmlstream::HTMLTag)>,
) -> String {
    // TODO figure out how to replace indents automatically.

    let reassembled_return_expression: String = tree
        .iter()
        .map(|pair| pair.1.html.clone())
        .collect::<Vec<String>>()
        .join("\n");

    return reassembled_return_expression;
}

// todo fix this stupid method
fn concat_dom_tags(tags: Vec<String>) -> String {
    let mut result = String::new();
    let mut indent = 0;
    let mut tag_stack = VecDeque::new();

    for tag in tags {
        if tag.starts_with("<") {
            // Opening tag
            result.push_str(&format!("{: >1$}", tag, indent * 2));
            result.push_str("\n");
            indent += 1;
            tag_stack.push_front(tag);
        } else {
            // Closing tag
            indent -= 1;
            result.push_str(&format!("{: >1$}", tag, indent * 2));
            result.push_str("\n");
            tag_stack.pop_front();
        }
    }
    result
}

// given a full jsx file, extract any functional, class, or stateless components.
fn extract_components(src: &str) -> Vec<&str> {
    // find the component block
    // TODO expand this pattern to identify class and const components.
    let function_component_regex =
        Regex::new(r"function\s+([a-zA-Z0-9_]+)\s*\((props)?\)\s*{").unwrap();
    // return the component block
    let mut functional_components: Vec<&str> = Vec::new();
    for result in function_component_regex.find_iter(src) {
        match result {
            Ok(match_) => {
                let closure_start = match_.end();
                let closure_end = find_closure_end(src, closure_start - 1);

                if let Some(closure_end) = closure_end {
                    let function_data = &src[match_.start()..closure_end + 1];
                    functional_components.append(&mut vec![function_data]);
                }
                println!("Found functional component, {:?}", match_)
            }
            Err(_) => panic!("Error with something"),
        }
    }
    return functional_components;
}

fn build_parse_tree(src: String) -> VecDeque<(htmlstream::Position, htmlstream::HTMLTag)> {
    let re = Regex::new(r"\s\s+");
    let file_data = re.unwrap().replace_all(&src, " ").to_string();

    let parser: htmlstream::HTMLTagIterator = htmlstream::tag_iter(&file_data);

    let mut cleaned_data: VecDeque<(htmlstream::Position, htmlstream::HTMLTag)> = parser
        .filter(|(_, tag)| !(tag.state == htmlstream::HTMLTagState::Text && tag.html == " "))
        .map(|(p, t)| (p, t))
        .collect();
    cleaned_data.retain(|pair| !pair.1.html.starts_with("return"));
    cleaned_data.retain(|pair| !pair.1.html.ends_with(")"));
    return cleaned_data;
}

fn generate_synthetic_key_name(file_path: &str, component: &str) -> String {
    let random_data: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    let component_name = get_component_name(component);
    let file_name = file_path.split("/").last().unwrap();
    let split_filename: Vec<&str> = file_name.split(".").collect();
    let filename = split_filename[0];

    return format!("{}_{}_{}_key", filename, component_name, random_data);
}
fn get_component_name(component: &str) -> &str {
    // TODO this should be a real function
    return "componentNameSlug";
}

fn find_closure_end(src: &str, closure_start: usize) -> Option<usize> {
    let mut stack = VecDeque::new();

    for (i, c) in src[closure_start..].char_indices() {
        match c {
            '{' => stack.push_back(c),
            '}' => {
                stack.pop_back();
                if stack.is_empty() {
                    return Some(closure_start + i);
                }
            }
            _ => {}
        }
    }
    None
}

// given a function, find the jsx within.
fn extract_returned_jsx_from_component(component: &str) -> Option<&str> {
    let return_jsx_component_regex_string =
        r"return\s*\(\s*<(\w+)?(.+?)?>\s*([\s\S]*?)\s*<\/\1?>\s*\)";
    let return_jsx_component_regex = Regex::new(return_jsx_component_regex_string).unwrap();
    let result = return_jsx_component_regex.captures(component);
    match result {
        Ok(capture) => {
            if let Some(capture) = capture {
                return Some(capture.get(0).unwrap().as_str());
            }
        }
        Err(_) => {
            panic!("error)")
        }
    }
    None
}

fn create_evaluated_jsx_string(text: String) -> String {
    return format!("{{ {} }}", text);
}

#[cfg(test)]
mod run_tests {

    use super::*;

    const FULL_FILE_FIXTURE: &str = r#"import { withAuthenticationRequired } from "@auth0/auth0-react";

    import { Outlet } from "react-router-dom";
    import { Box, Container } from "@mui/material";
    
    import SideNav from "./components/SideNav";
    
    function LoggedinViewBase() {
      return (
        <Box display="flex">
          <SideNav />
          <Container>
            <Outlet />
          </Container>
        </Box>
      );
    }
    
    const LoggedinView = withAuthenticationRequired(LoggedinViewBase);
    
    export default LoggedinView;
    "#;

    const SINGLE_COMPONENT_TEST_FIXTURE: &str = r#"function LoggedinViewBase() {
        return (
          <Box display="flex">
            <SideNav />
            <Container>
              <Outlet />
            </Container>
          </Box>
        );
      }"#;
    const BASIC_RETURN_EXPRESSION_WITH_JSX_FIXTURE: &str = r#"return (
        <Box display="flex">
          <SideNav />
          <Container>
            <Outlet />
          </Container>
        </Box>
      )"#;

    #[test]
    fn test_function_extraction() {
        let results = extract_components(FULL_FILE_FIXTURE);
        assert!(
            !results.is_empty(),
            "correctly identifies function component",
        );
        let mut returned_string = results[0].to_string();

        returned_string.retain(|c| !c.is_whitespace());
        let mut comparison = SINGLE_COMPONENT_TEST_FIXTURE.to_string();
        comparison.retain(|c| !c.is_whitespace());
        assert_eq!(returned_string, comparison);
    }

    #[test]
    fn test_jsx_extraction() {
        let result = extract_returned_jsx_from_component(SINGLE_COMPONENT_TEST_FIXTURE);
        assert!(
            !result.is_none(),
            "correctly identifies jsx within function component",
        );
        // silly hack to remove whitespace from the string
        let mut returned_string = result.unwrap().to_string();
        returned_string.retain(|c| !c.is_whitespace());
        let mut comparison = BASIC_RETURN_EXPRESSION_WITH_JSX_FIXTURE.to_string();
        comparison.retain(|c| !c.is_whitespace());

        assert_eq!(returned_string, comparison);
    }
    #[test]
    fn test_harder_example() {
        let src = r#"function TestView() {
            const [content, setContent] = useState("");
            useEffect(() => {
              fetch(GettingStarted)
                .then((response) => response.text())
                .then((text) => {
                  setContent(text);
                });
            }, []);
          
            return (
              <SyntaxHighlightedMarkdownView remarkPlugins={[remarkGfm]}>
                {content}
              </SyntaxHighlightedMarkdownView>
            );
          }"#;
        let closure_start = src.find("{").unwrap();
        let closure_end = find_closure_end(src, closure_start).unwrap();
        assert_eq!(closure_end, src.len() - 1)
    }

    #[test]
    fn test_closure_end() {
        let src = "function foo() { return { bar: 'baz' } }";
        let closure_start = src.find('{').unwrap();
        let closure_end = find_closure_end(src, closure_start).unwrap();
        assert_eq!(closure_end, src.len() - 1);
        let inner_closures_start = 24;
        let inner_closures_end = find_closure_end(src, inner_closures_start).unwrap();
        assert_eq!(inner_closures_end, 37);
    }
    #[test]
    fn test_bad_closure_end() {
        let src = "function foo() { return { bar: 'baz' }";
        let closure_start = src.find('{').unwrap();
        let closure_end = find_closure_end(src, closure_start);
        assert_eq!(closure_end, None);
    }

    #[test]
    fn test_jsx_evaluation_formatter() {
        let test_key = "testKey".to_string();
        let result = create_evaluated_jsx_string(test_key);
        assert_eq!(result, "{ testKey }")
    }

    #[test]
    fn test_converting_position_tag_iterators_back_to_strings() {
        let expected_result = "<Grid>
<Typography>
Hello world
</Typography>
</Grid>";
        let arbirtary_input = "<Grid><Typography>Hello world</Typography></Grid>";
        let mut vec_for_testing: Vec<(htmlstream::Position, htmlstream::HTMLTag)> = Vec::new();
        vec_for_testing.extend(htmlstream::tag_iter(arbirtary_input));

        let result = position_tag_iterator_to_string(vec_for_testing);

        assert_eq!(expected_result, result)
    } // ok slightly silly but this tests and works
}
