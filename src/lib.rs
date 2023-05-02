use mlua::prelude::*;

fn hello(_: &Lua, name: String) -> LuaResult<()> {
    println!("hello, {}!", name);
    Ok(())
}

fn reverse_string(_: &Lua, input_string: String) -> LuaResult<String> {
    Ok(input_string.chars().rev().collect::<String>())
}

#[mlua::lua_module]
fn my_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello", lua.create_function(hello)?)?;
    exports.set("reverse_string", lua.create_function(reverse_string)?)?;
    Ok(exports)
}
