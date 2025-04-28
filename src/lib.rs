#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(clippy::absolute_paths)]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::allow_attributes)]
#![deny(clippy::allow_attributes_without_reason)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "Kelvin Embedded style guide: prefer ordering for logical reasons over alphabetical."
)]
#![allow(
    clippy::arithmetic_side_effects,
    reason = "Kelvin Embedded style guide: Arithmetic side effects commonly used in embedded systems programming."
)]
#![allow(
    clippy::as_conversions,
    reason = "clippy::as_conversions explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::as_pointer_underscore)]
#![deny(clippy::as_underscore)]
#![deny(clippy::assertions_on_constants)]
#![deny(clippy::big_endian_bytes)]
#![deny(clippy::byte_char_slices)]
#![deny(clippy::cfg_not_test)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::create_dir)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::default_numeric_fallback)]
#![deny(clippy::default_union_representation)]
#![deny(clippy::deref_by_slicing)]
#![deny(clippy::disallowed_script_idents)]
#![deny(clippy::doc_include_without_cfg)]
#![allow(
    clippy::duplicated_attributes,
    reason = "https://github.com/rust-lang/rust-clippy/issues/13500"
)]
#![deny(clippy::else_if_without_else)]
#![deny(clippy::empty_drop)]
#![deny(clippy::empty_enum_variants_with_brackets)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::error_impl_error)]
#![allow(
    clippy::exhaustive_enums,
    reason = "Kelvin Embedded style guide: allowing public enumerants is consistent with a functional style use of enumerations as pure data."
)]
#![allow(
    clippy::exhaustive_structs,
    reason = "Kelvin Embedded style guide: allowing public field access is consistent with a functional style use of data structures as pure data."
)]
#![deny(clippy::exit)]
#![deny(clippy::expect_used)]
#![deny(clippy::field_scoped_visibility_modifiers)]
#![deny(clippy::filetype_is_file)]
#![deny(clippy::filter_map_bool_then)]
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::fn_to_numeric_cast_any)]
#![deny(clippy::format_push_string)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::host_endian_bytes)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::impl_trait_in_params)]
#![allow(
    clippy::implicit_return,
    reason = "Kelvin Embedded Style Guide: Implicit returns are an idiomatic approach that improves code readability."
)]
#![allow(clippy::indexing_slicing, reason = "Necessary to allow use of defmt.")]
#![deny(clippy::infinite_loop)]
#![deny(clippy::inline_asm_x86_att_syntax)]
#![allow(
    clippy::inline_asm_x86_intel_syntax,
    reason = "clippy::inline_asm_x86_intel_syntax explanation lost in history. TODO remove allow and find reason."
)]
#![allow(
    clippy::integer_division,
    reason = "Kelvin Embedded Style Guide: Integer division is a normally used operation in embedded systems programming."
)]
#![allow(
    clippy::integer_division_remainder_used,
    reason = "Kelvin Embedded Style Guide: Integer division is a normally used operation in embedded systems programming."
)]
#![deny(clippy::items_after_test_module)]
#![deny(clippy::iter_over_hash_type)]
#![deny(clippy::large_include_file)]
#![deny(clippy::legacy_numeric_constants)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::let_underscore_untyped)]
#![allow(
    clippy::little_endian_bytes,
    reason = "Little Endian is both our target and host endianness. Preference is specifying explixit over local."
)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::manual_is_finite)]
#![deny(clippy::manual_is_infinite)]
#![deny(clippy::manual_is_power_of_two)]
#![deny(clippy::manual_next_back)]
#![deny(clippy::manual_ok_or)]
#![deny(clippy::manual_pattern_char_comparison)]
#![deny(clippy::manual_rotate)]
#![deny(clippy::manual_while_let_some)]
#![deny(clippy::map_all_any_identity)]
#![deny(clippy::map_err_ignore)]
#![deny(clippy::map_with_unused_argument_over_ranges)]
#![deny(clippy::mem_forget)]
#![allow(
    clippy::min_ident_chars,
    reason = "Single characters are useful in small namespaces and should not be mechanically prohibited."
)]
#![deny(clippy::missing_assert_message)]
#![allow(
    clippy::missing_asserts_for_indexing,
    reason = "Asserts panic when false, which is prohibited in the embedded system."
)]
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "clippy::missing_docs_in_private_items explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::missing_enforced_import_renames)]
#![allow(
    clippy::missing_inline_in_public_items,
    reason = "clippy::missing_inline_in_public_items explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::missing_trait_methods)]
