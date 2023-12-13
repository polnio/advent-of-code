namespace AoC

open System.IO

module Part1 =

    type Combination =
        | Five = 6
        | Four = 5
        | FullHouse = 4
        | Three = 3
        | TwoPair = 2
        | OnePair = 1
        | HighCard = 0

    let cardSymbols =
        [| 'A'; 'K'; 'Q'; 'J'; 'T'; '9'; '8'; '7'; '6'; '5'; '4'; '3'; '2' |]

    let readLines (filename: string) =
        seq {
            use sr = new StreamReader(filename)

            while not sr.EndOfStream do
                yield sr.ReadLine()
        }

    let getCombination (cards: string) =
        let (symbol, count) =
            cards
            |> Seq.countBy id
            |> Seq.maxBy (fun (symbol, count) -> (count, cardSymbols |> Seq.findIndex ((=) symbol) |> (*) -1))

        match count with
        | 5 -> Combination.Five
        | 4 -> Combination.Four
        | 3 ->
            if cards |> Seq.countBy id |> Seq.exists (fun (_, count) -> count = 2) then
                Combination.FullHouse
            else
                Combination.Three
        | 2 ->
            if
                cards
                |> Seq.countBy id
                |> Seq.exists (fun (card, count) -> card <> symbol && count = 2)
            then
                Combination.TwoPair
            else
                Combination.OnePair
        | _ -> Combination.HighCard


    let rec compareCardSymbols (cards1: string) (cards2: string) =
        let index1 = cardSymbols |> Seq.findIndex ((=) cards1[0])
        let index2 = cardSymbols |> Seq.findIndex ((=) cards2[0])

        if index1 > index2 then
            -1
        else if index1 < index2 then
            1
        else
            compareCardSymbols (cards1.Substring(1)) (cards2.Substring(1))


    let get_result (filename: string) =
        readLines filename
        |> Seq.map (fun s ->
            let (cards, betStr) =
                match s.Split ' ' with
                | [| cards; bet |] -> (cards, bet)
                | _ -> failwith "bad input"

            let bet = int betStr
            let combination = getCombination cards
            (cards, combination, bet))
        |> Seq.sortWith (fun (cards1, combination1, _) (cards2, combination2, _) ->
            if combination1 > combination2 then 1
            else if combination1 < combination2 then -1
            else compareCardSymbols cards1 cards2)
        |> Seq.mapi (fun i (_, _, bet) -> bet * (i + 1))
        |> Seq.sum
