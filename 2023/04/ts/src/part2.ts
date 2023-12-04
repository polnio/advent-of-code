function getResult(input: string): number {
  const linesCounts = new Map<number, number>();
  const lines = input.split('\n')
  for (let i = 0; i < lines.length; i++) {
    const idStr = lines[i]?.split(':')[0]?.slice(5);
    const lists = lines[i]?.split(':')[1];
    if (idStr === undefined || lists === undefined) {
      continue
    }
    const id = parseInt(idStr);
    if (!linesCounts.has(id)) {
      linesCounts.set(id, 1);
    }
    const count = linesCounts.get(id)!;
    const [winning, personnal] = lists.split('|')
    if (winning === undefined || personnal === undefined) {
      continue
    }
    const winningList = Array.from(new Set(winning.trim().split(' ').map(x => parseInt(x)).filter(n => !Number.isNaN(n))));
    const personnalList = Array.from(new Set(personnal.trim().split(' ').map(x => parseInt(x)).filter(n => !Number.isNaN(n))));

    const intersection = personnalList.filter(x => winningList.includes(x));
    // console.log(id, intersection.length, count)
    for (let j = 0; j < intersection.length; j++) {
      linesCounts.set(id + j + 1, (linesCounts.get(id + j + 1) ?? 1) + count);
      /* const newLine = lines[i + j + 1];
      if (newLine === undefined) {
        continue
      } */
      // console.log(lines[i]?.split(':')[0], newLine.split(':')[0]);
      // lines.splice(i + 1, 0, newLine)
      // lines.push(newLine);
    }
  }
  return Array.from(linesCounts.values()).reduce((acc, count) => acc + count, 0)
}

const filename = Bun.argv[2];
if (!filename) {
  process.exit(1);
}
const input = await Bun.file(filename).text();
const result = getResult(input);
console.log(result);
