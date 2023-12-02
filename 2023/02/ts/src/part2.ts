function compute(input: string): number {
  return input
    .split('\n')
    .map((line: string) => {
      const croppedLine = line.replace(/^Game \d+: /, '')
      const sets = croppedLine.split(';')
      let maxRed = 0
      let maxGreen = 0
      let maxBlue = 0
      for (const set of sets) {
        const cubesCounts = set.split(',')
        for (const cubesCount of cubesCounts) {
          const [countStr, color] = cubesCount.trim().split(' ')
          const count = parseInt(countStr)
          if (color === 'red' && count > maxRed) {
            maxRed = count
          }
          if (color === 'green' && count > maxGreen) {
            maxGreen = count
          }
          if (color === 'blue' && count > maxBlue) {
            maxBlue = count
          }
        }
      }
      const power = maxRed * maxGreen * maxBlue
      return power
    })
    .reduce((sum: number, power: number) => {
      return sum + power
    }, 0)
}

const input = await Bun.file(Bun.argv[2]).text()
const result = compute(input)
console.log(result)
