function checkSymbols(matrix: string[][], i: number, j: number): boolean {
  for (let k = 0; k < 9; k++) {
    const newI = i + Math.floor(k / 3) - 1;
    const newJ = j + (k % 3) - 1;
    if (matrix[newI]?.[newJ] === undefined) {
      continue;
    }
    const matrixValue = matrix[newI][newJ];
    const isNumber = !Number.isNaN(Number(matrixValue));
    if (newI === i && newJ <= j && isNumber) {
      continue;
    }
    if (newI === i && newJ > j && isNumber) {
      // console.log(i, j, newI, newJ);
      if (checkSymbols(matrix, newI, newJ)) {
        return true
      }
      continue;
    }
    if (matrixValue !== '.') {
      return true
    }
  }
  return false
}

function getResult(input: string): number {
  const matrix = input.split('\n').map((line) => line.split(''));
  const numbers = []
  for (let i = 0; i < matrix.length; i++) {
    for (let j = 0; j < matrix[i].length; j++) {
      if (!Number.isNaN(Number(matrix[i][j]))) {
        const hasSymbol = checkSymbols(matrix, i, j);
        let numberStr = '';
        while (!Number.isNaN(Number(matrix[i][j]))) {
          numberStr += matrix[i][j];
          j++;
        }
        if (hasSymbol) {
          numbers.push(Number(numberStr));
        }
      }
    }
  }
  return numbers.reduce((acc, number) => acc + number, 0);
}

const file = Bun.file(Bun.argv[2]);
const input = await file.text();
const result = getResult(input);
console.log(result);
