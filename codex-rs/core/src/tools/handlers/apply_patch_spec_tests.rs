use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_apply_patch_freeform_tool_matches_expected_spec() {
    assert_eq!(
        create_apply_patch_freeform_tool(/*include_environment_id*/ false),
        ToolSpec::Freeform(FreeformTool {
            name: "apply_patch".to_string(),
            description:
                "Use the `apply_patch` tool to edit files. This is a FREEFORM tool, so do not wrap the patch in JSON."
                    .to_string(),
            format: FreeformToolFormat {
                r#type: "grammar".to_string(),
                syntax: "lark".to_string(),
                definition: APPLY_PATCH_LARK_GRAMMAR.to_string(),
            },
        })
    );
}

#[test]
fn create_apply_patch_freeform_tool_includes_environment_id_when_requested() {
    let ToolSpec::Freeform(tool) =
        create_apply_patch_freeform_tool(/*include_environment_id*/ true)
    else {
        panic!("expected freeform tool");
    };

    assert!(tool.format.definition.contains("environment_id?"));
    assert!(
        tool.format
            .definition
            .contains("\"*** Environment ID: \" filename LF")
    );
}

#[test]
fn create_apply_patch_function_tool_exposes_patch_parameter() {
    let ToolSpec::Function(tool) =
        create_apply_patch_function_tool(/*include_environment_id*/ false)
    else {
        panic!("expected function tool");
    };

    assert_eq!(tool.name, "apply_patch");
    let properties = tool
        .parameters
        .properties
        .expect("function tool should have object properties");
    assert!(properties.contains_key("patch"));
    assert!(!properties.contains_key("environment_id"));
    assert_eq!(
        tool.parameters.required.as_deref(),
        Some(["patch".to_string()].as_slice())
    );
}

#[test]
fn create_apply_patch_function_tool_adds_environment_id_when_requested() {
    let ToolSpec::Function(tool) =
        create_apply_patch_function_tool(/*include_environment_id*/ true)
    else {
        panic!("expected function tool");
    };

    let properties = tool
        .parameters
        .properties
        .expect("function tool should have object properties");
    assert!(properties.contains_key("patch"));
    assert!(properties.contains_key("environment_id"));
}
