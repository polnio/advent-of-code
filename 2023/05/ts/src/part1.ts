type Blocks = Map<string, {destination: string; matrix: number[][]}>

function getLocation(source: string, id: number, blocks: Blocks): number {
  const block = blocks.get(source);
  if (block === undefined) {
    throw new Error("No such map");
  }
  const row = block.matrix.find(row => row[1] <= id && row[1] + row[2] - 1 >= id);
  const destinationId = row !== undefined ? id - row[1] + row[0] : id;
  if (block.destination === "location") {
    return destinationId
  }
  return getLocation(block.destination, destinationId, blocks);
}

function getResult(input: string): number {
  const blockStrs = input.split("\n\n");
  const seeds = blockStrs.shift()!.split(" ").map(Number).filter(seed => !Number.isNaN(seed));
  const blocks: Blocks = new Map(
    blockStrs.map((str) => {
      const [before, after] = str.split(":");
      const [source, destination] = before.split(" ")[0].split("-to-");
      const matrix = after.split("\n").filter((s) => s.length > 0).map((row) => row.split(" ").map(Number));
      return [source, {destination, matrix}] as const;
    })
  );
  const lowestLocation = seeds.map((id) => getLocation("seed", id, blocks)).reduce((a, b) => a < b ? a : b);
  return lowestLocation;
}

const input = await Bun.file(Bun.argv[2]).text();
const result = getResult(input);
console.log(result);
