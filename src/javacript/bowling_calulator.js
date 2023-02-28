function calculateTotalScore(arr) {
  let totalscore = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i][0] === 10 && arr[i + 1][0] === 10) {
      totalscore = 10 + 10 + arr[i + 2][0];
    } else if (arr[i][0] === 10) {
      totalscore =
        10 +
        arr[i + 1][0] +
        (arr[i + 1][0] === 10 ? arr[i + 2][0] : arr[i + 1][1]);
    } else if (arr[i][0] + arr[i][1] === 10) {
      totalscore = 10 + arr[i + 1][0];
    } else {
      totalscore += arr[i][0] + arr[i][1];
    }
  }
  return totalscore;
}
