function getResult(input: string): number {
  let sum = 0
  for (const line of input.split('\n')) {
    const lists = line.split(':')[1];
    if (lists === undefined) {
      continue
    }
    const [winning, personnal] = lists.split('|')
    if (winning === undefined || personnal === undefined) {
      continue
    }
    const winningList = Array.from(new Set(winning.trim().split(' ').map(x => parseInt(x)).filter(n => !Number.isNaN(n))));
    const personnalList = Array.from(new Set(personnal.trim().split(' ').map(x => parseInt(x)).filter(n => !Number.isNaN(n))));

    const intersection = personnalList.filter(x => winningList.includes(x));
    if (intersection.length === 0) {
      continue
    }
    const points = 2**(intersection.length - 1);
    sum += points
  }
  return sum
}

const filename = Bun.argv[2];
if (!filename) {
  process.exit(1);
}
const input = await Bun.file(filename).text();
const result = getResult(input);
console.log(result);