#![deny(clippy::mixed_attributes_style)]
#![deny(clippy::mixed_read_write_in_expression)]
#![deny(clippy::mod_module_files)]
#![deny(clippy::module_name_repetitions)]
#![allow(
    clippy::modulo_arithmetic,
    reason = "clippy::modulo_arithmetic explanation lost in history. TODO remove allow and find reason."
)]
#![allow(
    clippy::multiple_crate_versions,
    reason = "clippy::multiple_crate_versions explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::multiple_inherent_impl)]
#![deny(clippy::multiple_unsafe_ops_per_block)]
#![deny(clippy::mutex_atomic)]
#![deny(clippy::needless_as_bytes)]
#![deny(clippy::needless_borrows_for_generic_args)]
#![deny(clippy::needless_else)]
#![deny(clippy::needless_pub_self)]
#![deny(clippy::needless_raw_strings)]
#![deny(clippy::needless_return_with_question_mark)]
#![deny(clippy::non_ascii_literal)]
#![deny(clippy::non_minimal_cfg)]
#![deny(clippy::non_zero_suggestions)]
#![deny(clippy::panic)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::partial_pub_fields)]
#![deny(clippy::pathbuf_init_then_push)]
#![deny(clippy::pattern_type_mismatch)]
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
#![deny(clippy::pub_use)]
#![allow(
    clippy::pub_with_shorthand,
    reason = "Denying the reciprocal pub_without_shorthand."
)]
#![deny(clippy::pub_without_shorthand)]
#![allow(
    clippy::question_mark_used,
    reason = "Allowed by Kelvin Style Guide. This is idiomatic Rust."
)]
#![deny(clippy::rc_buffer)]
#![deny(clippy::rc_mutex)]
#![deny(clippy::redundant_feature_names)]
#![deny(clippy::redundant_type_annotations)]
#![deny(clippy::ref_patterns)]
#![deny(clippy::renamed_function_params)]
#![deny(clippy::rest_pat_in_fully_bound_structs)]
#![allow(
    clippy::same_name_method,
    reason = "TODO reconsider. Due to bitflags! macro"
)]
#![deny(clippy::self_named_module_files)]
#![deny(clippy::semicolon_outside_block)]
#![allow(
    clippy::separated_literal_suffix,
    reason = "clippy::separated_literal_suffix explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_unrelated)]
#![allow(
    clippy::single_call_fn,
    reason = "Allowed by Kelvin style guide. This is best practice when creating well refactored code."
)]
#![allow(
    clippy::single_char_lifetime_names,
    reason = "Allowed by Kelvin style guide. Single character lifetime names are commonly used in idiomatic Rust."
)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::str_to_string)]
#![deny(clippy::string_add)]
#![deny(clippy::string_lit_chars_any)]
#![deny(clippy::string_slice)]
#![deny(clippy::string_to_string)]
#![deny(clippy::suspicious_xor_used_as_pow)]
#![deny(clippy::tests_outside_test_module)]
#![deny(clippy::to_string_trait_impl)]
#![deny(clippy::todo)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unnecessary_fallible_conversions)]
#![deny(clippy::unnecessary_map_or)]
#![deny(clippy::unnecessary_safety_comment)]
#![deny(clippy::unnecessary_safety_doc)]
#![deny(clippy::unnecessary_self_imports)]
#![deny(clippy::unneeded_field_pattern)]
#![deny(clippy::unreachable)]
#![deny(clippy::unseparated_literal_suffix)]
#![deny(clippy::unused_enumerate_index)]
#![deny(clippy::unused_result_ok)]
#![deny(clippy::unused_trait_names)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::use_debug)]
#![deny(clippy::verbose_file_reads)]
#![deny(clippy::wildcard_enum_match_arm)]

/// Copyright ©2025 Kelvin Systems
mod moving_average;

#[expect(clippy::useless_attribute, reason = "Working around clippy bug")]
#[expect(clippy::pub_use, reason = "Exporting without exposing file structure")]
pub use moving_average::MovingAverage;
