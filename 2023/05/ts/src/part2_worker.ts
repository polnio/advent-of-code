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

declare const self: Worker;

self.addEventListener("message", (e: MessageEvent<{seeds: [number, number], blocks: Blocks}>) => {
  const {seeds, blocks} = e.data;
  let lowestLocation = Infinity;
  for (let j = seeds[0]; j <= seeds[0] + seeds[1]; j++) {
    // console.log(j);
    const location = getLocation("seed", j, blocks);
    if (location < lowestLocation) {
      lowestLocation = location;
    }
  }
  self.postMessage(lowestLocation);
});
