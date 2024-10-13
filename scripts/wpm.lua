local wpm = 0
local times = {}
local N = 5 -- Define the window size for the moving average
local sum_time = 0
local m = {}

function m.handle_event(event)
    local time = event:time()
    if time > 0 then
        -- Add the new event time to the list
        table.insert(times, time)
        sum_time = sum_time + time

        -- Subtract the oldest time from the sum if the list exceeds N elements
        if #times > N then
            sum_time = sum_time - times[#times - N]
        end

        -- Calculate the average time per event in seconds
        local num_elements = math.min(#times, N)
        local average_time_per_event = sum_time / num_elements / 1000  -- Convert from ms to seconds

        -- Calculate WPM (words per minute)
        -- Assuming 1 word per 5 keystrokes as a rough estimate
        local keystrokes_per_word = 5
        local words_per_second = 1 / (average_time_per_event * keystrokes_per_word)
        wpm = words_per_second * 60  -- Convert to words per minute
    end
end

-- function m.result()
--     return wpm
-- end

return m