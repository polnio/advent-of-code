type Blocks = Map<string, {destination: string; matrix: number[][]}>

const workerUrl = new URL("./part2_worker.ts", import.meta.url);

async function getResult(input: string): Promise<number> {
  const blockStrs = input.split("\n\n");
  const seeds = blockStrs
    .shift()!
    .split(" ")
    .map(Number)
    .filter(seed => !Number.isNaN(seed))
  const blocks: Blocks = new Map(
    blockStrs.map((str) => {
      const [before, after] = str.split(":");
      const [source, destination] = before.split(" ")[0].split("-to-");
      const matrix = after.split("\n").filter((s) => s.length > 0).map((row) => row.split(" ").map(Number));
      return [source, {destination, matrix}] as const;
    })
  );
  let lowestLocation = Infinity;
  const promises = [];
  for (let i = 0; i < seeds.length; i+=2) {
    const worker = new Worker(workerUrl);

    worker.postMessage({
      seeds: [seeds[i], seeds[i+1]],
      blocks
    });

    promises.push(new Promise<void>((resolve) => {
      worker.addEventListener("message", (event) => {
        if (event.data < lowestLocation) {
          lowestLocation = event.data
        }
        worker.terminate();
        resolve();
      })
    }))
  }
  await Promise.all(promises);
  return lowestLocation;
}

const input = await Bun.file(Bun.argv[2]).text();
const result = await getResult(input);
console.log(result);
