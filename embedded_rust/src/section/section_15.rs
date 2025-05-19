//macro
//macroは関数呼び出しと異なり、その場で展開されて別のRustコードが埋め込まれる
//例えば、『println!("Hello world")』は次のようなコードに展開される
// ::std::io::_print(::core::fmt::Arguments::new_v1(
//   &["Hello, world!\n"],
//   &match () {
//     () => [],
//   },
// ));