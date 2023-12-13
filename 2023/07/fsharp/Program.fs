namespace AoC

open System

module Program =
    [<EntryPoint>]
    let main argv =
        let filename = argv.[0]
        let part = Environment.GetEnvironmentVariable "PART"

        if part = "2" then
            let result = Part2.get_result filename
            printfn "%A" result
        else
            let result = Part1.get_result filename
            printfn "%A" result

        0
