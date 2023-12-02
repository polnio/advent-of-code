function compute(input: string): number {
  return input
    .split('\n')
    .filter((line: string) => {
      const croppedLine = line.replace(/^Game \d+: /, '')
      const sets = croppedLine.split(';')
      for (const set of sets) {
        const cubesCounts = set.split(',')
        for (const cubesCount of cubesCounts) {
          const [countStr, color] = cubesCount.trim().split(' ')
          const count = parseInt(countStr)
          if (color === 'red' && count > 12) {
            return false
          }
          if (color === 'green' && count > 13) {
            return false
          }
          if (color === 'blue' && count > 14) {
            return false
          }
        }
      }
      return true
    })
    .reduce((sum: number, line: string) => {
      const idStr = /Game (\d+)/.exec(line)?.[1]
      if (!idStr) {
        return sum
      }
      const id = parseInt(idStr)
      return sum + id
    }, 0)
}

const input = await Bun.file(Bun.argv[2]).text()
const result = compute(input)
console.log(result)
