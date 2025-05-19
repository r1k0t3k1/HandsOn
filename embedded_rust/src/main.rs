mod section;

fn main() {
    println!("section1---------");
    section::section_1::hello_world();
    section::section_1::print_array();
    println!("-----------------");

    println!("section2---------");
    section::section_2::compile_error();
    println!("-----------------");

    println!("section3---------");
    section::section_3::imutable_valiable();
    section::section_3::mutable_valiable();
    section::section_3::constant_valiable();
    section::section_3::static_valiable();
    println!("-----------------");

    println!("section4---------");
    section::section_4::interger();
    section::section_4::float();
    section::section_4::bool();
    section::section_4::array();
    section::section_4::slice();
    section::section_4::mutable_slice();
    section::section_4::tuple();
    section::section_4::string();
    section::section_4::string_to_bytearray();
    println!("-----------------");

    println!("section5---------");
    section::section_5::ifelse();
    section::section_5::ifelse_is_expression();
    section::section_5::rust_loop();
    section::section_5::rust_while();
    section::section_5::rust_for();
    println!("-----------------");

    println!("section6---------");
    section::section_6::function();
    println!("-----------------");

    
    println!("section7---------");
    section::section_7::comment();
    section::section_7::documentation(32);
    println!("-----------------");

    println!("section8---------");
    section::section_8::struct_practice();
    section::section_8::implementation();
    section::section_8::related_function();
    println!("-----------------");

    println!("section9---------");
    section::section_9::print_type(section::section_9::Type::Int);
    section::section_9::print_type_bind(section::section_9::DefType::Float(64.5));
    println!("-----------------");

    println!("section10---------");
    section::section_10::use_option();
    section::section_10::use_result();
    if let Err(e) = section::section_10::error_raise() {
        println!("{:?}",e)
    }
    println!("-----------------");

    println!("section11---------");
    section::section_11::ownership();
    section::section_11::copy();
    section::section_11::_move();
    println!("-----------------");

    println!("section12---------");
    section::section_12::generics();
    println!("-----------------");
}
