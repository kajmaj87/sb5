print("hi")
function print_table(t, key, indent, visited)
    indent = indent or 2
    visited = visited or {}

    if visited[t] then
        print(string.rep("  ", indent) .. "*cycle detected* " .. visited[t])
        return
    end

    visited[t] = key or "root"

    local keys = {}

    for k in pairs(t) do
        table.insert(keys, k)
    end

    table.sort(keys)

    for _, k in ipairs(keys) do
        local v = t[k]
        local formatting = string.rep("  ", indent) .. k
        if type(v) == "table" then
            print(formatting .. ":")
            print_table(v, k, indent + 1, visited)
        else
            print(formatting .. ": " .. tostring(v))
        end
    end
end

function print_globals()
    print_table(_G)
end
wpm = require "scripts/wpm"

print_globals()

print_table(wpm)
