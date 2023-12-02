const STR_TO_NUM: Record<string, number> = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
  zero: 0,
  "1": 1,
  "2": 2,
  "3": 3,
  "4": 4,
  "5": 5,
  "6": 6,
  "7": 7,
  "8": 8,
  "9": 9,
  "0": 0,
}

function getSum(input: string): number {
  let sum = 0
  for (let line of input.split("\n")) {
    let firstDigit: number | undefined
    let lastDigit: number | undefined
    while (line.length > 0) {
      for (const key in STR_TO_NUM) {
        if (line.startsWith(key)) {
          const n = STR_TO_NUM[key]
          if (!firstDigit) {
            firstDigit = n
          } else {
            lastDigit = n
          }
          break
        }
      }
      line = line.slice(1)
    }
    if (firstDigit === undefined) {
      continue
    }
    lastDigit ??= firstDigit
    sum += firstDigit * 10 + lastDigit
  }
  return sum
}

const input = await Bun.file(Bun.argv[2]).text()
const sum = getSum(input)
console.log(sum)
