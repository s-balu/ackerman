#include <stdio.h>
#include <stdlib.h>

int ackermann(int m, int n) {
  int ans;

  if (m == 0)
    ans = n + 1;

  else if (n == 0)
    ans = ackermann(m - 1, 1);

  else
    ans = ackermann(m - 1, ackermann(m, n - 1));

  return ans;
}

int main(int argc, char **argv) {
  int m = atoi(argv[1]);
  int n = atoi(argv[2]);

  int result = ackermann(m, n);
  printf("Ackermann(%d, %d) = %d\n", m, n, result);

  return 0;
}
