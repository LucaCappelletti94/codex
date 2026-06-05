use codex_tools::FreeformTool;
use codex_tools::FreeformToolFormat;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use std::collections::BTreeMap;

const APPLY_PATCH_LARK_GRAMMAR: &str = include_str!("apply_patch.lark");

const APPLY_PATCH_FUNCTION_PATCH_DESCRIPTION: &str = r#"The full patch to apply, as a single string in the apply_patch envelope format.

Wrap the changes between `*** Begin Patch` and `*** End Patch`. Inside, use one section per file:
`*** Add File: <path>` followed by every new line prefixed with `+`.
`*** Update File: <path>` (optionally followed by `*** Move to: <new path>`), then `@@` context headers and hunk lines prefixed with `+` (added), `-` (removed), or a space (context).
`*** Delete File: <path>`.

Example:
*** Begin Patch
*** Add File: hello.txt
+Hello world
*** End Patch"#;

/// Returns a custom tool that can be used to edit files. Well-suited for GPT-5 models
/// https://platform.openai.com/docs/guides/function-calling#custom-tools
pub fn create_apply_patch_freeform_tool(include_environment_id: bool) -> ToolSpec {
    let definition = if include_environment_id {
        APPLY_PATCH_LARK_GRAMMAR.replace(
            "start: begin_patch hunk+ end_patch",
            "start: begin_patch environment_id? hunk+ end_patch\nenvironment_id: \"*** Environment ID: \" filename LF",
        )
    } else {
        APPLY_PATCH_LARK_GRAMMAR.to_string()
    };
    ToolSpec::Freeform(FreeformTool {
        name: "apply_patch".to_string(),
        description: "Use the `apply_patch` tool to edit files. This is a FREEFORM tool, so do not wrap the patch in JSON.".to_string(),
        format: FreeformToolFormat {
            r#type: "grammar".to_string(),
            syntax: "lark".to_string(),
            definition,
        },
    })
}

/// Returns a JSON `function` form of the apply_patch tool for function-calling
/// models that cannot use custom (freeform) tools, such as open-weight models
/// served over an OpenAI-compatible Responses API. The patch text is passed as a
/// single `patch` string argument and applied through the same path as the
/// freeform tool.
pub fn create_apply_patch_function_tool(include_environment_id: bool) -> ToolSpec {
    let mut properties = BTreeMap::from([(
        "patch".to_string(),
        JsonSchema::string(Some(APPLY_PATCH_FUNCTION_PATCH_DESCRIPTION.to_string())),
    )]);
    if include_environment_id {
        properties.insert(
            "environment_id".to_string(),
            JsonSchema::string(Some(
                "Identifier of the environment the patch targets.".to_string(),
            )),
        );
    }

    ToolSpec::Function(ResponsesApiTool {
        name: "apply_patch".to_string(),
        description:
            "Use the `apply_patch` tool to edit files by passing the patch as the `patch` argument."
                .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["patch".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

#[cfg(test)]
#[path = "apply_patch_spec_tests.rs"]
mod tests;
