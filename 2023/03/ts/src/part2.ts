function getLeftNumber(matrix: string[][], i: number, j: number): string | undefined {
  const n = Number(matrix[i]?.[j])
  if (Number.isNaN(n)) {
    return undefined
  }
  const left = getLeftNumber(matrix, i, j - 1);
  return `${left ?? ''}${n}`;
}

function getRightNumber(matrix: string[][], i: number, j: number): string | undefined {
  const n = Number(matrix[i]?.[j])
  if (Number.isNaN(n)) {
    return undefined
  }
  const right = getRightNumber(matrix, i, j + 1);
  return `${n}${right ?? ''}`
}

function getNumber(matrix: string[][], i: number, j: number): string | undefined {
  const n = Number(matrix[i]?.[j])
  if (Number.isNaN(n)) {
    return undefined
  }
  const left = getLeftNumber(matrix, i, j - 1);
  const right = getRightNumber(matrix, i, j + 1);
  return `${left ?? ''}${n}${right ?? ''}`
}

function getResult(input: string): number {
  const matrix = input.split('\n').map((line) => line.split(''));
  const ratios = []
  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix[i].length; j++) {
      if (matrix[i][j] === '*') {
        const adjacentNumbers = [];
        for (let k = 0; k < 9; k++) {
          const newI = i + Math.floor(k / 3) - 1;
          const newJ = j + (k % 3) - 1;
          if (matrix[newI]?.[newJ] === undefined) {
            continue;
          }
          const number = getNumber(matrix, newI, newJ);
          // console.log(i, j, newI, newJ, number);
          if (number !== undefined) {
            adjacentNumbers.push(Number(number));
            while (!Number.isNaN(Number(matrix[i + Math.floor(k / 3) - 1]?.[j + (k % 3) - 1])) && i + Math.floor(k / 3) - 1 === newI) {
              k++;
            }
            k--;
          }
        }
        if (adjacentNumbers.length === 2) {
          ratios.push(adjacentNumbers[0] * adjacentNumbers[1]);
        }
      }
    }
  }
  return ratios.reduce((acc, number) => acc + number, 0);
}

const file = Bun.file(Bun.argv[2]);
const input = await file.text();
const result = getResult(input);
console.log(result);
