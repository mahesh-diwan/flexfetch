return {
    name = "hello_test",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        return { value = "Hello from Lua, " .. user .. "!", type = "scalar" }
    end
}
