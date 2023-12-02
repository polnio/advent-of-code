function getSum(input: string): number {
  let sum = 0
  for (const line of input.split("\n")) {
    let firstDigit: number | undefined
    let lastDigit: number | undefined
    for (const char of line) {
      const n = parseInt(char)
      if (Number.isNaN(n)) {
        continue
      }
      if (!firstDigit) {
        firstDigit = n
      } else {
        lastDigit = n
      }
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
